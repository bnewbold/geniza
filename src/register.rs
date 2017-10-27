
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::io::SeekFrom;
use integer_encoding::FixedInt;
use std::fs::OpenOptions;
use crypto::blake2b::Blake2b;
use crypto::digest::Digest;
use crypto::ed25519;
use rand::{Rng, OsRng};

use errors::*;
use sleep::*;

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
    fn len_bytes(&mut self) -> Result<u64>;

    /// [UNIMPLEMENTED] Intended to do a deeper merkel-tree verification of all stored data
    fn verify(&mut self) -> Result<()>;

    /// Quick sanity checks on register store robust-ness
    fn check(&mut self) -> Result<()>;

    /// Can this register be appended to?
    fn writable(&self) -> bool;

    /// Returns a single tree entry (using tree indexing, not data indexing).
    fn get_tree_entry(&mut self, index: u64) -> Result<Vec<u8>>;
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

    fn hash_parent(lhash: &[u8], rhash: &[u8]) -> [u8; 40] {
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
        let mut hash = Blake2b::new(32);
        let mut index_buf = [0; 8];
        hash.input(&[2; 1]);
        for ri in HyperRegister::tree_root_nodes(index) {
            u64::to_be(ri).encode_fixed(&mut index_buf);
            let node = reg.get_tree_entry(ri)?;
            hash.input(&node[0..32]);
            hash.input(&index_buf);
            hash.input(&node[32..40]);
        }
        hash.result(&mut buf[0..32]);
        Ok(buf.to_vec())

    }

    fn tree_root_nodes(data_count: u64) -> Vec<u64> {
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
            let leaf = reg.get_tree_entry(i*2)?;
            sum += u64::from_be(FixedInt::decode_fixed(&leaf[32..40]));
        }
        Ok(sum)
    }

    /// Every node has a parent, so this function won't fail unless index is over 2^62, in which
    /// case it would overflow and panics instead.
    fn tree_parent_index(index: u64) -> u64 {
        for i in 0..62 {
            // find lowest-significant zero bit
            if (index & (1 << i)) == 0 {
                // set that bit and clear next higher
                return ((index | (1 << i))) & !(1 << (i+1));
            }
        }
        panic!("Parent lookup overflowed, huge index!");
    }

    /// Calling this on a leaf node is an error, as is calling very high node numbers (> 2^62)
    fn tree_child_indices(index: u64) -> Result<(u64,u64)> {
        if index % 2 == 0 {
            bail!("Leaf tree nodes have no children");
        }
        for i in 0..62 {
            // find lowest-significant zero bit...
            if (index & (1 << i)) == 0 {
                // larger child has this bit high, next lower bit cleared
                let right = ((index | (1 << i))) & !(1 << (i-1));
                // smaller child has next lower bit cleared
                let left = index & !(1 << (i-1));
                return Ok((left, right));
            }
        }
        bail!("Child lookup overflowed, huge index!");
    }
}

#[test]
fn test_tree_root_nodes() {
    assert_eq!(HyperRegister::tree_root_nodes(0), vec![]);
    assert_eq!(HyperRegister::tree_root_nodes(1), vec![0]);
    assert_eq!(HyperRegister::tree_root_nodes(2), vec![1]);
    assert_eq!(HyperRegister::tree_root_nodes(3), vec![1,4]);
    assert_eq!(HyperRegister::tree_root_nodes(4), vec![3]);
    assert_eq!(HyperRegister::tree_root_nodes(5), vec![3,8]);
    assert_eq!(HyperRegister::tree_root_nodes(6), vec![3,9]);
    assert_eq!(HyperRegister::tree_root_nodes(7), vec![3,9,12]);
    assert_eq!(HyperRegister::tree_root_nodes(8), vec![7]);
}

