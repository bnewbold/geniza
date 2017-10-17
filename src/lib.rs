
#[macro_use]
extern crate error_chain;
extern crate integer_encoding;

use std::io::prelude::*;
use std::path::Path;
use std::fs::File;
use integer_encoding::FixedInt;
use std::fs::OpenOptions;

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
    // Option isn't necessary here... idiomatic?
    algorithm_name: Option<String>,
}

impl SleepFile {

    // TODO: 'from' File trait

    pub fn open(path: &Path, writable: bool) -> Result<SleepFile> {

        let mut f = OpenOptions::new()
            .read(true)
            .write(writable)
            .create(false)
            .open(path)?;
        let mut header = [0; 32];
        f.read_exact(&mut header)?;
        let version: u8 = header[4];
        if version != 0 {
            return Err("Invalid SLEEP header: version must be 0".into());
        }
        let algo_len: u8 = header[7];
        if algo_len > 24 {
            return Err("Invalid SLEEP header: can't have algo_len > 24".into());
        }
        let algorithm_name = if algo_len == 0 { None } else {
            Some(String::from_utf8_lossy(&header[8..(8+(algo_len as usize))]).into_owned())
        };
        // TODO: endian-ness of u16 entry_size
        Ok(SleepFile {
            file: f,
            magic: u32::from_be(FixedInt::decode_fixed(&header[0..4])),
            entry_size: u16::from_be(FixedInt::decode_fixed(&header[5..7])),
            algorithm_name: algorithm_name,
        })
    }

    pub fn create(path: &Path, magic: u32, entry_size: u16, algo: Option<String>) -> Result<SleepFile> {
        // This function will *not* allow overwriting an existing file.

        let mut header = [0; 32];
        u32::to_be(magic).encode_fixed(&mut header[0..4]);
        header[4] = 0; // version
        u16::to_be(entry_size).encode_fixed(&mut header[5..7]);
        if let Some(name) = algo.clone() {
            let name = name.as_bytes();
            let algo_len = name.len();
            if algo_len > 24 {
                return Err("Algorithm name must be 24 bytes at most".into());
            }
            header[7] = algo_len as u8;
            header[8..(8+algo_len)].clone_from_slice(name);
        } else {
            header[7] = 0;
        };

        let mut f = OpenOptions::new()
            .read(true)
            .write(true)
            .create_new(true)
            .open(path)?;
        f.write(&header)?;
        Ok(SleepFile {
            file: f,
            magic: magic,
            entry_size: entry_size,
            algorithm_name: algo,
        })
    }
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
