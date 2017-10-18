
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
extern crate integer_encoding;
extern crate crypto;
extern crate rand;

use std::io::prelude::*;
use std::io::SeekFrom;
use std::path::Path;
use std::fs::File;
use integer_encoding::FixedInt;
use std::fs::OpenOptions;
use crypto::ed25519;
use crypto::blake2b::Blake2b;
use crypto::digest::Digest;
use rand::Rng;

#[allow(unused_doc_comment)]

mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain! {
        foreign_links { Fmt(::std::fmt::Error);
                        Io(::std::io::Error) #[cfg(unix)]; }
    }
}

#[doc(hidden)]
pub use errors::*;


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

    /// Returns the count of entries, meaning the highest index entry plus one (not necessarily the
    /// number of entries which have actually been written).
    fn len(&self) -> Result<u64>;
}

/// Local File implementation of SleepStorage
#[derive(Debug)]
pub struct SleepFile {
    file: File,
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
            return Err("Invalid SLEEP header: version must be 0".into());
        }
        let algo_len: u8 = header[7];
        if algo_len > 24 {
            return Err("Invalid SLEEP header: can't have algo_len > 24".into());
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
            return Err("Tried to read beyond end of SLEEP file".into());
        }
        let mut entry = vec![0; entry_size];
        self.file.seek(SeekFrom::Start(32 + (entry_size as u64) * index))?;
        self.file.read_exact(&mut entry)?;
        Ok(entry)
    }

    fn write(&mut self, index: u64, data: &[u8]) -> Result<()> {
        // TODO: need to extend file seek beyond end?
        if data.len() != self.entry_size as usize {
            return Err("Tried to write mis-sized data".into());
        }
        self.file.seek(SeekFrom::Start(32 + (self.entry_size as u64) * index))?;
        self.file.write_all(&data)?;
        Ok(())
    }

    fn len(&self) -> Result<u64> {
        let length = self.file.metadata()?.len();
        if length < 32 || (length - 32) % (self.entry_size as u64) != 0 {
            return Err("Bad SLEEP file: missing header or not multiple of entry_size".into());
        }
        return Ok((length - 32) / (self.entry_size as u64))
    }
}


/// Abstract access to Hypercore register
pub trait HyperRegister {

    /// Whether the register store contains the given (data) entry
    fn has(&self, index: u64) -> Result<bool>;

    /// Whether the register store contains *all* known (data) entries
    fn has_all(&self) -> Result<bool>;

    /// If the contiguous range of entries is in the store
    fn has_range(&self, start: u64, end: u64) -> Result<bool>;

    /// Reads a single data entry from the store.
    fn get_data_entry(&mut self, index: u64) -> Result<Vec<u8>>;

    /// Writes an entry to the store. Requires the private key to be present.
    fn append(&mut self, data: &[u8]) -> Result<u64>;

    /// Count of data entries for this register. This is the total count (highest entry index plus
    /// one); this particular store might be sparse.
    fn len(&self) -> Result<u64>;

    /// Total size of this register in bytes.
    fn len_bytes(&self) -> Result<u64>;

    /// [UNIMPLEMENTED] Intended to do a deeper merkel-tree verification of all stored data
    fn verify(&self) -> Result<()>;

    /// Quick sanity checks on register store robust-ness
    fn check(&self) -> Result<()>;

    /// Can this register be appended to?
    fn writable(&self) -> bool;

    /// Returns a single tree entry (using tree indexing, not data indexing).
    fn get_tree_entry(&mut self, index: u64) -> Result<Vec<u8>>;
}

/// Implementation of HyperRegister using a local directory of SLEEP files
#[derive(Debug)]
pub struct SleepDirRegister {
    tree_sleep: SleepFile,
    sign_sleep: SleepFile,
    bitfield_sleep: SleepFile,
    data_file: Option<File>,
    // Except, these should be Ed25519 keys, not bytes
    pub_key: Vec<u8>,
    secret_key: Option<Vec<u8>>,
}

impl SleepDirRegister {

    pub fn open(directory: &Path, prefix: &str, writable: bool) -> Result<SleepDirRegister> {
        // read public key from disk
        let mut pub_key: Vec<u8> = vec![];
        {
            let mut key_file = OpenOptions::new()
                .read(true)
                .write(false)
                .open(directory.join(Path::new(&(prefix.to_owned() + ".key"))))?;
            // TODO: check key length?
            key_file.read_to_end(&mut pub_key)?;
        }
        let data_path = &(prefix.to_owned() + ".data");
        let data_path = Path::new(data_path);
        let data_file = if data_path.is_file() {
            Some(OpenOptions::new()
                .read(true)
                .write(writable)
                .open(data_path)?)
        } else {
            None
        };
        let tree_sleep = SleepFile::open(
            &directory.join(Path::new(&(prefix.to_owned() + ".tree"))), writable)?;
        let sign_sleep = SleepFile::open(
            &directory.join(Path::new(&(prefix.to_owned() + ".signatures"))), writable)?;
        let bitfield_sleep = SleepFile::open(
            &directory.join(Path::new(&(prefix.to_owned() + ".bitfield"))), writable)?;
        let sf = SleepDirRegister {
            tree_sleep,
            sign_sleep,
            bitfield_sleep,
            data_file,
            pub_key,
            secret_key: None,
        };
        sf.check()?;
        Ok(sf)
    }

