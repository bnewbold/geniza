
use errors::*;
use network_msgs::*;
use bitfield::*;
use protocol::{DatNetMessage, DatConnection};
use sleep_register::HyperRegister;
use peer::DatPeer;
use sleep_register::SleepDirRegister;

pub enum SyncMode {
    RxMax,
    RxEndless,
    TxEndless,
    RxTxEndless,
}

pub struct Synchronizer {
    peers: Vec<DatPeer>,
    registers: Vec<SleepDirRegister>,
    mode: SyncMode,
    wanted: Bitfield,
    inflight: Vec<Vec<u64>>,
}

impl Synchronizer {

    pub fn next_wanted(&mut self, reg: u64) -> Option<(u64, u64)> {
        // XXX
        None
    }

    pub fn tick(&mut self) -> Result<()> {
        Ok(())
    }
}


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
pub fn node_simple_clone(host_port: &str, key: &[u8], register: &mut HyperRegister, reg_index: u8) -> Result<()> {

    if register.len()? > 0 {
        bail!("Register already had data in it (expected empty for naive clone)");
    }

    let mut dc = DatConnection::connect(host_port, key, false)?;

    // Info: downloading, not uploading
    let mut im = Info::new();
    im.set_uploading(false);
    im.set_downloading(true);
    let im = DatNetMessage::Info(im);
    dc.send_msg(&im, reg_index)?;

    // Have: nothing (so far)
    let mut hm = Have::new();
    hm.set_start(0);
    hm.set_length(0);
    let hm = DatNetMessage::Have(hm);
    dc.send_msg(&hm, reg_index)?;

    // UnHave: still nothing
    let mut uhm = Unhave::new();
    uhm.set_start(0);
    let uhm = DatNetMessage::Unhave(uhm);
    dc.send_msg(&uhm, reg_index)?;

    // Want: everything
    let mut wm = Want::new();
    wm.set_start(0);
    let wm = DatNetMessage::Want(wm);
    dc.send_msg(&wm, reg_index)?;

    let last_entry: u64;

    // Receive Have messages to determine lengths
    loop {
        let (msg, got_reg_index) = dc.recv_msg()?;
        match msg {
            DatNetMessage::Have(dh) =>  {
                info!("reg_index={}; {:?}; bitfield={:?}", got_reg_index, dh, dh.get_bitfield());
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
        dc.send_msg(&DatNetMessage::Request(rm), 0)?;
        let (msg, got_reg_index) = dc.recv_msg()?;
        assert!(got_reg_index == 0);
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
