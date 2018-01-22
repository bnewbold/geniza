
use errors::*;
use network_msgs::*;
use bitfield::*;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use protocol::{DatNetMessage, DatConnection};
use rand::{OsRng, Rng};
use sleep_register::HyperRegister;
use peer::{DatPeerThread, PeerMsg};
use sleep_register::SleepDirRegister;
use sodiumoxide::crypto::stream::Key;
use bit_vec::BitVec;
use discovery::discover_peers_dns;
use chan;

pub enum SyncMode {
    RxMax,
    RxEndless,
    TxEndless,
    RxTxEndless,
}

pub struct RegisterStatus {
    id: u8,
    register: SleepDirRegister,
    inflight: Vec<u64>,
    wanted: BitVec,
    key: Key,
}

pub struct Synchronizer {
    peers: HashMap<u64, DatPeerThread>,
    registers: Vec<RegisterStatus>,
    mode: SyncMode,
    local_id: [u8; 32],
    dir: Option<PathBuf>,
    unified_peers_tx: chan::Sender<Result<PeerMsg>>,
    unified_peers_rx: chan::Receiver<Result<PeerMsg>>,
}

impl Synchronizer {

    pub fn new_downloader(key: Key, mode: SyncMode, dir: &Path) -> Result<Synchronizer> {

        let mut metadata_reg = SleepDirRegister::create(dir.as_ref(), "metadata")?;

        let (unified_peers_tx, unified_peers_rx) = chan::async();

        let metadata_status = RegisterStatus {
            id: 0,
            register: metadata_reg,
            inflight: vec![],
            wanted: BitVec::new(),
            key,
        };

        let mut rng = OsRng::new()?;
        let mut local_id = [0; 32];
        rng.fill_bytes(&mut local_id);

        let s = Synchronizer {
            peers: HashMap::new(),
            mode,
            local_id,
            dir: Some(dir.to_path_buf()),
            registers: vec![metadata_status],
            unified_peers_tx,
            unified_peers_rx,
        };
        Ok(s)
    }

    pub fn run(&mut self) -> Result<()> {

        let meta_key = &self.registers[0].key;
        let peers = discover_peers_dns(&meta_key[0..32])?;
        let mut rng = OsRng::new()?;
        for p in peers {
            let handle = rng.gen::<u64>();
            let pt = DatPeerThread::connect(p, meta_key.clone(), handle, false, Some(&self.local_id), self.unified_peers_tx.clone())?;
            self.peers.insert(handle, pt);
            // TODO: send a large want? or wait for haves?
        };

        // bug in chan_select!() breaking `self` reference?
        let unified_peers_rx = self.unified_peers_rx.clone();

        loop {
            chan_select! {
                unified_peers_rx.recv() -> val => { },
            };
        }
    }

                /*
                match val {
                    Some(Ok(pm)) => {
                        self.handle_msg(&pm);
                    },
                    Some(Err(e)) => {
                        // TODO: don't bail here
                        bail!("Got a client error: {}", e);
                    },
                    _ => { unimplemented!() },
                };
                */

    fn handle_msg(&mut self, pm: &PeerMsg) -> Result<()> {

        match &pm.msg {
            &DatNetMessage::Feed(_) => { unimplemented!() },
            &DatNetMessage::Handshake(_) => { unimplemented!() },
            &DatNetMessage::Info(_) => { unimplemented!() },
            &DatNetMessage::Have(ref have) => {
                // TODO: bulk-add haves to peer status
            },
            &DatNetMessage::Unhave(_) => {}, // PASS
            &DatNetMessage::Want(_) => {}, // PASS
            &DatNetMessage::Unwant(_) => {}, // PASS
            &DatNetMessage::Request(_) => {}, // PASS
            &DatNetMessage::Cancel(_) => {}, // PASS
            &DatNetMessage::Data(_) => {
                if pm.feed_index as usize >= self.registers.len() {
                    // TODO: invalid feed! drop connection
                }
                // TODO: insert into feed
            },
        }
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

    let key = Key::from_slice(key).unwrap();
    let mut dc = DatConnection::connect(host_port, &key, false, None)?;

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
