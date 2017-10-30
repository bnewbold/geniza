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
            SubCommand::with_name("ls")
                .about("Lists current files in this dat")
        )
        .subcommand(
            SubCommand::with_name("log")
                .about("History of additions/deletions from this dat")
        )
        .get_matches();

/*
    mode: ::std::option::Option<u32>,
    uid: ::std::option::Option<u32>,
    gid: ::std::option::Option<u32>,
    size: ::std::option::Option<u64>,
    blocks: ::std::option::Option<u64>,
    offset: ::std::option::Option<u64>,
    byteOffset: ::std::option::Option<u64>,
    mtime: ::std::option::Option<u64>,
    ctime: ::std::option::Option<u64>,
*/

    let dir = Path::new(matches.value_of("dat-dir").unwrap());
    match matches.subcommand() {
        ("ls", Some(_subm)) => {
            let mut drive = DatDrive::open(dir, false)?;
            for entry in drive.read_dir_recursive("/") {
                let entry = entry?;
                println!("{}", entry.path.display());
            }
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
        _ => {
            println!("Missing or unimplemented command!");
            println!("{}", matches.usage());
            ::std::process::exit(-1);
        }
    }
    Ok(())
}

quick_main!(run);
