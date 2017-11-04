
use std::io::Read;
use std::path::{Path, PathBuf};
use protobuf::Message;
use protobuf::parse_from_bytes;
use integer_encoding::VarInt;

use errors::*;
use sleep_register::*;
use metadata_msgs::{Index, Stat, Node};

/// "Sort of" follows rust std::fs API for file system access.
pub struct DatDrive {
    metadata: SleepDirRegister,
    content: SleepDirRegister,
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

fn decode_children(raw: &[u8]) -> Result<Vec<Vec<u64>>> {
    let mut children = vec![];
    let mut offset = 0; // byte offset that we have read up to
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
        children.push(sub);
    }
    trace!("decoded children: {:?}", children);
    Ok(children)
}

impl<'a> DatDrive {

    fn entry_count(&mut self) -> Result<u64> {
        Ok(self.metadata.len()? - 1)
    }

    fn get_dir_entry(&mut self, entry_index: u64) -> Result<DriveEntry> {
        trace!("fetching drive entry {} (of {})", entry_index, self.entry_count()?);
        let data = self.metadata.get_data_entry(entry_index+1)?;
        let node = parse_from_bytes::<Node>(&data)?;
        let stat = match node.has_value() {
            true => Some(parse_from_bytes::<Stat>(&node.get_value())?),
            false => None,
        };

        let children = decode_children(node.get_paths())?;

        Ok(DriveEntry {
            index: entry_index,
            path: PathBuf::from(node.get_name()),
            stat: stat,
            children,
        })
    }

    fn get_nearest<P: AsRef<Path>>(&mut self, _path: P) -> Result<DriveEntry> {
        // 0. if register is empty, bail out early
        let len = self.entry_count()?;
        if len <= 0 {
            bail!("Expected at least one entry, but drive is empty")
        }

        // 1. get most recent entry (tail of register)
        return self.get_dir_entry(len-2);

        // XXX: unimplemented!()
    }

    pub fn history<'b>(&'b mut self, start: u64) -> DriveHistory<'b> {
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

    pub fn file_metadata<P: AsRef<Path>>(&mut self, _path: P) -> Result<Stat> {
        unimplemented!()
    }

    pub fn create_file_bytes<P: AsRef<Path>>(&mut self, _path: P, _stat: &Stat, _data: &[u8]) -> Result<()> {
        unimplemented!()
    }

    pub fn create_file<P: AsRef<Path>, R: Read>(&mut self, _path: P, _stat: &Stat, _source: R) -> Result<()> {
        unimplemented!()
    }

    /// Copies Stat metadata and all content from a file in the "real" filesystem into the
    /// DatDrive.
    pub fn import_file<P: AsRef<Path>, Q: AsRef<Path>>(&mut self, _source: P, _dest: Q) -> Result<()> {
        unimplemented!()
    }

    /// Copies a file from the drive to the "real" filesystem, preserving Stat metadata.
    pub fn export_file<P: AsRef<Path>, Q: AsRef<Path>>(&mut self, _source: P, _dest: Q) -> Result<()> {
        unimplemented!()
    }

    pub fn read_file_bytes<P: AsRef<Path>, R: Read>(&mut self, path: P) -> Result<Vec<u8>> {
        let de = self.broken_find_file(path.as_ref())?;
        if let Some(entry) = de {
            // TODO: read and concatonate chunks
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

    pub fn verify(&mut self) -> Result<()> {
        self.metadata.verify()?;
        self.content.verify()?;
        Ok(())
    }

/* Possible future helper functions to be even more like std::fs
    pub fn rename<P: AsRef<Path>, Q: AsRef<Path>>(&mut self, from: P, to: Q) -> Result<()>
    pub fn copy<P: AsRef<Path>, Q: AsRef<Path>>(&mut self, from: P, to: Q) -> Result<()>
    pub fn remove_file<P: AsRef<Path>>(&mut self, path: P) -> Result<()>
    pub fn remove_dir_all<P: AsRef<Path>>(&mut self, path: P) -> Result<()>
*/

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
    // XXX: assert_eq!(dd.read_dir("/").count(), 2);
    // XXX: assert_eq!(dd.read_dir_recursive("/").count(), 6);
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

// TODO: unpack Node into a pub struct
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
        if self.current >= self.drive.entry_count().unwrap() {
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

    // Entries to recurse over
    entries: Vec<u64>,
}

impl<'a> ReadDriveDir<'a> {
    fn init<P: AsRef<Path>>(drive: &mut DatDrive, path: P, recursive: bool) -> Result<ReadDriveDir> {

        let entries = if drive.entry_count()? == 0 {
            vec![]
        } else {
            let nearest = drive.get_nearest(path)?;
            // TODO: starting from the last data entry, recurse up to nearest directory, then recurse
            // down to base path
            let mut entries = vec![];
            /* XXX:
            if nearest.stat.is_some() {
                // XXX: mapping fixer
                entries.push(nearest.index - 1);
            }
            */
            // XXX: flatten entries, not really the right thing to do
            for mut sub in nearest.children {
                entries.append(&mut sub);
            }
            entries
        };
        Ok(ReadDriveDir {
            drive,
            recursive,
            entries: entries,
        })
    }
}

impl<'a> Iterator for ReadDriveDir<'a> {
    type Item = Result<DriveEntry>;
    fn next(&mut self) -> Option<Result<DriveEntry>> {
        // TODO: actually recurse
        match self.entries.pop() {
            None => None,
            // XXX: +1 here is to skip the initial header
            Some(this_index) => Some(self.drive.get_dir_entry(this_index))
        }
    }
}

