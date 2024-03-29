
use std::io::{Read, Write, BufReader};
use std::path::{Path, PathBuf};
use std::os::unix::fs::{MetadataExt, OpenOptionsExt};
use std::fs::{File, OpenOptions, read_dir, create_dir_all};
use std::cmp::min;
use std::ffi::OsStr;
use protobuf::Message;
use protobuf::parse_from_bytes;
use integer_encoding::VarInt;

use errors::*;
use sleep_register::*;
use metadata_msgs::{Index, Stat, Node};

/// "Sort of" follows rust std::fs API for file system access.
pub struct DatDrive {
    pub metadata: SleepDirRegister,
    pub content: SleepDirRegister,
}

impl DatDrive {

    /// Instantiates a drive in the given directory. Path should be the complete path (eg, ending
    /// in '/.dat/'), not an enclosing directory containing files.
    pub fn create<P: AsRef<Path>>(path: P) -> Result<DatDrive> {
        let mut metadata = SleepDirRegister::create(path.as_ref(), "metadata")?;
        let content = SleepDirRegister::create(path.as_ref(), "content")?;
        // Calculate content discovery key and write as Index entry in metadata register
        let dk = metadata.discovery_key();
        let mut index = Index::new();
        index.set_field_type("hyperdrive".into());
        index.set_content(dk);
        metadata.append(&index.write_to_bytes()?)?;
        Ok(DatDrive {
            metadata,
            content,
        })
    }

    /// Path should be the complete path (eg, ending in '/.dat/'), not an enclosing directory
    /// containing files.
    pub fn open<P: AsRef<Path>>(path: P, writable: bool) -> Result<DatDrive> {
        let metadata = SleepDirRegister::open(path.as_ref(), "metadata", writable)?;
        if metadata.len()? == 0 {
            bail!("Expected at least one entry (Index) in metadata register");
        }
        let content = SleepDirRegister::open(path.as_ref(), "content", writable)?;
        Ok(DatDrive {
            metadata,
            content,
        })
    }
}

/// Inflates a binary-encoded child index table. `current` is the entry index number that this
/// child index is associated with.
fn decode_children(raw: &[u8], current: u64) -> Result<Vec<Vec<u64>>> {
    let mut children = vec![];
    let mut offset = 0; // byte offset that we have read up to
    if raw.len() < 1 {
        bail!("Expected (binary-encoded) children to have len>=1");
    }
    let (header, inc): (u64, usize) = VarInt::decode_var(&raw[offset..]);
    offset += inc;
    let append_current = (header & 0x01) == 0x01;
    trace!("append_current: {} header: {}", append_current, header);
    while offset < raw.len() {
        trace!("offset={} len={}", offset, raw.len());
        let mut sub = vec![];
        // decode_var() returns a varint value and the bytes read
        let (sub_len, inc): (u64, usize) = VarInt::decode_var(&raw[offset..]);
        //trace!("sub_len={} inc={}", sub_len, inc);
        trace!("VARINT: {}", sub_len);
        offset += inc;
        let mut run = 0;
        for _ in 0..sub_len {
            let (var, inc): (u64, usize) = VarInt::decode_var(&raw[offset..]);
            trace!("VARINT: {}", var);
            run += var;
            offset += inc;
            sub.push(run);
        }
        if append_current {
            sub.push(current);
        }
        children.push(sub);
    }
    trace!("decoded children: {:?}", children);
    Ok(children)
}

/// Binary encodes a child index table. `current` is the entry index number that this child index
/// is associated with.
fn encode_children(children: &Vec<Vec<u64>>, current: u64) -> Result<Vec<u8>> {
    // Use of encode_var_vec() instead of encode_var() here is sort of lazy

    let mut buf = vec![];

    // Check if the "all arrays end with current index" flag is set
    let mut current_appended = true;
    for sub in children {
        if sub.len() == 0 || sub[sub.len() - 1] != current {
            current_appended = false;
            break;
        }
    }

    let header: u64 = if current_appended { 0x01 } else { 0x00 };
    buf.append(&mut header.encode_var_vec());

    for subvec in children {
        let mut subvec = subvec.clone();
        if current_appended {
            subvec.pop();
        }
        buf.append(&mut subvec.len().encode_var_vec());
        subvec.sort_unstable();
        let mut last = 0;
        for val in subvec {
            let run: u64 = val - last;
            buf.append(&mut run.encode_var_vec());
            last = val;
        }
    }
    Ok(buf)
}

