
First varint is a header. Lowest bit of header is "endsWithSeq".

When expanding, if "endsWithSeq", is set, add the current index to *every*
level of the array?

When adding, the current index is always added at every stage of the path.

In the index, include one entry for subdirectories; take the newest element of
the subdirectory as that entry (which might itself be a deeper nested
subdirectory entry)

There seem to be extra empty arrays everywhere? not sure what's up with that,
maybe I have an off-by-one.

The current path counts as an entry (eg, "/README" decomposes to "/" and
"README", two entries). I'm not sure why... so directories can have entries?

Entry indexes are effectively 1-based (0 is the pubkey entry); don't correct
for this.
