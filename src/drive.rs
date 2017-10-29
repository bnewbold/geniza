
use std::io::Read;
use std::path::{Path, PathBuf};
//XXX: use protobuf::Message;
use protobuf::parse_from_bytes;

use errors::*;
use sleep_register::*;
use metadata_msgs::{Stat, Node};

/// "Sort of" follows rust std::fs API for file system access.
pub struct DatDrive {
    metadata: SleepDirRegister,
    content: SleepDirRegister,
}

impl DatDrive {

    /// Instantiates a drive in the given directory. Path should be the complete path (eg, ending
    /// in '/.dat/'), not an enclosing directory containing files.
    pub fn create<P: AsRef<Path>>(path: P) -> Result<DatDrive> {
        // Calculate content discovery key and write as Index entry in metadata register
        unimplemented!()
    }

    /// Path should be the complete path (eg, ending in '/.dat/'), not an enclosing directory
    /// containing files.
    pub fn open<P: AsRef<Path>>(path: P, writable: bool) -> Result<DatDrive> {
        let mdrive = SleepDirRegister::open(path.as_ref(), "metadata", writable)?;
        if mdrive.len()? == 0 {
            bail!("Expected at least one entry (Index) in metadata register");
        }
        let cdrive = SleepDirRegister::open(path.as_ref(), "content", writable)?;
        Ok(DatDrive {
            metadata: mdrive,
            content: cdrive,
        })
    }
}

impl<'a> DatDrive {

    fn find_path(path: &Path) -> Result<u64> {
        unimplemented!()
    }

    pub fn history<'b>(&'b mut self, start: u64) -> DriveHistory<'b> {
        // Start must be at least 1; index 0 is the Index item
        let start = if start == 0 { 1 } else { start };
        DriveHistory {
            drive: self,
            current: start,
        }
    }

    pub fn read_dir_recursive<P: AsRef<Path>>(&mut self, path: P) -> ReadDriveDir<'a> {
        unimplemented!()
    }

    pub fn read_dir<P: AsRef<Path>>(&mut self, path: P) -> ReadDriveDir<'a> {
        unimplemented!()
    }

    pub fn file_metadata<P: AsRef<Path>>(&mut self, path: P) -> Result<Stat> {
        unimplemented!()
    }

    pub fn create_file_bytes<P: AsRef<Path>>(&mut self, path: P, stat: &Stat, data: &[u8]) -> Result<()> {
        unimplemented!()
    }

    pub fn create_file<P: AsRef<Path>, R: Read>(&mut self, path: P, stat: &Stat, source: R) -> Result<()> {
        unimplemented!()
    }

    /// Copies Stat metadata and all content from a file in the "real" filesystem into the
    /// DatDrive.
    pub fn import_file<P: AsRef<Path>, Q: AsRef<Path>>(&mut self, source: P, dest: Q) -> Result<()> {
        unimplemented!()
    }

    /// Copies a file from the drive to the "real" filesystem, preserving Stat metadata.
    pub fn export_file<P: AsRef<Path>, Q: AsRef<Path>>(&mut self, source: P, dest: Q) -> Result<()> {
        unimplemented!()
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
    assert_eq!(dd.history(1).count(), 2);
    assert_eq!(dd.read_dir("/").count(), 1);
    assert_eq!(dd.read_dir_recursive("/").count(), 1);
}

#[test]
fn test_dd_create() {
    use tempdir::TempDir;
    let tmp_dir = TempDir::new("geniza-test").unwrap();
    let mut dd = DatDrive::create(tmp_dir.path()).unwrap();

    assert_eq!(dd.history(1).count(), 0);
    assert_eq!(dd.read_dir("/").count(), 0);
    assert_eq!(dd.read_dir_recursive("/").count(), 0);
}

// TODO: unpack Node into a pub struct
#[derive(Debug)]
pub struct DriveEntry {
    pub index: u64,
    pub path: PathBuf,
    pub stat: Option<Stat>,
}

/// Iterator over full drive history (file additions/deletions).
pub struct DriveHistory<'a> {
    drive: &'a mut DatDrive,
    current: u64,
}

impl<'a> Iterator for DriveHistory<'a> {
    type Item = Result<DriveEntry>;
    fn next(&mut self) -> Option<Result<DriveEntry>> {
        if self.current >= self.drive.metadata.len().unwrap() {
            return None;
        }
        // TODO: handle Err, not unwrap
        let data = match self.drive.metadata.get_data_entry(self.current) {
            Err(e) => { return Some(Err(e)) },
            Ok(v) => v,
        };
        let node = match parse_from_bytes::<Node>(&data) {
            Err(e) => { return Some(Err(e.into())) },
            Ok(v) => v,
        };
        let stat = match node.has_value() {
            true => Some(parse_from_bytes::<Stat>(&node.get_value()).unwrap()),
            false => None,
        };
        let de = Ok(DriveEntry {
            index: self.current,
            path: PathBuf::from(node.get_name()),
            stat: stat,
        });
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
    fn init<P: AsRef<Path>>(drive: &mut DatDrive, path: P, recursive: bool) {
        unimplemented!();
        // TODO: starting from the last data entry, recurse up to nearest directory, then recurse
        // down to base path
        //ReadDriveDir {
        //    drive,
        //    recursive,
        //    entries: vec![],
        //}
    }
}

impl<'a> Iterator for ReadDriveDir<'a> {
    type Item = DriveEntry;
    fn next(&mut self) -> Option<DriveEntry> {
        unimplemented!();
    }
}