    pub fn create(directory: &Path, prefix: &str) -> Result<SleepDirRegister> {
        // TODO: audit this for crypto strength... is rand appropriate?
        let mut rand_seed = vec![0; 32];
        let mut rng = rand::OsRng::new()?;
        rng.fill_bytes(&mut rand_seed);
        let (secret_key, pub_key) = ed25519::keypair(&rand_seed);
        println!("{:?}", directory.join(Path::new(&(prefix.to_owned() + ".key"))));
        {
            let mut key_file = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(directory.join(Path::new(&(prefix.to_owned() + ".key"))))?;
            key_file.write_all(&pub_key)?;
        }
        let data_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create_new(true)
            .open(directory.join(Path::new(&(prefix.to_owned() + ".data"))))?;
        let tree_sleep = SleepFile::create(
            &directory.join(Path::new(&(prefix.to_owned() + ".tree"))),
            0x05025702,
            40,
            Some("BLAKE2b".to_string()))?;
        let sign_sleep = SleepFile::create(
            &directory.join(Path::new(&(prefix.to_owned() + ".signatures"))),
            0x05025701,
            65,
            Some("Ed25519".to_string()))?;
        let bitfield_sleep = SleepFile::create(
            &directory.join(Path::new(&(prefix.to_owned() + ".bitfield"))),
            0x05025700,
            3328,
            None)?;
        let sf = SleepDirRegister {
            tree_sleep,
            sign_sleep,
            bitfield_sleep,
            data_file: Some(data_file),
            pub_key: pub_key.to_vec(),
            secret_key: Some(secret_key.to_vec()),
        };
        sf.check()?;
        Ok(sf)
    }
}

impl HyperRegister {

    fn hash_leaf(data: &[u8]) -> [u8; 40] {
        let mut buf = [0; 40];
        u64::to_be(data.len() as u64)
            .encode_fixed(&mut buf[32..40]);
        let mut hash = Blake2b::new(32);
        hash.input(&[0; 1]);
        hash.input(&buf[32..40]);
        hash.input(&data);
        hash.result(&mut buf[0..32]);
        buf
    }

    fn hash_parent(lhash: &[u8; 40], rhash: &[u8; 40]) -> [u8; 40] {
        let mut buf = [0; 40];
        // TODO: check overflow
        let sum_size = u64::from_be(FixedInt::decode_fixed(&lhash[32..40])) +
                       u64::from_be(FixedInt::decode_fixed(&rhash[32..40]));
        u64::to_be(sum_size as u64)
            .encode_fixed(&mut buf[32..40]);

        let mut hash = Blake2b::new(32);
        hash.input(&[1; 1]);
        hash.input(&buf[32..40]);
        hash.input(&lhash[..]);
        hash.input(&rhash[..]);
        hash.result(&mut buf[0..32]);
        buf
    }

    pub fn hash_roots(reg: &mut HyperRegister, index: u64) -> Result<Vec<u8>> {
        let mut buf = [0; 40];
/*
        // TODO: check overflow
        let sum_size = u64::from_be(FixedInt::decode_fixed(&lhash[32..40])) +
                       u64::from_be(FixedInt::decode_fixed(&rhash[32..40]));
        u64::to_be(sum_size as u64)
            .encode_fixed(&mut buf[32..40]);
*/

        let mut hash = Blake2b::new(32);
        let mut index_buf = [0; 8];
        // TODO: turn these single bytes into declared constants
        hash.input(&[2; 1]);
        for ri in HyperRegister::root_nodes(index) {
            u64::to_be(ri).encode_fixed(&mut index_buf);
            let node = reg.get_tree_entry(ri)?;
            hash.input(&node[0..32]);
            hash.input(&index_buf);
            hash.input(&node[32..40]);
        }
        hash.result(&mut buf[0..32]);
        Ok(buf.to_vec())

    }

    fn root_nodes(data_count: u64) -> Vec<u64> {
        // Calculates the root notes for a given length (of data entries, not tree entries)
        // TODO: this should be an iterator
        // NB: this is a relatively "hot" function, gets called (repeatedly?) on every mutation,
        // and potentially in inner loops of lookups.
        if data_count == 0 {
            return vec![];
        }

        // Convert the count to a (descending) list of power-of-2 components
        let mut x = 0;
        let mut components = vec![];
        while 2u64.pow(x) <= data_count {
            if (data_count & 2u64.pow(x)) != 0 {
                components.push(2u64.pow(x));
            }
            x += 1;
        }
        components.reverse();

        // Add and accumulate
        let mut accum = 0;
        let mut roots = vec![];
        for x in components {
            roots.push(accum + (x - 1));
            accum += 2*x;
        }
        roots
    }

