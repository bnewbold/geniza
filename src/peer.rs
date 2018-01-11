
use errors::*;
use std::convert::From;
use protocol::{DatConnection, DatNetMessage};
use network_msgs::*;
use metadata_msgs::Index;
use bitfield::Bitfield;
use sodiumoxide::crypto::stream::Key;
use protobuf::parse_from_bytes;
use make_discovery_key;

/// Wraps a low-level DatConnection with extra state about mutually active registers, bitfields the
/// remote has declare they Have, etc.
pub struct DatPeer {
    registers: Vec<Key>,
    conn: DatConnection,
    remote_has: Vec<Vec<Bitfield>>,
}

impl DatPeer {

    /// Has the remote peer indicated they have the given chunk in the given register?
    pub fn has(self, register: u64, index: u64) -> Result<bool> {
        for bitfield in self.remote_has[register as usize].iter() {
            if bitfield.get(index)? {
                return Ok(true)
            }
        }
        Ok(false)
    }

    pub fn add_register(&mut self, key: &[u8]) -> Result<()> {

        let key_bytes = key;
        let key = Key::from_slice(key_bytes).unwrap();

        for k in self.registers.iter() {
            if *k == key {
                warn!("tried to add register an existing key on a DatPeer connection");
                return Ok(())
            }
        }

        let index = self.registers.len();
        assert!(index < 256);
        let discovery_key = make_discovery_key(key_bytes);;

        // Send (encrypted) Feed message for data feed
        let mut feed_msg = Feed::new();
        feed_msg.set_discoveryKey(discovery_key.to_vec());
        self.conn.send_msg(&DatNetMessage::Feed(feed_msg), index as u8)?;

        self.registers.push(key.clone());
        self.remote_has.push(vec![]);
        Ok(())

    }

    /// hyperdrive-specific helper for discovering the public key for the "content" hyperregister
    /// from the "metadata" register, and sending a Feed message to initialize on this connection.
    pub fn init_data_feed(&mut self) -> Result<()> {

        if self.registers.len() > 1 {
            return Ok(());
        }

        let data_key = self.get_drive_data_key()?;
        self.add_register(&data_key[0..32])
    }


    /// hyperdrive-specific helper for returning the "data" hyperregister public key (aka, index=1)
    pub fn get_drive_data_key(&mut self) -> Result<Key> {

        if self.registers.len() > 1 {
            // we already have the key
            let key = self.registers[1].clone();
            return Ok(key);
        }

        // Info: downloading, not uploading
        let mut im = Info::new();
        im.set_uploading(false);
        im.set_downloading(true);
        self.conn.send_msg(&DatNetMessage::Info(im), 0)?;

        // Have: nothing (so far)
        let mut hm = Have::new();
        hm.set_start(0);
        hm.set_length(0);
        self.conn.send_msg(&DatNetMessage::Have(hm), 0)?;

        // UnHave: still nothing
        let mut uhm = Unhave::new();
        uhm.set_start(0);
        self.conn.send_msg(&DatNetMessage::Unhave(uhm), 0)?;

        // Want: just the first element
        let mut wm = Want::new();
        wm.set_start(0);
        wm.set_length(1);
        self.conn.send_msg(&DatNetMessage::Want(wm), 0)?;

        // listen for Have
        loop {
            let (msg, reg_index) = self.conn.recv_msg()?;
            if reg_index == 1 {
                continue;
            }
            if let DatNetMessage::Have(_) = msg {
                break;
            } else {
                info!("Expected Have message, got: {:?}", &msg);
                continue;
            }
        }

        // Request
        let mut rm = Request::new();
        rm.set_index(0);
        self.conn.send_msg(&DatNetMessage::Request(rm), 0)?;

        loop {
            let (msg, reg_index) = self.conn.recv_msg()?;
            if reg_index == 1 {
                info!("Expected other message channel");
                continue;
            }
            if let DatNetMessage::Data(dm) = msg {
                info!("Got metadata: {}", dm.get_index());
                if dm.get_index() == 0 {
                    let index_msg = parse_from_bytes::<Index>(&mut dm.get_value())?;
                    if index_msg.get_field_type() == "hyperdrive" {
                        let data_key = index_msg.get_content();
                        if data_key.len() != 32 {
                            bail!("Received data key had wrong length: {}", data_key.len());
                        }
                        // TODO: ok_or(), but what?
                        return Ok(Key::from_slice(&data_key[0..32]).unwrap());
                    } else {
                        bail!("non-hyperdrive Index type: {}", index_msg.get_field_type());
                    }
                }
            } else {
                info!("Expected Data message, got: {:?}", &msg);
                continue;
            }
        }
    }
}

impl From<DatConnection> for DatPeer {

    fn from(dc: DatConnection) -> DatPeer {
        let key = dc.key.clone();
        DatPeer {
            registers: vec![key],
            conn: dc,
            remote_has: vec![vec![]],
        }
    }
}