/// Returns the count of path components 
///
/// NB: does not distinguish between paths ending in a directory ("/thing/") and those ending with
/// a file ("/thing")
fn longest_common_prefix<P: AsRef<Path>, Q: AsRef<Path>>(a: P, b: Q) -> u64 {

    let a: Vec<&OsStr> = a.as_ref().iter().collect();
    let b: Vec<&OsStr> = b.as_ref().iter().collect();

    let mut common = 0;
    for i in 0..min(a.len(), b.len()) {
        if a[i] != b[i] {
            break;
        }
        common = i + 1; // +1 for count
    }
    common as u64
}

#[test]
fn test_longest_common_prefix() {

    assert_eq!(longest_common_prefix(
        "a",
        "b"),
        0);
    assert_eq!(longest_common_prefix(
        "a",
        "a"),
        1);
    assert_eq!(longest_common_prefix(
        "/hello/world",
        "/hello/goodbye"),
        2);
    assert_eq!(longest_common_prefix(
        "/hello/my/friend/",
        "/hello/my/friend"),
        4);
    assert_eq!(longest_common_prefix(
        "/hello/goodbye",
        "/hello/my/friend"),
        2);
    assert_eq!(longest_common_prefix(
        "/ein/zwei",
        "/one/two/three"),
        1);
}

