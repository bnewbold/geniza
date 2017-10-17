
#[macro_use]
extern crate clap;

extern crate geniza;

// TODO: more careful import
use geniza::*;
use std::path::Path;

use clap::{App, SubCommand};

fn run() -> Result<()> {
 
    let matches = App::new("geniza-sleep")
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand(SubCommand::with_name("info")
            .about("Reads a SLEEP file and shows some basic metadata")
            .arg_from_usage("<FILE> 'SLEEP file to read'"))
        .subcommand(SubCommand::with_name("create")
            .about("Creates an empty SLEEP file (with header)")
            .arg_from_usage("<FILE> 'SLEEP file to write (can't exist)'")
            .arg_from_usage("<magic> 'Magic word to use (eg, 0x5025700)'")
            .arg_from_usage("<entry_size> 'Size of each entry (bytes)'")
            .arg_from_usage("<algo_name> 'Name of algorithm (empty string for none)'"))
        .subcommand(SubCommand::with_name("read-all")
            .about("Reads a SLEEP file, iterates through all entries, prints raw bytes")
            .arg_from_usage("<FILE> 'SLEEP file to read'"))
        .get_matches();


    match matches.subcommand() {
        ("info", Some(subm)) => {
            let path = Path::new(subm.value_of("FILE").unwrap());
            let sf = SleepFile::open(path, false)?;
            //debug!(println!("{:?}", sf));
            println!("Magic: 0x{:X}", sf.get_magic());
            println!("Algorithm: '{}'", sf.get_algorithm().or(Some("".to_string())).unwrap());
            println!("Entry Size (bytes): {}", sf.get_entry_size());
            println!("Entry count: {}", sf.len()?);
        },
        ("create", Some(subm)) => {
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
                algo_name)?;
            println!("Done!");
        },
        ("read-all", Some(subm)) => {
            let path = Path::new(subm.value_of("FILE").unwrap());
            let mut sf = SleepFile::open(path, false)?;
            for i in 0..sf.len()? {
                println!("{}: {:?}", i, sf.read(i));
            }
        },
        _ => {
            println!("Missing or unimplemented command!");
            println!("{}", matches.usage());
            ::std::process::exit(-1);
        },
    }
    Ok(())
}

// TODO: is there a shorter error_chain 'main()' to use here?
fn main() {
    if let Err(ref e) = run() {
        println!("error: {}", e);

        for e in e.iter().skip(1) {
            println!("caused by: {}", e);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            println!("backtrace: {:?}", backtrace);
        }

        ::std::process::exit(1);
    }
}
