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
use clap::{App, SubCommand};

fn run() -> Result<()> {
    env_logger::init().unwrap();

    let matches = App::new("geniza")
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand(
            SubCommand::with_name("clone")
                .about("Finds and downloads a dat archive from the network into a given folder")
                .arg_from_usage("<address> 'dat address (public key) to fetch'")
                .arg_from_usage("[dir] 'directory to clone into'")
                .arg_from_usage("--full 'pull and save complete history (not just latest version)'"),
        )
        .subcommand(
            SubCommand::with_name("init")
                .about("Creates a data archive in the current directory")
                .arg_from_usage("[dir] 'init somewhere other than current directory'"),
        )
        .subcommand(
            SubCommand::with_name("status")
                .about("Displays current status of archive and checkout")
        )
        .subcommand(
            SubCommand::with_name("log")
                .about("Displays version history of the archive")
        )
        .subcommand(
            SubCommand::with_name("checkout")
                .about("Copies (or overwrites) files from dat archive")
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
        .subcommand(
            SubCommand::with_name("ls")
                .about("Lists contents of the archive")
                .arg_from_usage("[path] 'path to display'")
                .arg_from_usage("--recursive 'show directory recursively'"),
        )
        .subcommand(
            SubCommand::with_name("seed")
                .about("Uploads indefinately to any peer")
        )
        .subcommand(
            SubCommand::with_name("pull")
                .about("Pulls highest known version from all possible peers")
                .arg_from_usage("--forever 'continues to search for updates forever'"),
        )
        .get_matches();

    match matches.subcommand() {
        ("clone", Some(subm)) => {
            let dat_key = subm.value_of("dat_key").unwrap();
            let _key_bytes = parse_dat_address(&dat_key)?;
            unimplemented!();
            //let dir = Path::new(subm.value_of("dat-dir").unwrap());
            //let mut metadata = SleepDirRegister::create(&dir, "metadata")?;
            //node_simple_clone(host_port, &key_bytes, &mut metadata, false)?;
        }
        ("init", Some(subm)) => {
            let _dir = Path::new(subm.value_of("dir").unwrap());
            unimplemented!();
        }
        ("status", Some(_subm)) => {
            unimplemented!();
        }
        ("log", Some(_subm)) => {
            unimplemented!();
        }
        ("checkout", Some(subm)) => {
            let _path = Path::new(subm.value_of("path").unwrap());
            unimplemented!();
        }
        ("add", Some(subm)) => {
            let _path = Path::new(subm.value_of("path").unwrap());
            unimplemented!();
        }
        ("rm", Some(subm)) => {
            let _path = Path::new(subm.value_of("path").unwrap());
            unimplemented!();
        }
        ("ls", Some(subm)) => {
            let _path = Path::new(subm.value_of("path").unwrap());
            unimplemented!();
        }
        ("seed", Some(subm)) => {
            let _path = Path::new(subm.value_of("path").unwrap());
            unimplemented!();
        }
        ("pull", Some(subm)) => {
            let _path = Path::new(subm.value_of("path").unwrap());
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