impl<'a> DatDrive {

    /// Returns number of drive metadata entries (not including the first entry, which is the
    /// content register public key)
    pub fn entry_count(&mut self) -> Result<u64> {
        Ok(self.metadata.len()? - 1)
    }

    /// Entry index is counted by drive entries (not including the first register entry, which is
    /// the content register public key)
    fn get_dir_entry(&mut self, entry_index: u64) -> Result<DriveEntry> {
        if entry_index == 0 {
            bail!("First entry in a drive is pubkey metadata, not a DriveEntry");
        }
        trace!("fetching drive entry {} (of {})", entry_index, self.entry_count()?);
        let data = self.metadata.get_data_entry(entry_index)?;
        let node = parse_from_bytes::<Node>(&data)?;
        let stat = match node.has_value() {
            true => Some(parse_from_bytes::<Stat>(&node.get_value())?),
            false => None,
        };

        let children = decode_children(node.get_paths(), entry_index)?;

        Ok(DriveEntry {
            index: entry_index,
            path: PathBuf::from(node.get_name()),
            stat: stat,
            children,
        })
    }

    /// Returns the drive entry which: 1) has the longest common path prefix to the given path 2)
    /// is the most recent
    fn get_nearest<P: AsRef<Path>>(&mut self, path: P) -> Result<Option<DriveEntry>> {

        let path = path.as_ref();
        trace!("get_nearest: {}", path.display());

        // If register is empty, bail early
        let reg_len = self.entry_count()?;
        if reg_len == 0 {
            return Ok(None);
        }

        // 1. get most recent entry (tail of register)
        let mut current = self.get_dir_entry(reg_len)?; // "len + 1 - 1"

        // 1.1 If either path didn't start with '/', bail early
        if !path.has_root() {
            bail!("Passed a path with no root prefix: {}", path.display());
        }
        if !current.path.has_root() {
            bail!("Passed a path with no root prefix: {}", current.path.display());
        }
        // If paths match, return current
        if current.path.starts_with(path) {
            return Ok(Some(current));
        }

        // 2. find longest common prefix; take all entries from that level
        let mut common_components = longest_common_prefix(path, &current.path);
        assert!(common_components >= 1);
        if current.children.len() == 0 {
            // Empty drive tree
            return Ok(None);
        }
        let mut entries = current.children[(common_components-1) as usize].clone();

        // 3. for each of those entries (going in recent-first order):
        //      - if a full prefix match, return entry
        //      - if a closer (longer) match, clear entries and recurse
        //      - if not longer match, continue
        //      - if end of list, return current entry
        'outer: loop {
            trace!("entries loop: {:?}", entries);
            if entries.len() == 0 {
                break;
            }
            'inner: for e in entries.clone().iter().rev() {
                let entry = self.get_dir_entry(*e)?;
                if entry.path.starts_with(path) {
                    return Ok(Some(entry));
                }
                let this_common = longest_common_prefix(path, &entry.path);
                if this_common > common_components {
                    common_components = this_common;
                    current = entry;
                    entries = current.children[(common_components-1) as usize].clone();
                    continue 'outer;
                } else {
                    continue 'inner;
                }
            }
            break 'outer;
        }
        Ok(Some(current))
    }

    fn get_file_entry(&mut self, path: &Path) -> Result<Option<DriveEntry>> {
        match self.get_nearest(path)? {
            None => return Ok(None),
            Some(de) => {
                if de.path != path || !de.stat.is_some() {
                    return Ok(None);
                } else {
                    return Ok(Some(de));
               }
            }
        }
    }

    /// 'start' is the drive metadata register entry index. Zero is skipped automatically.
    pub fn history<'b>(&'b mut self, start: u64) -> DriveHistory<'b> {
        // skip pubkey entry
        let start = if start == 0 { 1 } else { start };
        DriveHistory {
            drive: self,
            current: start,
        }
    }

    pub fn read_dir_recursive<'b, P: AsRef<Path>>(&'b mut self, path: P) -> ReadDriveDir<'b> {
        // TODO: pass a single error if there is an error?
        ReadDriveDir::init(self, path, true).unwrap()
    }

    pub fn read_dir<'b, P: AsRef<Path>>(&'b mut self, path: P) -> ReadDriveDir<'b> {
        // TODO: pass a single error if there is an error?
        ReadDriveDir::init(self, path, false).unwrap()
    }

    pub fn file_metadata<P: AsRef<Path>>(&mut self, path: P) -> Result<Stat> {
        let de = self.get_file_entry(path.as_ref())?;
        if let Some(entry) = de {
            // if entry.stat was None, we'd have gotten None back
            return Ok(entry.stat.unwrap());
        } else {
            bail!("Couldn't find path: {}", path.as_ref().display());
        }
    }

    /// On success, returns version number including the added data.
    pub fn add_file_bytes<P: AsRef<Path>>(&mut self, path: P, stat: &mut Stat, data: &[u8]) -> Result<u64> {
        self.add_file(path, stat, data)
    }

    /// On success, returns version number including the added file.
    pub fn add_file<P: AsRef<Path>, R: Read>(&mut self, path: P, stat: &mut Stat, mut source: R) -> Result<u64> {
        // TODO: canonicalize path
        // TODO: check if file already exists
        let mut total_size: u64 = 0;
        let mut data_entries: u64 = 0;
        let mut buf = [0; 65536];
        let data_offset = self.content.len()?;
        let data_byte_offset = self.content.len_bytes()?;

        loop {
            // 1. read chunk
            let rlen = source.read(&mut buf)?;
            if rlen == 0 {
                break;
            }
            // 2. append chunk to data register
            self.content.append(&buf[0..rlen])?;

            // 3. increment metadata size
            total_size += rlen as u64;
            data_entries += 1;
        }

        // 4. write metadata
        stat.set_size(total_size as u64);
        stat.set_blocks(data_entries);
        stat.set_offset(data_offset);
        stat.set_byteOffset(data_byte_offset);
        return self.append_metadata_entry(&path, Some(&stat), None);
    }

    /// If this metadata entry represents a change (overwriting a previous entry), then `remove`
    /// should be set to the old index.
    /// If this entry is a deletion/removal, `remove` should be set and `stat` should be None.
    fn append_metadata_entry<P: AsRef<Path>>(&mut self, path: P, stat: Option<&Stat>, remove: Option<u64>) -> Result<u64> {
        let index = self.entry_count()? + 1;
        let path = path.as_ref();
        let mut children = self.new_child_index(&path,
            if stat.is_some() { Some(index) } else { None })?;
        if remove.is_some() {
            // This is a removal; delete both current and old from all children
            // (Vec.remove_item() is still nightly)
            for dir_level in 0..children.len() {
                children[dir_level].retain(
                    |&x| x != remove.unwrap());
            }

            // Cleanup by removing trailing empty dir levels
            children.retain(|ref x| x.len() > 0);
        }
        let children = encode_children(&children, index)?;
        let mut node = Node::new();
        node.set_name(path.to_string_lossy().into_owned());
        if let Some(val) = stat {
            node.set_value(val.write_to_bytes()?);
        }
        node.set_paths(children);
        self.metadata.append(&node.write_to_bytes()?)?;
        return Ok(index);
    }

    /// If index is included, it will be inserted at every level, replacing the previous ("nearest")
    /// pointer at that path
    fn new_child_index<P: AsRef<Path>>(&mut self, path: P, index: Option<u64>) -> Result<Vec<Vec<u64>>> {

        let path = path.as_ref();
        let path_len = path.iter().count() as u64;
        let mut depth: u64 = 0;
        let mut children: Vec<Vec<u64>> = vec![];
        while depth < path_len {
            // 1. get nearest at every level of path (starting at "/")
            let prefix: Vec<String> = path.iter().take(depth as usize).map(|s| s.to_string_lossy().into_owned()).collect();
            let prefix = Path::new("/").join(prefix.join("/"));
            let nearest = match self.get_nearest(prefix)? {
                None => {
                    if let Some(i) = index {
                        children.push(vec![i]);
                    }
                    depth += 1;
                    continue;
                },
                Some(de) => de,
            };
            // 2. consider up to common components
            let common = longest_common_prefix(path, &nearest.path);
            // (assuming we had any new common components; if not, fill in with outself)
            if common <= depth {
                if let Some(i) = index {
                    for _ in depth..path_len {
                        children.push(vec![i]);
                    }
                }
                break;
            }
            for i in depth..common {
                let mut component_entries = nearest.children[i as usize].clone();
                // 3. add this entry to each component...
                if let Some(idx) = index {
                    if i + 1 < common {
                        // ... while removing previous ("nearest") path component in all but last
                        // directory
                        component_entries.retain(|&e| e != nearest.index);
                    }
                    component_entries.push(idx);
                }
                children.push(component_entries);
            }
            // 4. loop for remaining components
            assert!(common > depth);
            depth = common;
        }
        Ok(children)
    }

    /// Copies Stat metadata and all content from a file in the "real" filesystem into the
    /// DatDrive.
    /// On success, returns version number including the added file.
    pub fn import_file<P: AsRef<Path>, Q: AsRef<Path>>(&mut self, source: P, dest: Q) -> Result<u64> {
        info!("importing file: '{:?}' as '{:?}'", source.as_ref(), dest.as_ref());
        let in_file = File::open(source)?;
        let in_metadata = in_file.metadata()?;
        let in_file = BufReader::new(in_file);
        let mut stat = Stat::new();
        stat.set_mode(in_metadata.mode());
        stat.set_uid(in_metadata.uid());
        stat.set_gid(in_metadata.gid());
        stat.set_size(in_metadata.size());
        stat.set_mtime(in_metadata.mtime() as u64);
        stat.set_ctime(in_metadata.ctime() as u64);
        self.add_file(dest, &mut stat, in_file)
    }

    /// Copies a file from the drive to the "real" filesystem, preserving Stat metadata.
    /// 'dest' must be a file, not a directory.
    pub fn export_file<P: AsRef<Path>, Q: AsRef<Path>>(&mut self, source: P, dest: Q) -> Result<()> {
        info!("exporting file: '{:?}' as '{:?}'", source.as_ref(), dest.as_ref());
        let source = source.as_ref();
        let dest = dest.as_ref();
        let de = self.get_file_entry(source)?;
        if let Some(entry) = de {
            let stat = entry.stat.unwrap();
            // create enclosing directory if it doesn't exist
            // TODO: this could be more efficient as an "attempt, create dir if not exists"
            let dir = dest.parent().unwrap();
            if !dir.is_dir() {
                create_dir_all(dir)?;
            }
            let mut out_file = OpenOptions::new()
                .create_new(true)
                .write(true)
                .mode(stat.get_mode())
                .open(dest)?;
            let offset = stat.get_offset();
            let blocks = stat.get_blocks();
            for i in offset..(offset+blocks) {
                let chunk = self.content.get_data_entry(i)?;
                out_file.write_all(&chunk)?;
            }
            // TODO: more outfile metadata (uid, guid, etc)
        } else {
            bail!("Couldn't find path: {}", source.display());
        }

        Ok(())
    }

    /// Copies Stat metadata and all content from a directory (recursively) from the "real"
    /// filesystem into the DatDrive.
    /// On success, returns version number including all the added files.
    pub fn import_dir_all<P: AsRef<Path>, Q: AsRef<Path>>(&mut self, source: P, dest: Q) -> Result<u64> {
        let source = source.as_ref();
        let dest = dest.as_ref();
        // TODO: check that dest doesn't exist (or is directory)
        let nearest = self.get_nearest(dest)?;
        if let Some(nearest) = nearest {
            if nearest.path == dest {
                bail!("destination already exists (as a file)");
            }
        }
        let mut ret = self.entry_count()?;
        if source.is_dir() {
            for entry in read_dir(source)? {
                let entry = entry?;
                let path = entry.path();
                let fname = path.file_name().unwrap().to_owned();
                if fname.to_str() == Some(".dat") {
                    // Don't import yourself!
                    continue
                }
                if path.is_dir() {
                    ret = self.import_dir_all(path, dest.join(fname))?;
                } else {
                    ret = self.import_file(path, dest.join(fname))?;
                }
            }
        } else {
            bail!("Source path wasn't a directory");
        }
        Ok(ret)
    }

    /// Copies a full directory from the drive to the "real" filesystem, preserving Stat metadata.
    pub fn export_dir<P: AsRef<Path>, Q: AsRef<Path>>(&mut self, source: P, dest: Q) -> Result<()> {
        let source = source.as_ref();
        let dest = dest.as_ref();
        // TODO: this collect() is inefficient; read doesn't mutate, so shouldn't really need a
        // mutable borrow
        let path_list: Vec<Result<DriveEntry>> = self.read_dir_recursive(source).collect();
        for entry in path_list {
            let path = entry?.path.to_owned();
            let out_path = dest.join(path.strip_prefix(source).unwrap());
            self.export_file(path, out_path)?;
        }
        Ok(())
    }

    pub fn read_file_bytes<P: AsRef<Path>>(&mut self, path: P) -> Result<Vec<u8>> {
        let de = self.get_file_entry(path.as_ref())?;
        if let Some(entry) = de {
            let stat = entry.stat.unwrap();
            let mut buf = vec![];
            let offset = stat.get_offset();
            let blocks = stat.get_blocks();
            for i in offset..(offset+blocks) {
                let mut chunk = self.content.get_data_entry(i)?;
                buf.append(&mut chunk);
            }
            return Ok(buf);
        } else {
            bail!("Couldn't find path: {}", path.as_ref().display());
        }
    }

    /// For now, simply verifies that both metadata and content registers are properly signed.
    pub fn verify(&mut self) -> Result<()> {
        self.metadata.verify()?;
        self.content.verify()?;
        Ok(())
    }

    /// Returns version number containing completed removal on success.
    pub fn remove_file<P: AsRef<Path>>(&mut self, path: P) -> Result<u64> {
        let path = path.as_ref();
        let current = self.get_file_entry(path)?;
        if let Some(val) = current {
            return self.append_metadata_entry(&val.path, None, Some(val.index));
        } else {
            bail!("Tried to delete non-existant file: {}", path.display());
        }
    }

    /// Returns version number containing completed removal on success.
    /// Partial success (returning an error) leaves the drive in an undefined state.
    pub fn remove_dir_all<P: AsRef<Path>>(&mut self, path: P) -> Result<u64> {
        // Crude implementation:
        // 1. get list of all file paths
        let path = path.as_ref();
        let files: Vec<PathBuf> = self.read_dir_recursive(path).map(|de| de.unwrap().path).collect();

        // 2. remove each
        let mut last_version = 0;
        for f in files {
            last_version= self.remove_file(&f)?;
        }
        Ok(last_version)
    }

    /// Returns version number of completed action on success.
    pub fn copy_file<P: AsRef<Path>, Q: AsRef<Path>>(&mut self, from: P, to: Q) -> Result<u64> {
        let from = from.as_ref();
        let to = to.as_ref();
        if from == to {
            bail!("Can't copy from self to self: {}", from.display());
        }
        let prev = if let Some(thing) = self.get_file_entry(from)? {
            thing 
        } else {
            bail!("File not in drive: {}", from.display());
        };
        // This check might be defensive (can we ever receive a deletion from get_file_entry()?)
        let stat = if let Some(thing) = prev.stat {
            thing
        } else {
            bail!("'from' file was deleted");
        };
        return self.append_metadata_entry(&to, Some(&stat), None);
    }

    /// Returns version number containing rename action on success.
    pub fn rename<P: AsRef<Path>, Q: AsRef<Path>>(&mut self, from: P, to: Q) -> Result<u64> {
        // Crude implementation:
        // 1. copy file
        let from = from.as_ref();
        self.copy_file(from, to)?;

        // 2. delete the original
        self.remove_file(from)
    }
}

