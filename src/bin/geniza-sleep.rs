// Free Software under GPL-3.0, see LICENSE
// Copyright 2017 Bryan Newbold

#[macro_use]
extern crate clap;
extern crate env_logger;
#[macro_use]
extern crate error_chain;
extern crate geniza;

// TODO: more careful import
use geniza::*;
use std::path::Path;
use clap::{App, SubCommand};

fn run() -> Result<()> {
    env_logger::init().unwrap();

    let matches = App::new("geniza-sleep")
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand(
            SubCommand::with_name("info")
                .about("Reads a SLEEP dir register and shows some basic metadata")
                .arg_from_usage("<DIR> 'directory containing files'")
                .arg_from_usage("<prefix> 'prefix for each data file'"),
        )
        .subcommand(
            SubCommand::with_name("create")
                .about("Creates an SLEEP directory register (with header)")
                .arg_from_usage("<DIR> 'directory containing files'")
                .arg_from_usage("<prefix> 'prefix for each data file'"),
        )
        .subcommand(
            SubCommand::with_name("chunk")
                .about("Dumps raw data for a single given chunk from a register (by index)")
                .arg_from_usage("<DIR> 'directory containing files'")
                .arg_from_usage("<prefix> 'prefix for each data file'")
                .arg_from_usage("<index> 'index of the data chunk to dump'"),
        )
        .subcommand(
            SubCommand::with_name("file-info")
                .about("Reads a single SLEEP file and shows some basic metadata")
                .arg_from_usage("<FILE> 'SLEEP file to read'"),
        )
        .subcommand(
            SubCommand::with_name("file-create")
                .about("Creates an empty single SLEEP file (with header)")
                .arg_from_usage("<FILE> 'SLEEP file to write (can't exist)'")
                .arg_from_usage("<magic> 'Magic word to use (eg, 0x5025700)'")
                .arg_from_usage("<entry_size> 'Size of each entry (bytes)'")
                .arg_from_usage("<algo_name> 'Name of algorithm (empty string for none)'"),
        )
        .subcommand(
            SubCommand::with_name("file-read-all")
                .about("Reads a single SLEEP file, iterates through all entries, prints raw bytes")
                .arg_from_usage("<FILE> 'SLEEP file to read'"),
        )
        .subcommand(
            SubCommand::with_name("file-chunk")
                .about("Dumps raw data for a single given chunk from a SLEEP file (by index)")
                .arg_from_usage("<FILE> 'SLEEP file to read'")
                .arg_from_usage("<index> 'index of the data chunk to dump'"),
        )
        .subcommand(
            SubCommand::with_name("write-example-alphabet")
                .about("Creates a content register in the given folder, with example data added")
                .arg_from_usage("<DIR> 'directory containing files'")
                .arg_from_usage("<prefix> 'prefix for each data file'")
        )
        .get_matches();


    match matches.subcommand() {
        ("info", Some(subm)) => {
            let dir = Path::new(subm.value_of("DIR").unwrap());
            let prefix = subm.value_of("prefix").unwrap();
            let mut sdr = SleepDirRegister::open(dir, prefix, false)?;
            //debug!(println!("{:?}", sdr));
            println!("Entry count: {}", sdr.len()?);
            println!("Total size (bytes): {}", sdr.len_bytes()?);
        }
        ("create", Some(subm)) => {
            let dir = Path::new(subm.value_of("DIR").unwrap());
            let prefix = subm.value_of("prefix").unwrap();
            SleepDirRegister::create(dir, prefix)?;
            println!("Done!");
        }
        ("chunk", Some(subm)) => {
            let dir = Path::new(subm.value_of("DIR").unwrap());
            let prefix = subm.value_of("prefix").unwrap();
            let index = value_t_or_exit!(subm, "index", u64);
            let mut sdr = SleepDirRegister::open(dir, prefix, false)?;
            //debug!(println!("{:?}", sdr));
            println!("{:?}", sdr.get_data_entry(index)?);
        }
        ("file-info", Some(subm)) => {
            let path = Path::new(subm.value_of("FILE").unwrap());
            let sf = SleepFile::open(path, false)?;
            //debug!(println!("{:?}", sf));
            println!("Magic: 0x{:X}", sf.get_magic());
            println!(
                "Algorithm: '{}'",
                sf.get_algorithm().or(Some("".to_string())).unwrap()
            );
            println!("Entry Size (bytes): {}", sf.get_entry_size());
            println!("Entry count: {}", sf.len()?);
        }
        ("file-create", Some(subm)) => {
            let path = Path::new(subm.value_of("FILE").unwrap());
            let algo_name = subm.value_of("algo_name").unwrap();
            let algo_name = if algo_name.len() == 0 {
                None
            } else {
                Some(algo_name.to_string())
            };
            SleepFile::create(
                path,
                value_t_or_exit!(subm, "magic", u32),
                value_t_or_exit!(subm, "entry_size", u16),
                algo_name,
            )?;
            println!("Done!");
        }
        ("file-read-all", Some(subm)) => {
            let path = Path::new(subm.value_of("FILE").unwrap());
            let mut sf = SleepFile::open(path, false)?;
            for i in 0..sf.len()? {
                println!("{}: {:?}", i, sf.read(i));
            }
        }
        ("file-chunk", Some(subm)) => {
            let path = Path::new(subm.value_of("FILE").unwrap());
            let index = value_t_or_exit!(subm, "index", u64);
            let mut sf = SleepFile::open(path, false)?;
            //debug!(println!("{:?}", sdr));
            println!("{:?}", sf.read(index)?);
        }
        ("write-example-alphabet", Some(subm)) => {
            let dir = Path::new(subm.value_of("DIR").unwrap());
            let prefix = subm.value_of("prefix").unwrap();
            let mut sdr = SleepDirRegister::create(dir, prefix)?;
            sdr.append(&[0x61; 1])?; // a
            sdr.append(&[0x62; 1])?; // b
            sdr.append(&[0x63; 1])?; // c
            sdr.append(&[0x64; 1])?; // d
            sdr.append(&[0x65; 1])?; // e
            sdr.append(&[0x66; 1])?; // f
            println!("Done!");
        }
        _ => {
            println!("Missing or unimplemented command!");
            println!("{}", matches.usage());
            ::std::process::exit(-1);
        }
    }
    Ok(())
}

quick_main!(run);