    pub fn get_data_offset(reg: &mut HyperRegister, index: u64) -> Result<u64> {
        // TODO: this is a naive (linear) implementation
        // log(N) would go up previous parent nodes (eg, use root_nodes())
        let mut sum: u64 = 0;
        for i in 0..index {
            let mut leaf = reg.get_tree_entry(i*2)?;
            // TODO: overflow
            sum += u64::from_be(FixedInt::decode_fixed(&leaf[32..40]));
        }
        Ok(sum)
    }
}

#[test]
fn test_root_nodes() {
    assert_eq!(HyperRegister::root_nodes(0), vec![]);
    assert_eq!(HyperRegister::root_nodes(1), vec![0]);
    assert_eq!(HyperRegister::root_nodes(2), vec![1]);
    assert_eq!(HyperRegister::root_nodes(3), vec![1,4]);
    assert_eq!(HyperRegister::root_nodes(4), vec![3]);
    assert_eq!(HyperRegister::root_nodes(5), vec![3,8]);
    assert_eq!(HyperRegister::root_nodes(6), vec![3,9]);
    assert_eq!(HyperRegister::root_nodes(7), vec![3,9,12]);
    assert_eq!(HyperRegister::root_nodes(8), vec![7]);
}

impl HyperRegister for SleepDirRegister {

    fn has(&self, index: u64) -> Result<bool> {
        // looks in bitfield
        unimplemented!()
    }

    fn has_all(&self) -> Result<bool> {
        self.has_range(0, self.len()?)
    }

    fn has_range(&self, start: u64, end: u64) -> Result<bool> {
        // This function is un-motivated and could be removed
        assert!(end > start);
        for i in start..end {
            if !self.has(i)? {
                return Ok(false);
            }
        }
        Ok(true)
    }

    fn get_data_entry(&mut self, index: u64) -> Result<Vec<u8>> {

        // Get metadata about chunk (offset and length)
        let offset = HyperRegister::get_data_offset(self, index)?;

        // Do we even have this chunk?
        if !self.has(index)? {
            return Err("Don't have that chunk".into());
        }

        let mut data_file = if let Some(ref mut df) = self.data_file {
            df
        } else {
            return Err("No data file in this register".into());
        };
        let mut leaf = self.tree_sleep.read(index*2)?;
        let data_len = u64::from_be(FixedInt::decode_fixed(&leaf[32..40]));
        // TODO: avoid foot-gun in development: cap at ~1 billion bytes
        assert!(data_len < 2u64.pow(29));

        // Read chunk
        let mut data = vec![0; data_len as usize];
        data_file.seek(SeekFrom::Start(offset))?;
        data_file.read_exact(&mut data)?;

        // TODO: check the hash? separate function?
        Ok(data)
    }

    fn get_tree_entry(&mut self, index: u64) -> Result<Vec<u8>> {
        self.tree_sleep.read(index)
    }

    fn append(&mut self, data: &[u8]) -> Result<u64> {

        let index = self.len()?;
        let hash = HyperRegister::hash_roots(self, index+1)?;

        let mut data_file = if let Some(ref df) = self.data_file {
            df
        } else {
            return Err("No data file in this register".into());
        };
        // 1. Hash data chunk
        // 2. Append data to data file
        data_file.seek(SeekFrom::End(0))?;
        data_file.write_all(data)?;
        // 3. Add hash to tree
        // 4. Add signature to signature file
        // 5. Update bitfile
        unimplemented!()
    }

    fn len(&self) -> Result<u64> {
        // Length in entry count.
        let tree_len = self.tree_sleep.len()?;
        if tree_len == 0 {
            Ok(0)
        } else if tree_len % 2 != 1 {
            Err("Even number of tree file SLEEP entries".into())
        } else {
            Ok((self.tree_sleep.len()? / 2) + 1)
        }
    }

    fn len_bytes(&self) -> Result<u64> {
        // Total binary size of data file.
        let mut data_file = if let Some(ref df) = self.data_file {
            df
        } else {
            return Err("No data file in this register".into());
        };
        // Elaborate version will iterate through tree root nodes.
        Ok(data_file.metadata()?.len())
    }

    fn verify(&self) -> Result<()> {
        unimplemented!()
    }

    fn check(&self) -> Result<()> {
        let sign_len = self.sign_sleep.len()?;
        let tree_len = self.tree_sleep.len()?;
        if (tree_len == 0) && (sign_len == 0) {
            return Ok(())
        }
        if tree_len != (sign_len * 2) - 1 {
            return Err("Inconsistent SLEEP signature/tree file sizes".into());
        }
        Ok(())
    }

    fn writable(&self) -> bool {
        unimplemented!()
        //self.sign_sleep.file.writable()
    }
}