#[test]
fn test_dd_open() {

    let mut dd =
        DatDrive::open(Path::new("test-data/dat/simple/.dat/"), false).unwrap();

    // verified from dat log
    assert_eq!(dd.history(0).count(), 2);
    assert_eq!(dd.read_dir("/").count(), 1);
    assert_eq!(dd.read_dir_recursive("/").count(), 1);

    let mut dd =
        DatDrive::open(Path::new("test-data/dat/tree/.dat/"), false).unwrap();

    // verified from dat log
    assert_eq!(dd.history(0).count(), 8);
    assert_eq!(dd.read_dir("/").count(), 2);
    assert_eq!(dd.read_dir_recursive("/").count(), 6);

    let mut dd =
        DatDrive::open(Path::new("test-data/dat/alphabet/.dat/"), false).unwrap();

    // verified from dat log
    assert_eq!(dd.history(0).count(), 6);
    assert_eq!(dd.read_dir("/").count(), 6);
    assert_eq!(dd.read_dir_recursive("/").count(), 6);
}

#[test]
fn test_dd_get_nearest() {

    let mut dd =
        DatDrive::open(Path::new("test-data/dat/tree/.dat/"), false).unwrap();

    assert!(dd.get_nearest("asdf").is_err());
    assert_eq!(dd.get_nearest("/NonExistant").unwrap().unwrap().index, 8);
    assert_eq!(dd.get_nearest("/").unwrap().unwrap().index, 8);
    assert_eq!(dd.get_nearest("/Fungi/Basidiomycota").unwrap().unwrap().index, 6);
    assert_eq!(dd.get_nearest("/datapackage.json").unwrap().unwrap().index, 8);
    assert_eq!(dd.get_nearest("/README.md").unwrap().unwrap().index, 1);
}

