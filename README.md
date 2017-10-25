
                      _          
      __ _  ___ _ __ (_)______ _ 
     / _` |/ _ \ '_ \| |_  / _` |
    | (_| |  __/ | | | |/ / (_| |
     \__, |\___|_| |_|_/___\__,_|
     |___/                       

this is a very poor / partial / non-compliant dat implementation in rust.
it will eat-your-data.

### Progress

- [ ] SLEEP v2 Files
    - [x] read/write file headers
    - [x] read/write file chunks as raw bytes
    - [ ] pread/pwrite file chunks without seeking
    - [x] read data entries by index
    - [ ] append data entries
    - [ ] verify entire register (signatures and merkel tree)
    - [ ] bitfields
- [ ] Drive metadata and files
    - [ ] add file to register
    - [ ] checkout file from register
    - [ ] print file tree ("ls")
    - [ ] add directory recursively
    - [ ] checkout directory recursively
- [ ] Networking
    - [ ] send/receive encrypted messages to a known host
    - [ ] pull register from a known host
    - [ ] sync register to a known host
    - [ ] lookup hosts in DHT swarm by discovery key
- [ ] Wrapper command
    - [ ] clone
    - [ ] share
    - [ ] log
    - [ ] status
    - [ ] add

### Dependencies

Requires libsodium-dev system-wide (for now).

Notable Rust Libraries:
- rust-crypto for blake2b 
- sodiumoxide Ed25519 (signing) and XSalsa20 (stream)
- integer-encoding for simple varints (network protocol)
- rust-protobuf for protobuf (network protocol)
- ??? for Kademlia Mainline DHT

protobuf code generated with `rust-protobuf`; the `protoc` tool (`sudo apt
install protobuf-compiler`) and rust plugin `protoc-gen-rust` (`cargo install
protobuf`) are only needed when changing .proto files, eg:

    protoc --rust_out . network_proto.proto
