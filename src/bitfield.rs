
use errors::*;
use integer_encoding::VarInt;
use bit_field::BitArray;
use network_msgs::Have;

pub struct Bitfield {
    inner: Vec<u64>,
}

impl Bitfield {

    pub fn from_have_msg(_msg: &Have) -> Bitfield {
        unimplemented!()
    }

    pub fn get(&self, _index: u64) -> Result<bool> {
        unimplemented!()
    }
}

pub fn decode_bitfield(raw_bf: &[u8]) -> Result<Vec<u8>> {
    let mut offset = 0; // byte offset that we have read up to
    if raw_bf.len() < 1 {
        bail!("Expected (varint-encoded) bitfield to have len>=1");
    }
    let mut bit_array: Vec<u8> = vec![];
    while offset < raw_bf.len() {
        let (header, inc): (u64, usize) = VarInt::decode_var(&raw_bf[offset..]);
        offset += inc;

        if (header & 0x01) == 0x01 {
            // compressed
            let bit = (header & 0x02) == 0x02;
            let run_len = header >> 2;
            if bit {
                bit_array.append(&mut vec![0xFF; run_len as usize]);
            } else {
                bit_array.append(&mut vec![0x00; run_len as usize]);
            }
        } else {
            // uncompressed
            let byte_count = header >> 1;
            let mut data = raw_bf[offset..(offset + byte_count as usize)].to_vec();
            bit_array.append(&mut data);
            offset += byte_count as usize;
        }
    }
    // XXX: HACK
    bit_array.reverse();
    return Ok(bit_array);
}

/// Finds the index of the lowest bit
pub fn max_high_bit(bf: &[u8]) -> u64 {
    // XXX: HACK, going backwards
    for i in 0..bf.bit_length() {
        if bf.get_bit(i) {
            return (bf.bit_length() - i - 1) as u64;
        }
    }
    return 0;
}
