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
use clap::{App, SubCommand, Arg};

fn run() -> Result<()> {
    env_logger::init().unwrap();

    let matches = App::new("geniza")
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand(
            SubCommand::with_name("clone")
                .about("Finds and downloads a dat archive from the network into a given folder")
                .arg_from_usage("<address> 'dat address (public key) to fetch'"),
        )
        .subcommand(
            SubCommand::with_name("init")
                .about("Creates a data archive in the current directory")
        )
        .subcommand(
            SubCommand::with_name("checkout")
                .about("Copies (or overwrites) files from dat archive into current folder")
                .arg_from_usage("<path> 'relative path to checkout'"),
        )
        .subcommand(
            SubCommand::with_name("add")
                .about("Adds a path to the current dat archive")
                .arg_from_usage("<path> 'file to delete from dat archive'"),
        )
        .subcommand(
            SubCommand::with_name("rm")
                .about("Removes a path from the current dat archive, and from disk (danger!)")
                .arg_from_usage("<path> 'file to delete from dat archive'"),
        )
        .get_matches();


    match matches.subcommand() {
        ("clone", Some(subm)) => {
            let dat_key = subm.value_of("dat_key").unwrap();
            let key_bytes = parse_dat_address(&dat_key)?;
            unimplemented!();
            //let dir = Path::new(subm.value_of("dat-dir").unwrap());
            //let mut metadata = SleepDirRegister::create(&dir, "metadata")?;
            //node_simple_clone(host_port, &key_bytes, &mut metadata, false)?;
        }
        ("init", Some(subm)) => {
            unimplemented!();
        }
        ("checkout", Some(subm)) => {
            let path = Path::new(subm.value_of("path").unwrap());
            unimplemented!();
        }
        ("add", Some(subm)) => {
            let path = Path::new(subm.value_of("path").unwrap());
            unimplemented!();
        }
        ("rm", Some(subm)) => {
            let path = Path::new(subm.value_of("path").unwrap());
            unimplemented!();
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
