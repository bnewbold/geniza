
use errors::*;
use network_msgs::*;
use protocol::{DatNetMessage, DatConnection};
use integer_encoding::VarInt;
use bit_field::BitArray;
use sleep_register::HyperRegister;

fn decode_bitfiled(raw_bf: &[u8]) -> Result<Vec<u8>> {
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
fn max_high_bit(bf: &[u8]) -> u64 {
    // XXX: HACK, going backwards
    for i in 0..bf.bit_length() {
        if bf.get_bit(i) {
            return (bf.bit_length() - i - 1) as u64;
        }
    }
    return 0;
}

fn max_index(have_msg: &Have) -> Result<u64> {
    if have_msg.has_length() {
        return Ok(have_msg.get_start() + have_msg.get_length());
    } else if have_msg.has_bitfield() {
        let raw_bf = have_msg.get_bitfield();
        let bf = decode_bitfiled(raw_bf)?;
        trace!("decoded bitfield: {:?}", bf);
        return Ok(max_high_bit(&bf));
    } else {
        return Ok(have_msg.get_start() + 1);
    }
}

#[test]
fn test_max_index() {
    let mut hm = Have::new();
    hm.set_start(0);
    hm.set_bitfield(vec![7,2,128]);
    assert_eq!(max_index(&hm).unwrap(), 8);

    hm.set_bitfield(vec![2, 207]);
    assert_eq!(max_index(&hm).unwrap(), 7);

    // Alphabet test dat
    hm.set_bitfield(vec![2, 254]);
    assert_eq!(max_index(&hm).unwrap(), 6);
    hm.set_bitfield(vec![2, 252]);
    assert_eq!(max_index(&hm).unwrap(), 5);
}

/// Tries to connect to a single peer, pull register, and close.
pub fn node_simple_clone(host_port: &str, key: &[u8], register: &mut HyperRegister, is_content: bool) -> Result<()> {

    if register.len()? > 0 {
        bail!("Register already had data in it (expected empty for naive clone)");
    }

    let mut dc = DatConnection::connect(host_port, key, false)?;

    // Info: downloading, not uploading
    let mut im = Info::new();
    im.set_uploading(false);
    im.set_downloading(true);
    let im = DatNetMessage::Info(im);
    dc.send_msg(&im, is_content)?;

    // Have: nothing (so far)
    let mut hm = Have::new();
    hm.set_start(0);
    hm.set_length(0);
    let hm = DatNetMessage::Have(hm);
    dc.send_msg(&hm, is_content)?;

    // UnHave: still nothing
    let mut uhm = Unhave::new();
    uhm.set_start(0);
    let uhm = DatNetMessage::Unhave(uhm);
    dc.send_msg(&uhm, is_content)?;

    // Want: everything
    let mut wm = Want::new();
    wm.set_start(0);
    let wm = DatNetMessage::Want(wm);
    dc.send_msg(&wm, is_content)?;

    let last_entry: u64;

    // Receive Have messages to determine lengths
    loop {
        let (was_content, msg) = dc.recv_msg()?;
        match msg {
            DatNetMessage::Have(dh) =>  {
                info!("is_content={}; {:?}; bitfield={:?}", was_content, dh, dh.get_bitfield());
                last_entry = max_index(&dh)?;
                break;
            },
            _ => {
                info!("Other message: {:?}", &msg);
            }
        }
    }

    info!("last_entry={}", last_entry);

    // Request / Data loops
    for i in 0..(last_entry+1) {
        let mut rm = Request::new();
        rm.set_index(i);
        info!("Sending request: {:?}", rm);
        dc.send_msg(&DatNetMessage::Request(rm), false)?;
        let (was_content, msg) = dc.recv_msg()?;
        assert!(!was_content);
        match msg {
            DatNetMessage::Data(dm) =>  {
                info!("Got data: index={}", dm.get_index());
                assert!(dm.has_value());
                assert!(dm.get_index() == i);
                register.append(dm.get_value())?;
            },
            _ => {
                info!("Other message: {:?}", &msg);
            }
        }
    }
    Ok(())
}
