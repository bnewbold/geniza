
use errors::*;
use network_msgs::*;
use bitfield::*;
use protocol::{DatNetMessage, DatConnection};
use sleep_register::HyperRegister;

// Synchronizer
//  register_keys
//  peers: vec
//  registers: HyperRegisters
//  mode: enum
//  state: enum
//  wanted: bitfield
//  requested: vec
//
// fn next_wanted() -> Option((reg, u64))
// fn tick()


fn max_index(have_msg: &Have) -> Result<u64> {
    if have_msg.has_length() {
        return Ok(have_msg.get_start() + have_msg.get_length());
    } else if have_msg.has_bitfield() {
        let raw_bf = have_msg.get_bitfield();
        let bf = decode_bitfield(raw_bf)?;
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
