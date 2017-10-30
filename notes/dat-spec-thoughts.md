
Edited and posted to: https://gist.github.com/bnewbold/c9cebbb9384236e3910fb9d4948cb054

I've been implementing a dat client in Rust: https://github.com/bnewbold/geniza

It's been fun! The "whitepaper"/spec has been very helpful. Below are a few thoughts/comments on the paper, documentation, and protocols.

## Informal Protocol Proposals

With my archival and inter-op hat on, I wish that the hyperdrive metadata register (specifically Node protobuf messages) included standard full-file hashes (eg, SHA1 or BLAKE2b of the entire file, with no length prefix). These could be optional, but could presumably be calculated when adding files to archives with little overhead. This could make auditing, verification, and interoperability between distributed networks easier. Storage and compute overhead would be non-zero.

It seems like the network protocol really should have a version field... in the initial pre-encryption Register message?

I'm curious if the protobuf coded stream format (CodedInputStream/CodedOutputStream) was considered as an alternative to the existing {varint-length, varint-header, message} scheme. My brief reading of the coded streams is that they add varint length and type encoding to a stream of messages, which is almost exactly what the existing protocol is, but has broad support in well-optimized cross-platform wrapper libraries. The protocol network message scheme might need to change to have a wrapper type that wraps every message. This would be a clear place to include the channel identifier as a separate varint... which would add at least one byte overhead to each message.

It might be nice to include an "added" timestamp to hyperdrive Node entries. The semantics would be the same as git commit messages: the timestamps are not verified or used in the protocol in any way, but are trusted in the same way the content itself is trusted (signed by creator) and is helpful for humans to understand what is going on. Commiter name (and email) could also be added, but that seems like a bigger can of worms than a simple UTC timestamp.

## Things I Learned

I initially explored dat and read the paper over the summer. While implementing more recently I learned (or remembered) a few things.

Appending to SLEEP registers ("append only logs") results in writes into middle of tree files (for root nodes). So, while the register as a whole is append-only, the files are not. This is obviously the case for sparse (or partial) synchronization, but is also the case for simple appends. From a disk I/O and caching standpoint, this seems suboptimal, in the context of being able to write dat tree files as fast as disk I/O allows. On reflection, it probably isn't that big of a dea.: clients can still operate on power-of-2 bulk chunks, and only need to backwards-write something like log(n) entries. My feelings on this are mostly speculative; "benchmarks needed".

hypercore relies on libsodium for most cryptography, which seems like a wise decision. The details were not specified in the whitepaper (they should be!), but were easy to figure out: "XOR streaming" mode with XSalsa20 keys/nonces (defaults with libsodium for this use case). However, the use of the "streaming xor" API in the network protocol is somewhat non-standard: it does not seem to be supported or mentioned in the underlying (and well audited) NaCL code. This isn't hugely problematic, but it could be a roadbump for implementors in non-node.js languages. The particular necessary function which is not always included in language wrappers (and is not mentioned in libsodium documentation) is `crypto_stream_salsa20_xor_ic()`, which allows specifying a block offset when encrypting a buffer.

My gut feeling is that protocol (network) messages should be prefixed with a fixed size uint32 instead of a varint... maybe i'm just old-fashioned though. As noted above, being compatible with coded/tagged streams of messages is probably best, and that scheme itself uses varints to encode length.

## A whole bunch of little notes

- merkle tree: what if there is only a single chunk/entry in a register tree? then a leaf acts as a root?
- unclear where protobuf schema actually lives... spread across multiple repos
- protocol extension stuff not in the whitepaper
- whitepaper should be versioned/tagged, or marked as "work in progress"
- encryption not really covered in whitepaper... seems to be libsodium's XSalsa20 XOR'd with data
- "ping" mechanism: sending a 1-byte message (0 length body) to keep channel alive

## Questions in the form of issues

* [Please clarify key names and new SLEEP file types](https://github.com/datproject/docs/issues/93)
* [Current chunking: content-aware? de-duped?](https://github.com/datproject/discussions/issues/77)
* [Please clarify hyperdrive metadata "children"/"trie" encoding](https://github.com/datproject/docs/issues/92)
* [Spec: how to delete files?](https://github.com/datproject/docs/issues/94)
