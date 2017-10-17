
### Register File Types

    tree: SLEEP merkel tree
        magic: 0x05025702
        entry-size: 40 bytes
        algorithm name: 'BLAKE2b' (len=7)
        entry: 32-byte BLAKE2b node hash + 8-byte Uint64BE length
    signatures: SLEEP array
        magic: 0x05025701
        entry-size: 65
        algorithm name: 'Ed25519' (len=7)
        entry: 64-byte Ed25519 signature
    bitfield: SLEEP array of bitfields
        magic: 0x05025700
        entry-size: 3328
        algorithm name: '' (len=0)
        entry: 1024 bytes data, 2048 bytes tree, 256 bitfield
    key: public key
        no header, raw bytes
    data
        no header, raw bytes, concatonated

### Register API

    SleepDirectoryRegister struct
        open(directory, prefix)
        create(directory, prefix, options)

    file handles for tree, sig, bitfield, data; key is write-once

    has(index) -> bool
    has_all() -> bool
    has_range(start, end) -> bool
    get(index) -> data (arbitrary length)
    append(entry) -> index written
    len() -> u64
    len_bytes() -> u64
    verify() -> bool (verifies merkel tree against data, and signatures)
    check() -> bool (quick consistency check of lengths, etc)
    writable() -> bool

    read_block(start, end) -> read api implementer