#[test]
fn test_dd_create() {
    use tempdir::TempDir;
    let tmp_dir = TempDir::new("geniza-test").unwrap();
    let mut dd = DatDrive::create(tmp_dir.path()).unwrap();

    assert_eq!(dd.history(0).count(), 0);
    assert_eq!(dd.read_dir("/").count(), 0);
    assert_eq!(dd.read_dir_recursive("/").count(), 0);
}

#[cfg(test)]
fn make_test_stat() -> Stat {
    let mut stat = Stat::new();
    stat.set_mode(0o777);
    stat.set_uid(1000);
    stat.set_gid(1000);
    stat.set_size(0);
    stat.set_mtime(54321);
    stat.set_ctime(65432);
    stat
}

#[test]
fn test_dd_add() {
    use tempdir::TempDir;
    let tmp_dir = TempDir::new("geniza-test").unwrap();
    let mut dd = DatDrive::create(tmp_dir.path()).unwrap();

    let data = vec![7; 123];
    let mut stat = make_test_stat();
    stat.set_size(123);
    dd.add_file_bytes("/bytes.bin", &mut stat, &data).unwrap();
    assert_eq!(dd.history(0).count(), 1);
    assert_eq!(dd.read_dir("/").count(), 1);
    assert_eq!(dd.read_dir_recursive("/").count(), 1);
    assert_eq!(dd.content.len_bytes().unwrap(), 123);

    stat.set_size(65);
    dd.add_file("/bytes_read.bin", &mut stat, &data[0..65]).unwrap();
    assert_eq!(dd.history(0).count(), 2);
    assert_eq!(dd.read_dir("/").count(), 2);
    assert_eq!(dd.read_dir_recursive("/").count(), 2);
    assert_eq!(dd.content.len_bytes().unwrap(), 123+65);

    dd.add_file_bytes("/here/msg.txt", &mut stat, "hello world".as_bytes()).unwrap();
    dd.add_file_bytes("/there/msg.txt", &mut stat, "goodbye world".as_bytes()).unwrap();
}

