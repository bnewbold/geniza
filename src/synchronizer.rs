
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
use protobuf::parse_from_bytes;
use network_msgs::Data;
use metadata_msgs::Index;
use std::net::SocketAddr;
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
    is_drive: bool,
    local_id: [u8; 32],
    dir: Option<PathBuf>,
    unified_peers_tx: chan::Sender<Result<PeerMsg>>,
    unified_peers_rx: chan::Receiver<Result<PeerMsg>>,
    potential_peers: Vec<SocketAddr>,
}

impl Synchronizer {

    pub fn new_downloader(key: Key, mode: SyncMode, dir: &Path) -> Result<Synchronizer> {

        let metadata_reg = SleepDirRegister::create(dir.as_ref(), "metadata")?;

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
            is_drive: true,
            dir: Some(dir.to_path_buf()),
            registers: vec![metadata_status],
            unified_peers_tx,
            unified_peers_rx,
            potential_peers: vec![],
        };
        Ok(s)
    }

    pub fn discover(&mut self) -> Result<u64> {

        let meta_key = &self.registers.get(0).unwrap().key.clone();
        let new_peers = discover_peers_dns(&meta_key[0..32])?;
        let new_count = new_peers.len() as u64;

        for p in new_peers {
            self.add_peer(p);
        }
        Ok(new_count)
    }

    pub fn add_peer(&mut self, sa: SocketAddr) {

        if !self.potential_peers.contains(&sa) {
            self.potential_peers.push(sa);
        }
    }

    pub fn run(&mut self) -> Result<()> {

        let meta_key = &self.registers.get(0).unwrap().key.clone();
        let mut rng = OsRng::new()?;
        for p in &self.potential_peers {
            // TODO: somewhere in here validate that we haven't already connected to this peer id
            // by a different name
            let handle = rng.gen::<u64>();
            let pt = DatPeerThread::connect(p, meta_key.clone(), handle, false, Some(&self.local_id), self.unified_peers_tx.clone())?;
            self.peers.insert(handle, pt);
            let pt = self.peers.get_mut(&handle).unwrap();

            match self.mode {
                SyncMode::RxMax => {
                    init_want_everything(pt, 0)?;
                },
                SyncMode::RxEndless => unimplemented!(),
                SyncMode::TxEndless => unimplemented!(),
                SyncMode::RxTxEndless => unimplemented!(),
            };
        };

        // bug in chan_select!() breaking `self` reference?
        // "recursion limit reached while expanding the macro `chan_select`"
        let unified_peers_rx = self.unified_peers_rx.clone();

        loop {
            chan_select! {
                unified_peers_rx.recv() -> val => {
                    if let Some(Ok(pm)) = val {
                        self.handle_msg(&pm)?;
                    }
                },
            };
        }
    }

    fn handle_msg(&mut self, pm: &PeerMsg) -> Result<()> {

        // mutable ref to PeerThread for this message
        let pt = self.peers.get_mut(&pm.peer_handle).unwrap();

        // NB: this is the simplistic model of registers (only works up to 2x per peer?)
        if pm.feed_index as usize >= self.registers.len() {
            // XXX: invalid feed! drop connection
            pt.close()?;
        }

        match &pm.msg {
            &DatNetMessage::Feed(_) => { unimplemented!() },
            &DatNetMessage::Handshake(_) => { unimplemented!() },
            &DatNetMessage::Info(_) => { unimplemented!() },
            &DatNetMessage::Have(ref msg) => {
                // TODO: depending on mode...

                //let peer_has = extract_bitfield(msg)?;
                // TODO: remove bits we already have
                // TODO: depending on mode, extend 'wanted' bits
                // TODO: send a Request on this channel
                // XXX: dummy for testing
                let mut request = Request::new();
                request.set_index(msg.get_start());
                pt.send(DatNetMessage::Request(request), pm.feed_index)?;
            },
            &DatNetMessage::Unhave(_) => {}, // PASS
            &DatNetMessage::Want(_) => {}, // PASS
            &DatNetMessage::Unwant(_) => {}, // PASS
            &DatNetMessage::Request(_) => {}, // PASS
            &DatNetMessage::Cancel(_) => {}, // PASS
            &DatNetMessage::Data(ref msg) => {

                // TODO: feed indexing?
                // Insert into local feed
                // XXX self.registers[pm.feed_index].insert(msg);

                // If a drive, and this is the first entry of metadata feed, it has the config for
                // the content feed
                if self.is_drive && pm.feed_index == 0 && msg.get_index() == 0 {
                    let data_key = parse_drive_data_key(msg)?;
                    pt.add_feed(&data_key)?;
                    init_want_everything(pt, 1)?;

                    // If we haven't already, create and save a local register
                    if self.registers.len() < 2 {

                        let dir = self.dir.clone().unwrap();
                        let content_reg = SleepDirRegister::create(&dir, "content")?;

                        let content_status = RegisterStatus {
                            id: 1,
                            register: content_reg,
                            inflight: vec![],
                            wanted: BitVec::new(),
                            key: data_key,
                        };

                        self.registers.push(content_status);
                    }
                }

                // TODO: send next wanted, or otherwise update state
            },
        }
        Ok(())
    }
}

fn parse_drive_data_key(data_msg: &Data) -> Result<Key> {
    let index_msg = parse_from_bytes::<Index>(&mut data_msg.get_value())?;
    if index_msg.get_field_type() == "hyperdrive" {
        let data_key = index_msg.get_content();
        if data_key.len() != 32 {
            bail!("Received data key had wrong length: {}", data_key.len());
        }
        return Ok(Key::from_slice(&data_key[0..32]).unwrap());
    } else {
        bail!("non-hyperdrive Index type: {}", index_msg.get_field_type());
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

fn init_want_everything(dpt: &mut DatPeerThread, reg_index: u8) -> Result<()> {

    // Info: downloading, not uploading
    let mut im = Info::new();
    im.set_uploading(false);
    im.set_downloading(true);
    let im = DatNetMessage::Info(im);
    dpt.send(im, reg_index)?;

    // Have: nothing (so far)
    let mut hm = Have::new();
    hm.set_start(0);
    hm.set_length(0);
    let hm = DatNetMessage::Have(hm);
    dpt.send(hm, reg_index)?;

    // UnHave: still nothing
    let mut uhm = Unhave::new();
    uhm.set_start(0);
    let uhm = DatNetMessage::Unhave(uhm);
    dpt.send(uhm, reg_index)?;

    // Want: everything
    let mut wm = Want::new();
    wm.set_start(0);
    let wm = DatNetMessage::Want(wm);
    dpt.send(wm, reg_index)?;

    Ok(())
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
