
use std::io::{Read, Write};
use std::path::Path;
use protobuf::Message;
use protobuf::parse_from_bytes;

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
        unimplemented!()
    }

    /// Path should be the complete path (eg, ending in '/.dat/'), not an enclosing directory
    /// containing files.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<DatDrive> {
        unimplemented!()
    }
}

impl<'a> DatDrive {

    pub fn history(start: u64) -> DriveHistory<'a> {
        unimplemented!()
    }

    pub fn read_dir_recursive<P: AsRef<Path>>(path: P) -> ReadDriveDir<'a> {
        unimplemented!()
    }

    pub fn read_dir<P: AsRef<Path>>(path: P) -> ReadDriveDir<'a> {
        unimplemented!()
    }

    pub fn file_metadata<P: AsRef<Path>>(path: P) -> Result<Stat> {
        unimplemented!()
    }

    pub fn create_file_bytes<P: AsRef<Path>>(path: P, stat: &Stat, data: &[u8]) -> Result<()> {
        unimplemented!()
    }

    pub fn create_file<P: AsRef<Path>, R: Read>(path: P, stat: &Stat, source: R) -> Result<()> {
        unimplemented!()
    }

    /// Copies Stat metadata and all content from a file in the "real" filesystem into the
    /// DatDrive.
    pub fn import_file<P: AsRef<Path>, Q: AsRef<Path>>(source: P, dest: Q) -> Result<()> {
        unimplemented!()
    }

/* Possible future helper functions to be even more like std::fs
    pub fn rename<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> Result<()>
    pub fn copy<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> Result<()>
    pub fn remove_file<P: AsRef<Path>>(path: P) -> Result<()>
    pub fn remove_dir_all<P: AsRef<Path>>(path: P) -> Result<()>
*/

}

#[test]
fn test_dd_open() {

    let mut dd =
        DatDrive::open(Path::new("test-data/dat/simple/.dat/")).unwrap();

    // verified from dat log
    assert_eq!(dd.history().collect().len(), 2);
    assert_eq!(dd.read_dir().collect().len(), 1);
    assert_eq!(dd.read_dir_recurisve().collect().len(), 1);
}

#[test]
fn test_dd_create() {
    use tempdir::TempDir;
    let tmp_dir = TempDir::new("geniza-test").unwrap();
    let mut dd = DatDrive::create(tmp_dir.path()).unwrap();

    assert_eq!(dd.history().collect().len(), 0);
    assert_eq!(dd.read_dir().collect().len(), 0);
    assert_eq!(dd.read_dir_recurisve().collect().len(), 0);
}

pub struct DriveEntry {
    node: Node,
    index: u64,
}

pub struct DriveHistory<'a> {
    drive: &'a mut DatDrive,
}

/// Iterator 
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

impl<'a> Iterator for DriveHistory<'a> {
    type Item = DriveEntry;
    fn next(&mut self) -> Option<DriveEntry> {
        unimplemented!();
    }
}

impl<'a> Iterator for ReadDriveDir<'a> {
    type Item = DriveEntry;
    fn next(&mut self) -> Option<DriveEntry> {
        unimplemented!();
    }
}