#[test]
fn test_dd_readback() {
    use tempdir::TempDir;
    let tmp_dir = TempDir::new("geniza-test").unwrap();
    let mut dd = DatDrive::create(tmp_dir.path()).unwrap();

    let mut stat = make_test_stat();
    dd.add_file_bytes("/here/msg.txt", &mut stat, "hello world".as_bytes()).unwrap();
    let mut stat = make_test_stat();
    dd.add_file_bytes("/sub/other.txt", &mut stat, "goodbye".as_bytes()).unwrap();

    assert_eq!(&dd.read_file_bytes("/here/msg.txt").unwrap()[..],
               "hello world".as_bytes());
    assert_eq!(&dd.read_file_bytes("/sub/other.txt").unwrap()[..],
               "goodbye".as_bytes());
}

/* TODO: needs data in register, or support for reading from checkout
#[test]
fn test_dd_read_file_bytes() {

    let mut dd =
        DatDrive::open(Path::new("test-data/dat/alphabet/.dat/"), false).unwrap();

    assert_eq!("a".as_bytes(), &dd.read_file_bytes("/a").unwrap()[..]);
    assert_eq!("b".as_bytes(), &dd.read_file_bytes("/b").unwrap()[..]);
    assert_eq!("c".as_bytes(), &dd.read_file_bytes("/c").unwrap()[..]);
    assert_eq!("e".as_bytes(), &dd.read_file_bytes("/e").unwrap()[..]);
}
*/

