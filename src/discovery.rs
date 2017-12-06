
use errors::*;
use std::net::{IpAddr, SocketAddr};
use make_discovery_key;
use data_encoding::HEXLOWER;
use resolve::{self, DnsConfig, DnsResolver, resolve_host};
use resolve::record::Srv;

pub fn discover_peers_dns(dat_key: &[u8]) -> Result<Vec<SocketAddr>> {

    let dk = make_discovery_key(dat_key);
    let dk_hex = HEXLOWER.encode(&dk);
    let dk_name = format!("{}.dat.local", &dk_hex[0..40]);
    info!("discovering peers using DNS: {}", dk_name);

    let dns1: Vec<IpAddr> = resolve_host("discovery1.publicbits.org")?.collect();
    let dns2: Vec<IpAddr> = resolve_host("discovery2.publicbits.org")?.collect();

    let default_config = resolve::default_config()?;
    let config = DnsConfig {
        name_servers: vec![
            SocketAddr::from((dns1[0], 53)),
            SocketAddr::from((dns2[0], 53))],
        search: vec!["dat.local".to_string()],
        n_dots: default_config.n_dots,
        timeout: default_config.timeout,
        attempts: default_config.attempts,
        rotate: true,
        use_inet6: false,
    };

    let resolver = DnsResolver::new(config)?;

    let peers: Vec<Srv> = resolver.resolve_record(&dk_name)?;
    // target (IP addresses) are returned with a trailing period that must be stripped
    let peers: Vec<SocketAddr> = peers.into_iter().map(|r| format!("{}:{}", &r.target[0..(r.target.len()-1)], r.port).parse().unwrap()).collect();
    info!("found peers: {:?}", peers);
    Ok(peers)
}
