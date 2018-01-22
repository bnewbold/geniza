// Free Software under GPL-3.0, see LICENSE
// Copyright 2017 Bryan Newbold

extern crate clap;
extern crate env_logger;
#[macro_use]
extern crate error_chain;
extern crate geniza;
extern crate sodiumoxide;

// TODO: more careful import
use geniza::*;
use std::path::Path;
use clap::{App, SubCommand, Arg};
use sodiumoxide::crypto::stream::Key;

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
            SubCommand::with_name("discovery-key")
                .about("Prints (in hex) the discovery key for a dat archive")
                .arg_from_usage("<dat_key> 'dat key (public key) to convert (in hex)'"),
        )
        .subcommand(
            SubCommand::with_name("discovery-dns-name")
                .about("Prints the DNS name to query (mDNS or centrally) for peers")
                .arg_from_usage("<dat_key> 'dat key (public key) to convert (in hex)'"),
        )
        .subcommand(
            SubCommand::with_name("discover-dns")
                .about("Does a centralized DNS lookup for peers with the given key")
                .arg_from_usage("<dat_key> 'dat key (public key) to lookup"),
        )
        .subcommand(
            SubCommand::with_name("naive-clone")
                .about("Pulls a drive from a single (known) peer, using a naive algorithm")
                .arg(Arg::with_name("dat-dir")
                    .short("d")
                    .long("dat-dir")
                    .value_name("PATH")
                    .help("dat drive directory")
                    .default_value(".dat")
                    .takes_value(true))
                .arg_from_usage("<host_port> 'peer host:port to connect to'")
                .arg_from_usage("<dat_key> 'dat key (public key) to pull"),
        )
        .get_matches();


    match matches.subcommand() {
        ("connect", Some(subm)) => {
            let host_port = subm.value_of("host_port").unwrap();
            let dat_key = subm.value_of("dat_key").unwrap();
            let key_bytes = parse_dat_address(&dat_key)?;
            let key = Key::from_slice(&key_bytes).unwrap();
            DatConnection::connect(host_port, &key, false)?;
            println!("Done!");
        }
        ("discovery-key", Some(subm)) => {
            let dat_key = subm.value_of("dat_key").unwrap();
            let key_bytes = parse_dat_address(&dat_key)?;
            let disc_key = make_discovery_key(&key_bytes);
            for b in disc_key {
                print!("{:02x}", b);
            }
            println!("");
        }
        ("discovery-dns-name", Some(subm)) => {
            let dat_key = subm.value_of("dat_key").unwrap();
            let key_bytes = parse_dat_address(&dat_key)?;
            let disc_key = make_discovery_key(&key_bytes);
            for b in 0..20 {
                print!("{:02x}", disc_key[b]);
            }
            println!(".dat.local");
        }
        ("discover-dns", Some(subm)) => {
            let dat_key = subm.value_of("dat_key").unwrap();
            let key_bytes = parse_dat_address(&dat_key)?;
            let peers = discover_peers_dns(&key_bytes)?;
            if peers.len() == 0 {
                println!("No peers found!");
            } else {
                for p in peers {
                    println!("{}", p);
                }
            }
        }
        ("naive-clone", Some(subm)) => {
            let host_port = subm.value_of("host_port").unwrap();
            let dat_key = subm.value_of("dat_key").unwrap();
            let key_bytes = parse_dat_address(&dat_key)?;
            let dir = Path::new(subm.value_of("dat-dir").unwrap());
            let mut metadata = SleepDirRegister::create(&dir, "metadata")?;
            node_simple_clone(host_port, &key_bytes, &mut metadata, 0)?;
            // TODO: read out content key from metadata register
            //let content = SleepDirRegister::create(&dir, "content")?;
            //node_simple_clone(host_port, &key_bytes, &mut content, true)?;
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