#[test]
fn test_dd_import_file() {

    use tempdir::TempDir;
    let tmp_dir = TempDir::new("geniza-test").unwrap();
    let mut dd = DatDrive::create(tmp_dir.path()).unwrap();

    dd.import_file("test-data/dat/alphabet/a", "/a").unwrap();
    dd.import_file("test-data/dat/alphabet/b", "/subdir/b.txt").unwrap();

    assert_eq!(dd.history(0).count(), 2);
    assert_eq!(&dd.read_file_bytes("/a").unwrap()[..],
               "a".as_bytes());
    assert_eq!(&dd.read_file_bytes("/subdir/b.txt").unwrap()[..],
               "b".as_bytes());

    assert!(dd.import_file("/non-existant-file-path", "/z").is_err());
}

#[test]
fn test_dd_export_file() {

    use tempdir::TempDir;
    let tmp_dir = TempDir::new("geniza-test").unwrap();
    let mut dd = DatDrive::create(tmp_dir.path()).unwrap();

    dd.import_file("test-data/dat/alphabet/a", "/a").unwrap();

    dd.export_file("/a", tmp_dir.path().join("a.txt")).unwrap();
    assert!(dd.export_file("/z", tmp_dir.path().join("never-created")).is_err());
}

#[test]
fn test_dd_import_dir_all() {

    use tempdir::TempDir;
    use env_logger;
    env_logger::init().unwrap();
    let tmp_dir = TempDir::new("geniza-test").unwrap();
    let mut dd = DatDrive::create(tmp_dir.path()).unwrap();

    dd.import_dir_all("test-data/dat/tree/Animalia/", "/").unwrap();

    assert_eq!(dd.read_dir("/").count(), 0);
    assert_eq!(dd.read_dir_recursive("/").count(), 2);

    dd.import_file("test-data/dat/alphabet/a", "/a").unwrap();
    assert!(dd.import_dir_all("test-data/dat/tree/Animalia/", "/a/").is_err());

}

#[test]
fn test_dd_export_dir() {

    use tempdir::TempDir;
    //use env_logger;
    //env_logger::init().unwrap();
    let tmp_dir = TempDir::new("geniza-test").unwrap();
    let mut dd = DatDrive::create(tmp_dir.path()).unwrap();

    dd.import_dir_all("test-data/dat/tree/Animalia/", "/").unwrap();

    dd.export_dir("/", tmp_dir.path()).unwrap();
    dd.export_dir("/Chordata/Mammalia/Carnivora/Caniformia/", tmp_dir.path()).unwrap();
    //assert!(dd.export_dir("/Fruit/", tmp_dir.path()).is_err());
}

#[test]
fn test_dd_remove_file() {

    use tempdir::TempDir;
    let tmp_dir = TempDir::new("geniza-test").unwrap();
    let mut dd = DatDrive::create(tmp_dir.path()).unwrap();

    dd.import_file("test-data/dat/alphabet/a", "/a").unwrap();
    dd.import_file("test-data/dat/alphabet/b", "/b").unwrap();
    assert_eq!(dd.read_dir("/").count(), 2);

    dd.remove_file("/a").unwrap();
    assert_eq!(dd.read_dir_recursive("/").count(), 1);
    dd.remove_file("/b").unwrap();
    assert_eq!(dd.read_dir("/").count(), 0);
    assert!(&dd.read_file_bytes("/b").is_err());

    assert!(dd.remove_file("/a").is_err());
}

#[test]
fn test_dd_remove_dir_all() {

    use tempdir::TempDir;
    let tmp_dir = TempDir::new("geniza-test").unwrap();
    let mut dd = DatDrive::create(tmp_dir.path()).unwrap();

    // This is also a regression test for `ls`
    dd.import_file("test-data/dat/alphabet/a", "/a").unwrap();
    dd.import_file("test-data/dat/alphabet/b", "/sub/b").unwrap();
    dd.import_file("test-data/dat/alphabet/c", "/sub/c").unwrap();
    dd.import_file("test-data/dat/alphabet/d", "/sub/sub/d").unwrap();
    assert_eq!(dd.read_dir_recursive("/").count(), 4);

    dd.remove_dir_all("/sub").unwrap();
    assert_eq!(dd.read_dir_recursive("/").count(), 1);
    assert!(&dd.read_file_bytes("/sub/b").is_err());
}

#[test]
fn test_dd_copy_file() {

    use tempdir::TempDir;
    let tmp_dir = TempDir::new("geniza-test").unwrap();
    let mut dd = DatDrive::create(tmp_dir.path()).unwrap();

    dd.import_file("test-data/dat/alphabet/a", "/a").unwrap();
    dd.copy_file("/a", "/c").unwrap();
    assert_eq!(dd.history(0).count(), 2);
    assert!(&dd.read_file_bytes("/a").is_ok());
    assert!(&dd.read_file_bytes("/c").is_ok());
}

