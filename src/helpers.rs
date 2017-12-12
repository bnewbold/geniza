
use errors::*;
use crypto::digest::Digest;
use crypto::blake2b::Blake2b;

/// Helper to calculate a discovery key from a public key. 'key' should be 32 bytes; the returned
/// array will also be 32 bytes long.
///
/// dat discovery keys are calculated as a BLAKE2b "keyed hash" (using the passed key) of the string
/// "hypercore" (with no trailing null byte).
pub fn make_discovery_key(key: &[u8]) -> Vec<u8> {
    let mut discovery_key = [0; 32];
    let mut hash = Blake2b::new_keyed(32, key);
    hash.input(&"hypercore".as_bytes());
    hash.result(&mut discovery_key);
    discovery_key.to_vec()
}

/// Helper to parse a dat address (aka, public key) in string format.
///
/// Address can start with 'dat://'. It should contain 64 hexadecimal characters.
pub fn parse_dat_address(input: &str) -> Result<Vec<u8>> {

    let raw_key = if input.starts_with("dat://") {
        &input[6..]
    } else {
        input
    };
    if raw_key.len() != 32 * 2 {
        bail!("dat key not correct length");
    }
    let mut key_bytes = vec![];
    for i in 0..32 {
        let r = u8::from_str_radix(&raw_key[2 * i..2 * i + 2], 16);
        match r {
            Ok(b) => key_bytes.push(b),
            Err(e) => bail!("Problem with hex: {}", e),
        };
    }
    Ok(key_bytes)
}

#[test]
fn test_parse_dat_address() {

    assert!(parse_dat_address(
        "c7638882870abd4044d6467b0738f15e3a36f57c3a7f7f3417fd7e4e0841d597").is_ok());
    assert!(parse_dat_address(
        "C7638882870ABD4044D6467B0738F15E3A36F57C3A7F7F3417FD7E4E0841D597").is_ok());
    assert!(parse_dat_address(
        "dat://c7638882870abd4044d6467b0738f15e3a36f57c3a7f7f3417fd7e4e0841d597").is_ok());

    assert!(parse_dat_address(
        "c7638882870ab").is_err());
    assert!(parse_dat_address(
        "g7638882870abd4044d6467b0738f15e3a36f57c3a7f7f3417fd7e4e0841d597").is_err());
    assert!(parse_dat_address(
        "dat://c7638882870abd4044d6467b0738f15e3a36f57c3a7f7f3417fd7e4e0841d5970").is_err());
    assert!(parse_dat_address(
        "dat://c7638882870abd4044d6467b0738f15e3a36f57c3a7f7f3417fd7e4e0841d59").is_err());
}