#[test]
fn test_tree_parent_index() {
    assert_eq!(HyperRegister::tree_parent_index(0), 1);
    assert_eq!(HyperRegister::tree_parent_index(1), 3);
    assert_eq!(HyperRegister::tree_parent_index(2), 1);
    assert_eq!(HyperRegister::tree_parent_index(3), 7);
    assert_eq!(HyperRegister::tree_parent_index(4), 5);
    assert_eq!(HyperRegister::tree_parent_index(5), 3);
    assert_eq!(HyperRegister::tree_parent_index(6), 5);
    assert_eq!(HyperRegister::tree_parent_index(7), 15);
    assert_eq!(HyperRegister::tree_parent_index(8), 9);
    assert_eq!(HyperRegister::tree_parent_index(9), 11);
    assert_eq!(HyperRegister::tree_parent_index(21), 19);
    assert_eq!(HyperRegister::tree_parent_index(22), 21);
}

#[test]
fn test_tree_child_indices() {
    assert!(HyperRegister::tree_child_indices(0).is_err());
    assert!(HyperRegister::tree_child_indices(1024).is_err());
    assert_eq!(HyperRegister::tree_child_indices(1).unwrap(),  (0, 2));

    assert_eq!(HyperRegister::tree_child_indices(3).unwrap(),  (1, 5));
    assert_eq!(HyperRegister::tree_child_indices(5).unwrap(),  (4, 6));
    assert_eq!(HyperRegister::tree_child_indices(7).unwrap(),  (3, 11));
    assert_eq!(HyperRegister::tree_child_indices(9).unwrap(),  (8, 10));
    assert_eq!(HyperRegister::tree_child_indices(11).unwrap(), (9, 13));
    assert_eq!(HyperRegister::tree_child_indices(13).unwrap(), (12, 14));
    assert_eq!(HyperRegister::tree_child_indices(15).unwrap(), (7, 23));
    assert_eq!(HyperRegister::tree_child_indices(19).unwrap(), (17, 21));
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
        let mut sf = SleepDirRegister {
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

    /// In addition to what one would expect, also creates an Ed25519 key-pair using OsRng
    pub fn create(directory: &Path, prefix: &str) -> Result<SleepDirRegister> {
        let mut rand_seed = vec![0; 32];
        let mut rng = OsRng::new()?;
        rng.fill_bytes(&mut rand_seed);
        let (secret_key, pub_key) = ed25519::keypair(&rand_seed);
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
            64,
            Some("Ed25519".to_string()))?;
        let bitfield_sleep = SleepFile::create(
            &directory.join(Path::new(&(prefix.to_owned() + ".bitfield"))),
            0x05025700,
            3328,
            None)?;
        let mut sf = SleepDirRegister {
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

impl HyperRegister for SleepDirRegister {

    fn has(&self, _index: u64) -> Result<bool> {
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
            bail!("Don't have that chunk");
        }

        let data_file = if let Some(ref mut df) = self.data_file {
            df
        } else {
            bail!("No data file in this register");
        };
        let leaf = self.tree_sleep.read(index*2)?;
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

        if !self.data_file.is_some() {
            bail!("No data file in this register");
        };

        let index = self.len()?;
        // 1. Hash data chunk
        let leaf_hash = HyperRegister::hash_leaf(data);

        // 2. Append data to data file
        if let Some(ref mut df) = self.data_file {
            df.seek(SeekFrom::End(0))?;
            df.write_all(data)?;
            df.sync_data()?;
        }

        // 3. Add hash to tree file, update merkel tree
        self.tree_sleep.write(index*2, &leaf_hash)?;
        let mut parent = HyperRegister::tree_parent_index(index*2);
        while parent < index*2 {
            let (left, right) = HyperRegister::tree_child_indices(parent)?;
            let (left, right) = (self.tree_sleep.read(left)?,
                                 self.tree_sleep.read(right)?);
            let parent_hash = HyperRegister::hash_parent(&left[0..40], &right[0..40]);
            self.tree_sleep.write(parent, &parent_hash[0..40])?;
            parent = HyperRegister::tree_parent_index(parent);
        }
 
        // 4. Add signature to signature file
        let root_hash = HyperRegister::hash_roots(self, index+1)?;
        let root_sig = ed25519::signature(&root_hash, &self.secret_key.clone().unwrap());
        self.sign_sleep.append(&root_sig)?;
 
        // 5. Update bitfile
        Ok(index)
    }

    fn len(&self) -> Result<u64> {
        // Length in entry count.
        let tree_len = self.tree_sleep.len()?;
        if tree_len == 0 {
            Ok(0)
        } else if tree_len % 2 != 1 {
            bail!("Even number of tree file SLEEP entries");
        } else {
            Ok((self.tree_sleep.len()? / 2) + 1)
        }
    }

    fn len_bytes(&mut self) -> Result<u64> {
        // TODO: this is a naive (linear) implementation
        // log(N) would go up previous parent nodes (eg, use tree_root_nodes())
        let mut sum: u64 = 0;
        for i in 0..self.len()? {
            let leaf = self.get_tree_entry(i*2)?;
            sum += u64::from_be(FixedInt::decode_fixed(&leaf[32..40]));
        }
        Ok(sum)
    }

    fn verify(&mut self) -> Result<()> {
        unimplemented!()
    }

    fn check(&mut self) -> Result<()> {
        let sign_len = self.sign_sleep.len()?;
        let tree_len = self.tree_sleep.len()?;
        if (tree_len == 0) && (sign_len == 0) {
            return Ok(())
        }
        if tree_len != (sign_len * 2) - 1 {
            bail!("Inconsistent SLEEP signature/tree file sizes");
        }
        let computed = self.len_bytes()?;
        if let Some(ref df) = self.data_file {
            let file_size = df.metadata()?.len();
            if file_size != computed {
                bail!("Computed vs. data file size mismatch");
            }
        }
        Ok(())
    }

    fn writable(&self) -> bool {
        unimplemented!()
    }
}

#[test]
fn test_sdr_open() {

    let mut sdr = SleepDirRegister::open(
       Path::new("test-data/dat/simple/.dat/"), "metadata", false).unwrap();

    // Values from 'dat log'
    assert_eq!(sdr.len().unwrap(), 3);
    assert_eq!(sdr.len_bytes().unwrap(), 145);

    let mut sdr = SleepDirRegister::open(
       Path::new("test-data/dat/simple/.dat/"), "content", false).unwrap();

    // Values from 'dat log'
    assert_eq!(sdr.len().unwrap(), 2);
    assert_eq!(sdr.len_bytes().unwrap(), 204);
}

#[test]
fn test_sdr_create() {

    use tempdir::TempDir;

    let tmp_dir = TempDir::new("geniza-test").unwrap();

    let mut sdr = SleepDirRegister::create(tmp_dir.path(), "dummy").unwrap();

    assert_eq!(sdr.len().unwrap(), 0);
    assert_eq!(sdr.len_bytes().unwrap(), 0);
}

#[test]
fn test_sdr_append() {

    use tempdir::TempDir;

    let tmp_dir = TempDir::new("geniza-test").unwrap();

    let mut sdr = SleepDirRegister::create(tmp_dir.path(), "dummy").unwrap();

    sdr.append("hello world!".as_bytes()).unwrap();
    sdr.check().unwrap();
    assert_eq!(sdr.len().unwrap(), 1);
    assert_eq!(sdr.len_bytes().unwrap(), 12);
    let count = 100;  // TODO: make this >1000 when things are faster
    for _ in 0..count {
        sdr.append(&[1,2,3,4,5]).unwrap();
    }
    sdr.check().unwrap();
    assert_eq!(sdr.len().unwrap(), 1+count);
    assert_eq!(sdr.len_bytes().unwrap(), 12 + (count*5));
}
