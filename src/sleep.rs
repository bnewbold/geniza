
use std::io::prelude::*;
use std::io::SeekFrom;
use std::path::Path;
use std::fs::File;
use integer_encoding::FixedInt;
use std::fs::OpenOptions;

use errors::*;

/// Abstract access to SLEEP content
///
/// Back-ends could be in RAM, on disk, remote HTTP, etc.
pub trait SleepStorage {

    /// Returns the 32-bit "magic word", indicating the content type, in native format (aka, not
    /// necessarily big-endian).
    fn get_magic(&self) -> u32;

    /// If the algorithm string is empty, returns None, otherwise the String (owned), decoded from
    /// UTF-8. Encoded (bytes) representation is at most 24 bytes.
    fn get_algorithm(&self) -> Option<String>;

    /// Size (in bytes) of each entry for this SLEEP file.
    fn get_entry_size(&self) -> u16;

    /// Returns a single raw entry at the given index (which is not a byte offset).
    /// TODO: should write into a supplied buffer and return size.
    fn read(&mut self, index: u64) -> Result<Vec<u8>>;

    /// Writes an entry at the given entry index (which is not a byte offset).
    fn write(&mut self, index: u64, data: &[u8]) -> Result<()>;

    /// Writes a new entry at the end of the file
    fn append(&mut self, data: &[u8]) -> Result<()>;

    /// Returns the count of entries, meaning the highest index entry plus one (not necessarily the
    /// number of entries which have actually been written).
    fn len(&self) -> Result<u64>;
}

/// Local File implementation of SleepStorage
#[derive(Debug)]
pub struct SleepFile {
    pub file: File,
    magic: u32,
    entry_size: u16,
    // Option isn't necessary here... idiomatic?
    algorithm_name: Option<String>,
}

impl SleepFile {

    // TODO: 'from' pre-existing File

    // Something here to allow paths as references or actual Path...
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
            bail!("Invalid SLEEP header: version must be 0");
        }
        let algo_len: u8 = header[7];
        if algo_len > 24 {
            bail!("Invalid SLEEP header: can't have algo_len > 24");
        }
        let algorithm_name = if algo_len == 0 { None } else {
            Some(String::from_utf8_lossy(&header[8..(8+(algo_len as usize))]).into_owned())
        };
        let sf = SleepFile {
            file: f,
            magic: u32::from_be(FixedInt::decode_fixed(&header[0..4])),
            entry_size: u16::from_be(FixedInt::decode_fixed(&header[5..7])),
            algorithm_name: algorithm_name,
        };
        // call length for consistency checks
        sf.len()?;
        Ok(sf)
    }

    /// This function will *not* allow overwriting an existing file.
    pub fn create(path: &Path, magic: u32, entry_size: u16, algo: Option<String>) -> Result<SleepFile> {

        let mut header = [0; 32];
        u32::to_be(magic).encode_fixed(&mut header[0..4]);
        header[4] = 0; // version
        u16::to_be(entry_size).encode_fixed(&mut header[5..7]);
        if let Some(name) = algo.clone() {
            let name = name.as_bytes();
            let algo_len = name.len();
            if algo_len > 24 {
                bail!("Algorithm name must be 24 bytes at most");
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
        f.write_all(&header)?;
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

    fn read(&mut self, index: u64) -> Result<Vec<u8>> {
        let entry_size = self.entry_size as usize;
        if index + 1 > self.len()? {
            bail!("Tried to read beyond end of SLEEP file");
        }
        let mut entry = vec![0; entry_size];
        self.file.seek(SeekFrom::Start(32 + (entry_size as u64) * index))?;
        self.file.read_exact(&mut entry)?;
        Ok(entry)
    }

    fn write(&mut self, index: u64, data: &[u8]) -> Result<()> {
        // TODO: need to extend file seek beyond end?
        if data.len() != self.entry_size as usize {
            bail!("Tried to write mis-sized data");
        }
        self.file.seek(SeekFrom::Start(32 + (self.entry_size as u64) * index))?;
        self.file.write_all(&data)?;
        Ok(())
    }

    fn append(&mut self, data: &[u8]) -> Result<()> {
        let index = self.len()?;
        self.write(index, data)
    }

    fn len(&self) -> Result<u64> {
        let length = self.file.metadata()?.len();
        if length < 32 || (length - 32) % (self.entry_size as u64) != 0 {
            bail!("Bad SLEEP file: missing header or not multiple of entry_size");
        }
        return Ok((length - 32) / (self.entry_size as u64))
    }
}

#[test]
fn test_sleep_open() {

    let sf = SleepFile::open(
       Path::new("test-data/sleep/empty/empty.sleep"), false).unwrap();

    assert_eq!(sf.len().unwrap(), 0);
    assert_eq!(sf.get_magic(), 0x050257FF);
    assert_eq!(sf.get_algorithm(), None);
    assert_eq!(sf.get_entry_size(), 1);

    let sf = SleepFile::open(
       Path::new("test-data/dat/simple/.dat/metadata.tree"), false).unwrap();

    // Calculated from 'dat log'
    assert_eq!(sf.len().unwrap(), 5);
    assert_eq!(sf.get_magic(), 0x05025702);
    assert_eq!(sf.get_algorithm(), Some("BLAKE2b".to_string()));
    assert_eq!(sf.get_entry_size(), 40);
}

#[test]
fn test_sleep_create() {

    use tempdir::TempDir;
    let tmp_dir = TempDir::new("geniza-test").unwrap();

    SleepFile::create(
        &tmp_dir.path().join("empty2.sleep"),
        0x050257FF,
        1,
        None).unwrap();

    // TODO: binary diff against 'test-data/sleep/empty/empty.sleep'

    SleepFile::create(
        &tmp_dir.path().join("simple_metadata.sleep"),
        0x05025702,
        40,
        Some("BLAKE2b".into())).unwrap();
}
