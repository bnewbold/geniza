
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

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate integer_encoding;
extern crate crypto;
extern crate rand;
extern crate protobuf;

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
mod sleep;
pub use sleep::*;
mod register;
pub use register::*;
mod sync;
pub use sync::*;
pub mod network_proto;
pub mod drive_proto;
