// Free Software under GPL-3.0, see LICENSE
// Copyright 2017 Bryan Newbold

extern crate clap;
extern crate env_logger;
#[macro_use]
extern crate error_chain;
extern crate geniza;

// TODO: more careful import
use geniza::*;
use clap::{App, SubCommand};

fn run() -> Result<()> {
    env_logger::init().unwrap();

    let matches = App::new("geniza-net")
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand(
            SubCommand::with_name("connect")
                .about("Connects to a peer and exchanges handshake")
                .arg_from_usage("<host_port> 'peer host:port to connect to'")
                .arg_from_usage("<dat_key> 'dat key (public key) to register with'"),
        )
        .subcommand(
            SubCommand::with_name("receive-all")
                .about("Connects to a peer, pulls all metadata and content")
                .arg_from_usage("<host_port> 'peer host:port to connect to'")
                .arg_from_usage("<dat_key> 'dat key (public key) to register with'"),
        )
        .subcommand(
            SubCommand::with_name("discovery-key")
                .about("Prints (in hex) the discovery key for a dat archive")
                .arg_from_usage("<dat_key> 'dat key (public key) to convert (in hex)'"),
        )
        .subcommand(
            SubCommand::with_name("discovery-dns-name")
                .about("Prints the DNS name to query (mDNS or centrally) for peers")
                .arg_from_usage("<dat_key> 'dat key (public key) to convert (in hex)'"),
        )
        .get_matches();


    match matches.subcommand() {
        ("connect", Some(subm)) => {
            let host_port = subm.value_of("host_port").unwrap();
            let dat_key = subm.value_of("dat_key").unwrap();
            if dat_key.len() != 32 * 2 {
                bail!("dat key not correct length");
            }
            let mut key_bytes = vec![];
            for i in 0..32 {
                let r = u8::from_str_radix(&dat_key[2 * i..2 * i + 2], 16);
                match r {
                    Ok(b) => key_bytes.push(b),
                    Err(e) => bail!("Problem with hex: {}", e),
                };
            }
            DatConnection::connect(host_port, &key_bytes, false)?;
            println!("Done!");
        }
        ("receive-all", Some(subm)) => {
            let host_port = subm.value_of("host_port").unwrap();
            let dat_key = subm.value_of("dat_key").unwrap();
            if dat_key.len() != 32 * 2 {
                bail!("dat key not correct length");
            }
            let mut key_bytes = vec![];
            for i in 0..32 {
                let r = u8::from_str_radix(&dat_key[2 * i..2 * i + 2], 16);
                match r {
                    Ok(b) => key_bytes.push(b),
                    Err(e) => bail!("Problem with hex: {}", e),
                };
            }
            let mut dc = DatConnection::connect(host_port, &key_bytes, false)?;
            // XXX: number here totally arbitrary
            dc.receive_all(false, 10)?;
            dc.receive_all(true, 10)?;
            println!("Done!");
        }
        ("discovery-key", Some(subm)) => {
            let dat_key = subm.value_of("dat_key").unwrap();
            if dat_key.len() != 32 * 2 {
                bail!("dat key not correct length");
            }
            let mut key_bytes = vec![];
            for i in 0..32 {
                let r = u8::from_str_radix(&dat_key[2 * i..2 * i + 2], 16);
                match r {
                    Ok(b) => key_bytes.push(b),
                    Err(e) => bail!("Problem with hex: {}", e),
                };
            }
            let disc_key = make_discovery_key(&key_bytes);
            for b in disc_key {
                print!("{:02x}", b);
            }
            println!("");
        }
        ("discovery-dns-name", Some(subm)) => {
            let dat_key = subm.value_of("dat_key").unwrap();
            if dat_key.len() != 32 * 2 {
                bail!("dat key not correct length");
            }
            let mut key_bytes = vec![];
            for i in 0..32 {
                let r = u8::from_str_radix(&dat_key[2 * i..2 * i + 2], 16);
                match r {
                    Ok(b) => key_bytes.push(b),
                    Err(e) => bail!("Problem with hex: {}", e),
                };
            }
            let disc_key = make_discovery_key(&key_bytes);
            for b in 0..20 {
                print!("{:02x}", disc_key[b]);
            }
            println!(".dat.local");
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
