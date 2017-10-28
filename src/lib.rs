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
