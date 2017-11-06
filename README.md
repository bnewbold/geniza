
                      _          
      __ _  ___ _ __ (_)______ _ 
     / _` |/ _ \ '_ \| |_  / _` |
    | (_| |  __/ | | | |/ / (_| |
     \__, |\___|_| |_|_/___\__,_|
     |___/                       


this is a poor / partial / non-compliant dat implementation in rust.

it will eat-your-data!

### Status

[![Build Status](https://travis-ci.org/bnewbold/geniza.svg?branch=master)](https://travis-ci.org/bnewbold/geniza)

- [ ] SLEEP v2 files and registers
    - [x] read/write file headers
    - [x] read/write file chunks as raw bytes
    - [x] pread/pwrite file chunks without seeking
    - [x] read data entries by index
    - [x] append data entries
    - [x] verify entire register (signatures and merkel tree)
    - [ ] bitfields
- [ ] Drive metadata and files
    - [x] read full history ("log")
    - [x] read file tree ("ls")
    - [x] import file to register
    - [x] export file from register
    - [ ] import/export directories recursively
- [ ] Networking
    - [x] send/receive encrypted messages to a known host
    - [ ] receive entire register from a known host
    - [ ] share (upload) register to a known host
    - [ ] bitfields
    - [ ] lookup hosts in DHT swarm by discovery key
- [ ] Wrapper commands
    - [ ] clone
    - [ ] share
    - [x] log
    - [ ] status
    - [ ] add

### Differences from dat

A few simplifications were made compared to the regular dat client:

- Content data is always stored in the SLEEP directory (`content.data`),
  instead of just having the latest data in files in the working directory.
  This results in data duplication.
- Sparse registers aren't implemented: full history needs to be present for
  both metadata and content.
- Tracking of remote node state (bitfields) is minimized as much as possible.
- Almost everything is synchronous and single-threaded, so only a single remote
  node connection at a time is allowed.
- discovery (DHT, multicast-DNS, etc) is not implemented.

### Dependencies

Notable Rust Libraries:
- `rust-crypto` for hashing (BLAKE2b)
- `sodiumoxide` signing (Ed25519) and network stream encryption (XSalsa20)
- `integer-encoding` for simple "varints"
- `rust-protobuf` for protobuf messages

While (temporarily?) using Cargo's `[patch]` feature to patch sodiumoxide,
builds require Rust 1.21 (stable circa Oct 2017).

Requires the libsodium library installed system-wide to run, and header files
(`libsodium-dev`) for now, due to dynamic linking and the simple build
configuration. It should be possible to statically link a free-standing
executable, and to auto-build the libsodium C library during compilation if it
isn't found system-wide.

protobuf encode/decode methods are auto-generated ahead of time using
`rust-protobuf`, so if you're just compiling or installing `geniza` you don't
need any special tools. However, if you change or extend the `.proto` schema
files, you do, along with the `protoc` tool (`sudo apt install
protobuf-compiler`) and rust plugin `protoc-gen-rust` (`cargo install
protobuf`). The command to regenerate a single file is:

    protoc --rust_out . network_msgs.proto
