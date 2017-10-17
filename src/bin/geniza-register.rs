
#[macro_use]
extern crate clap;

extern crate geniza;

// TODO: more careful import
use geniza::*;
use std::path::Path;

use clap::{App, SubCommand};

fn run() -> Result<()> {
 
    let matches = App::new("geniza-register")
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand(SubCommand::with_name("info")
            .about("Reads a SLEEP dir register and shows some basic metadata")
            .arg_from_usage("<DIR> 'directory containing files'")
            .arg_from_usage("<prefix> 'prefix for each data file'"))
        .subcommand(SubCommand::with_name("create")
            .about("Creates an SLEEP directory register (with header)")
            .arg_from_usage("<DIR> 'directory containing files'")
            .arg_from_usage("<prefix> 'prefix for each data file'"))
        .get_matches();


    match matches.subcommand() {
        ("info", Some(subm)) => {
            let dir = Path::new(subm.value_of("DIR").unwrap());
            let prefix = subm.value_of("prefix").unwrap();
            let sdr = SleepDirRegister::open(dir, prefix, false)?;
            //debug!(println!("{:?}", sdr));
            println!("Entry count: {}", sdr.len()?);
        },
        ("create", Some(subm)) => {
            let dir = Path::new(subm.value_of("DIR").unwrap());
            let prefix = subm.value_of("prefix").unwrap();
            SleepDirRegister::create(dir, prefix)?;
            println!("Done!");
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