
### Binary Header 

Fixed 32-bytes

    -----------------------------------------------------------------
    |                             MAGIC                             |
    -----------------------------------------------------------------
    |  VERSION (0)  |      ENTRY SIZE (uint16BE)    | ALGO NAME LEN |
    -----------------------------------------------------------------
    |                                                               |
    |                                                               |
    |                                                               |
    |                 ALGORITHM NAME (lower-case string)            |
    |                                                               |
    |                                                               |
    -----------------------------------------------------------------


    MAGIC and ENTRY SIZE are both "big-endian", meaning that the "first" (aka
    "most-significant" byte is in the "first" memory location (aka, "lowest
    address").

    PREFIX LEN is max 24 (bytes)

    offset: 32 + entrySize * entryIndex
    count: (fileSize - 32) / entrySize


### SLEEP API

    SleepStorage trait
    SleepFile struct/impl

    get_magic() -> u32
    get_algorithm() -> &str
    read(index) -> &bytes[]
    write(index, &bytes[])
    length() -> u64

    First time around just do regular file seek() stuff
