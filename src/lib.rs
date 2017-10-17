
#[macro_use]
extern crate error_chain;
extern crate integer_encoding;

use std::io::prelude::*;
use std::path::Path;
use std::fs::File;
use integer_encoding::FixedInt;

mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain! {
        foreign_links { Fmt(::std::fmt::Error);
                        Io(::std::io::Error) #[cfg(unix)]; }
    }
}
pub use errors::*;


// Abstract access to SLEEP content
pub trait SleepStorage {
    fn get_magic(&self) -> u32;
    fn get_algorithm(&self) -> Option<String>;
    fn get_entry_size(&self) -> u16;
    fn read(&self, index: u64) -> Result<&[u8]>;
    fn write(&self, index: u64, data: &[u8]) -> Result<()>;
    fn length(&self) -> Result<u64>;
}

#[derive(Debug)]
pub struct SleepFile {
    file: File,
    magic: u32,
    entry_size: u16,
    algorithm_name: Option<String>,
}

impl SleepFile {

    // TODO: 'from' File trait

    pub fn open(path: &Path, writable: bool) -> Result<SleepFile> {

        // TODO: use writable here
        let mut f = File::open(path)?;
        let mut header = [0; 32];
        f.read_exact(&mut header)?;
        let version: u8 = header[4];
        if version != 0 {
            return Err("Invalid SLEEP header: version must be 0".into());
        }
        let algo_len: u8 = header[8];
        if algo_len > 24 {
            return Err("Invalid SLEEP header: can't have algo_len > 24".into());
        }
        let algorithm_name = if algo_len == 0 { None } else {
            Some(String::from_utf8_lossy(&header[8..(8+(algo_len as usize))]).into_owned())
        };
        // TODO: endian-ness of u16 entry_size
        Ok(SleepFile {
            file: f,
            magic: FixedInt::decode_fixed(&header[0..4]),
            entry_size: FixedInt::decode_fixed(&header[6..8]),
            algorithm_name: algorithm_name,
        })
    }
/*
    pub fn create(path: Path, magic: u32, entry_size: u16, algo: &str) -> Result<SleepFile> {
        let mut sf = SleepFile {

        };
    }
*/
}

impl SleepStorage for SleepFile {

    fn get_magic(&self) -> u32 { self.magic }
    fn get_algorithm(&self) -> Option<String> { self.algorithm_name.clone() }
    fn get_entry_size(&self) -> u16 { self.entry_size }

    fn read(&self, index: u64) -> Result<&[u8]> {
        unimplemented!()
        //Err("unimplemented")
    }

    fn write(&self, index: u64, data: &[u8]) -> Result<()> {
        Ok(())
    }

    fn length(&self) -> Result<u64> {
        let length = self.file.metadata()?.len();
        // TODO: assert that length >= 32
        // TODO: assert that (length - 32) % self.entry_size == 0
        return Ok((length - 32) / (self.entry_size as u64))
    }

}