#[test]
fn test_dd_rename() {

    use tempdir::TempDir;
    let tmp_dir = TempDir::new("geniza-test").unwrap();
    let mut dd = DatDrive::create(tmp_dir.path()).unwrap();

    dd.import_file("test-data/dat/alphabet/a", "/a").unwrap();
    dd.rename("/a", "/c").unwrap();
    assert_eq!(dd.read_dir("/").count(), 1);
    assert!(&dd.read_file_bytes("/a").is_err());
}

#[derive(Debug)]
pub struct DriveEntry {
    pub index: u64,
    pub path: PathBuf,
    pub stat: Option<Stat>,
    pub children: Vec<Vec<u64>>,
}

/// Iterator over full drive history (file additions/deletions).
pub struct DriveHistory<'a> {
    drive: &'a mut DatDrive,
    current: u64,
}

impl<'a> Iterator for DriveHistory<'a> {
    type Item = Result<DriveEntry>;
    fn next(&mut self) -> Option<Result<DriveEntry>> {
        // pubkey increment-by-one logic here
        // TODO: unwrap. on error, return Some(err), then None?
        if self.current > self.drive.entry_count().unwrap() {
            return None;
        }
        let de = self.drive.get_dir_entry(self.current);
        self.current += 1;
        return Some(de);
    }
}

/// Iterator over drive file entries.
pub struct ReadDriveDir<'a> {
    drive: &'a mut DatDrive,
    recursive: bool,
    path: PathBuf,

    // Entries to iterate over. Tuple of (depth, entry_index), where depth is the path prefix count
    // where this entry was encountered and added to the list.
    entries: Vec<(u64, u64)>,
}

impl<'a> ReadDriveDir<'a> {
    fn init<P: AsRef<Path>>(drive: &mut DatDrive, path: P, recursive: bool) -> Result<ReadDriveDir> {

        let path = path.as_ref();

        // first entry is content pub key
        let entries = if drive.entry_count()? == 0 {
            vec![]
        } else {
            // start at the latest entry with the same path prefix
            match drive.get_nearest(path)? {
                Some(nearest) => {
                    if nearest.children.len() == 0 {
                        // Empty tree
                        vec![]
                    } else {
                        let common_components = longest_common_prefix(path, nearest.path);
                        let list = nearest.children[(common_components - 1) as usize].clone();
                        list.iter().map(|e| (common_components, *e)).collect()
                    }
                },
                None => vec![],
            }
        };
        Ok(ReadDriveDir {
            drive,
            path: path.to_path_buf(),
            recursive,
            entries: entries,
        })
    }
}

impl<'a> Iterator for ReadDriveDir<'a> {
    type Item = Result<DriveEntry>;

    fn next(&mut self) -> Option<Result<DriveEntry>> {
        debug!("ReadDriveDir: {:?}", self.entries);
        let (depth, entry) = match self.entries.pop() {
            None => { return None },
            Some((depth, this_index)) => (depth, self.drive.get_dir_entry(this_index))
        };
        let entry = match entry {
            Err(_) => return Some(entry),
            Ok(e) => e,
        };

        // defensive programming... shouldn't ever have entries that aren't children of path
        if !entry.path.starts_with(&self.path) {
            warn!("unexpected non-child path entry in ReadDriveDir iterator: {}",
                entry.path.display());
            return self.next();
        }

        if entry.path.iter().count() <= self.path.iter().count() + 1 {
            // direct child of the path; always return
            if entry.stat.is_some() {
                return Some(Ok(entry));
            } else {
                return self.next();
            }
        } else {
            // subdirectory entry; depends on recursion
            if !self.recursive {
                return self.next();
            } else {
                // if entry was added as a child of this depth, just return it...
                if entry.children.len() as u64 <= depth + 1 {
                    if entry.stat.is_some() {
                        return Some(Ok(entry));
                    } else {
                        return self.next();
                    }
                }
                // ... else add child path entries and recurse
                for subdir in (depth as usize)..entry.children.len() {
                    let mut new_children: Vec<(u64, u64)> = entry.children[subdir].iter()
                        .filter(|&e| (*e != entry.index || subdir == entry.children.len()))
                        .map(|&e| (subdir as u64 + 1, e))
                        .collect();
                    self.entries.append(&mut new_children);
                }
                if entry.stat.is_some() {
                    return Some(Ok(entry));
                } else {
                    return self.next();
                }
            }
        }
    }
}

