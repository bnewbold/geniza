// Free Software under GPL-3.0, see LICENSE
// Copyright 2017 Bryan Newbold

extern crate clap;
extern crate env_logger;
#[macro_use]
extern crate error_chain;
extern crate geniza;

// TODO: more careful import
use geniza::*;
use std::path::Path;
use clap::{App, Arg, SubCommand};

fn run() -> Result<()> {
    env_logger::init().unwrap();

    let matches = App::new("geniza-drive")
        .version(env!("CARGO_PKG_VERSION"))
        // TODO: dat-dir for all commands up here, and have a default vaule ('./dat')
        .arg(Arg::with_name("dat-dir")
            .short("d")
            .long("dat-dir")
            .value_name("PATH")
            .help("dat drive directory")
            .default_value(".dat") // TODO: needs to be default_value_os?
            .takes_value(true))
        .subcommand(
            SubCommand::with_name("init")
                .about("Creates a blank drive")
        )
        .subcommand(
            SubCommand::with_name("ls")
                .about("Lists current files in this dat")
        )
        .subcommand(
            SubCommand::with_name("cat")
                .about("Prints a file (as a string) to stdout")
                .arg_from_usage("<FILE> 'file to add'")
        )
        .subcommand(
            SubCommand::with_name("import-file")
                .about("Adds an indivudal file to the dat")
                .arg_from_usage("<FILE> 'file to add'")
                .arg_from_usage("--target <path> 'path to import the file to (if not top level)'")
        )
        .subcommand(
            SubCommand::with_name("log")
                .about("History of additions/deletions from this dat")
        )
        .subcommand(
            SubCommand::with_name("verify")
                .about("Checks signatures et al")
        )
        .subcommand(
            SubCommand::with_name("dump-entries")
                .about("Dump all entries in a debug-friendly format")
        )
        .get_matches();

    let dir = Path::new(matches.value_of("dat-dir").unwrap());
    match matches.subcommand() {
        ("init", Some(_subm)) => {
            let drive = DatDrive::create(dir)?;
            // TODO: print public key in hex
            println!("Done!");
        }
        ("ls", Some(_subm)) => {
            let mut drive = DatDrive::open(dir, false)?;
            for entry in drive.read_dir_recursive("/") {
                let entry = entry?;
                println!("{}", entry.path.display());
            }
        }
        ("import-file", Some(subm)) => {
            let path = Path::new(subm.value_of("FILE").unwrap());
            let mut drive = DatDrive::open(dir, true)?;
            let fpath = match subm.value_of("target") {
                None => Path::new("/").join(path.file_name().unwrap()),
                Some(p) => Path::new("/").join(p)
            };
            drive.import_file(&path, &fpath)?;

        }
        ("cat", Some(subm)) => {
            let path = Path::new(subm.value_of("FILE").unwrap());
            let mut drive = DatDrive::open(dir, true)?;
            let data = drive.read_file_bytes(&path)?;
            // TODO: just write to stdout
            let s = String::from_utf8(data).unwrap();
            println!("{}", s);

        }
        ("log", Some(_subm)) => {
            let mut drive = DatDrive::open(dir, false)?;
            for entry in drive.history(0) {
                let entry = entry?;
                if let Some(stat) = entry.stat {
                    if stat.get_blocks() == 0 {
                        println!("{}\t[chg]  {}",
                            entry.index, entry.path.display());
                    } else {
                        println!("{}\t[put]  {}\t{} bytes ({} blocks)",
                            entry.index, entry.path.display(), stat.get_size(), stat.get_blocks());
                    }
                } else {
                    println!("{}\t[del]  {}",
                        entry.index, entry.path.display());
                }
            }
        }
        ("verify", Some(_subm)) => {
            let mut drive = DatDrive::open(dir, false)?;
            println!("{:?}", drive.verify());
        }
        ("dump-entries", Some(_subm)) => {
            let mut drive = DatDrive::open(dir, false)?;
            for entry in drive.history(0) {
                let entry = entry?;
                println!("{}\tpath: {}",
                    entry.index, entry.path.display());
                println!("\tchildren: {:?}",
                    entry.children);
                if let Some(_) = entry.stat {
                    println!("\tstat: Some (add/change)");
                } else {
                    println!("\tstat: None (delete)");
                }
            }
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
