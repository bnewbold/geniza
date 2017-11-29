// Free Software under GPL-3.0, see LICENSE
// Copyright 2017 Bryan Newbold

//! This is a hobby/learning implementation of the dat distributed data synchronization system.
//!
//! Subcomponents are roughly organized in library form for easier reading/review, but buyer-beware
//! actually trying to reuse this code for anything other than education or interop testing.
//!
//! ### References
//!
//! [Dat Whitepaper](https://github.com/datproject/docs)
//!
//! Additional notes in the source code for this repo, under the 'notes' directory. Also, see
//! README.

extern crate crypto;
extern crate env_logger;
#[macro_use]
extern crate error_chain;
extern crate integer_encoding;
#[macro_use]
extern crate log;
extern crate protobuf;
extern crate rand;
extern crate sodiumoxide;
extern crate bit_field;

#[cfg(test)]
extern crate tempdir;

#[allow(unused_doc_comment)]
mod errors {

    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain! {
        foreign_links { Fmt(::std::fmt::Error);
                        Io(::std::io::Error) #[cfg(unix)];
                        Protobuf(::protobuf::ProtobufError); }
    }
}

#[doc(hidden)]
pub use errors::*;

// Organize code internally (files, modules), but pull it all into a flat namespace to export.
mod sleep_file;
pub use sleep_file::*;
mod sleep_register;
pub use sleep_register::*;
mod drive;
pub use drive::*;
mod protocol;
pub use protocol::*;
pub mod network_msgs;
pub mod metadata_msgs;
mod node;
pub use node::*;

// Shared functions
use crypto::digest::Digest;
use crypto::blake2b::Blake2b;

/// Helper to calculate a discovery key from a public key. 'key' should be 32 bytes; the returned
/// array will also be 32 bytes long.
///
/// dat discovery keys are calculated as a BLAKE2b "keyed hash" (using the passed key) of the string
/// "hypercore" (with no trailing null byte).
pub fn make_discovery_key(key: &[u8]) -> Vec<u8> {
    let mut discovery_key = [0; 32];
    let mut hash = Blake2b::new_keyed(32, key);
    hash.input(&"hypercore".as_bytes());
    hash.result(&mut discovery_key);
    discovery_key.to_vec()
}

