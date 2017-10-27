
use std::net::TcpStream;
use std::time::Duration;
use std::io::{Read, Write};
use std::cmp;
use crypto::digest::Digest;
use crypto::blake2b::Blake2b;
use sodiumoxide::crypto::stream::*;
use rand::{OsRng, Rng};
use protobuf::Message;
use protobuf::parse_from_bytes;
use integer_encoding::{VarIntReader, VarIntWriter};

use errors::*;
use network_msg::*;
use metadata_msg::Index;

#[derive(Debug)]
pub enum DatNetMessage {
    Register(Feed),
    Handshake(Handshake),
    Status(Info),
    Have(Have),
    Unhave(Unhave),
    Want(Want),
    Unwant(Unwant),
    Request(Request),
    Cancel(Cancel),
    Data(Data),
}

fn msg_code(msg: &DatNetMessage) -> u8 {
    match msg {
        &DatNetMessage::Register(_)  => 0,
        &DatNetMessage::Handshake(_) => 1,
        &DatNetMessage::Status(_)    => 2,
        &DatNetMessage::Have(_)      => 3,
        &DatNetMessage::Unhave(_)    => 4,
        &DatNetMessage::Want(_)      => 5,
        &DatNetMessage::Unwant(_)    => 6,
        &DatNetMessage::Request(_)   => 7,
        &DatNetMessage::Cancel(_)    => 8,
        &DatNetMessage::Data(_)      => 9,
    }
}

fn msg_sugar(msg: &DatNetMessage) -> &Message {
    match msg {
        &DatNetMessage::Register(ref m)  => m,
        &DatNetMessage::Handshake(ref m) => m,
        &DatNetMessage::Status(ref m)    => m,
        &DatNetMessage::Have(ref m)      => m,
        &DatNetMessage::Unhave(ref m)    => m,
        &DatNetMessage::Want(ref m)      => m,
        &DatNetMessage::Unwant(ref m)    => m,
        &DatNetMessage::Request(ref m)   => m,
        &DatNetMessage::Cancel(ref m)    => m,
        &DatNetMessage::Data(ref m)      => m,
    }
}

/// This helper is pretty slow/inefficient; lots of copying memory
fn bytewise_stream_xor_ic_inplace(buf: &mut [u8], byte_offset: u64, nonce: &Nonce, key: &Key) {

    let mut offset = byte_offset;

    // We may have a partial-64-byte-block to finish encrypting first
    let partial_offset: usize = (offset % 64) as usize;
    let partial_len: usize = cmp::min(64 - partial_offset, buf.len());
    if partial_len != 0 {
        let mut partial = vec![0; 64];
        for i in 0..partial_len {
            partial[partial_offset+i] = buf[i];
        }
        let partial_enc = stream_xor_ic(&partial, offset / 64, &nonce, &key);
        offset += partial_len as u64;
        for i in 0..partial_len {
            buf[i] = partial_enc[partial_offset+i];
        }
    }
    if buf.len() > partial_len {
        let main_enc = stream_xor_ic(&buf[partial_len..], offset / 64, &nonce, &key);
        //offset += main_enc.len() as u64;
        for i in 0..main_enc.len() {
            buf[partial_len+i] = main_enc[i];
        }
    }
}

#[test]
fn test_bsxii_short() {

    let nonce = gen_nonce();
    let key = gen_key();

    for size in [10, 100, 1234].iter() {
        let mut a = vec![7; *size];
        let mut b = vec![7; *size];
        let c = vec![7; *size];

        assert_eq!(a, b);
        bytewise_stream_xor_ic_inplace(&mut a, 0, &nonce, &key);
        bytewise_stream_xor_ic_inplace(&mut b, 0, &nonce, &key);
        assert_eq!(a, b);
        assert_ne!(a, c);
        bytewise_stream_xor_ic_inplace(&mut a, 0, &nonce, &key);
        assert_eq!(a, c);
    }
}

#[test]
fn test_bsxii_continued() {

    let nonce = gen_nonce();
    let key = gen_key();

    let mut a = vec![7; 1234];
    let mut b = vec![7; 1234];
    let c = vec![7; 1234];

    assert_eq!(a, b);
    bytewise_stream_xor_ic_inplace(&mut a[0..10], 0, &nonce, &key);
    bytewise_stream_xor_ic_inplace(&mut a[10..20], 10, &nonce, &key);
    bytewise_stream_xor_ic_inplace(&mut b[0..20], 0, &nonce, &key);
    assert_eq!(a, b);
    bytewise_stream_xor_ic_inplace(&mut a[20..50], 20, &nonce, &key);
    bytewise_stream_xor_ic_inplace(&mut a[50..500], 50, &nonce, &key);
    bytewise_stream_xor_ic_inplace(&mut b[20..500], 20, &nonce, &key);
    assert_ne!(a, c);
    assert_eq!(a, b);
    bytewise_stream_xor_ic_inplace(&mut a[500..1234], 500, &nonce, &key);
    bytewise_stream_xor_ic_inplace(&mut a, 0, &nonce, &key);
    assert_eq!(a, c);
}

fn make_discovery_key(key: &[u8]) -> Vec<u8> {
    // calculate discovery key
    let mut discovery_key = [0; 32];
    let mut hash = Blake2b::new_keyed(32, key);
    hash.input(&"hypercore".as_bytes());
    hash.result(&mut discovery_key);
    discovery_key.to_vec()
}

/// Represents a bi-directional connection to a network peer
///
/// Spec says nonce is 32 bytes, by dat implementation (hypercore-protocol) is 24 bytes.
pub struct DatConnection {
    id: [u8; 32],
    remote_id: [u8; 32],
    tcp: TcpStream,
    live: bool,
    key: Key,
    discovery_key: [u8; 32],
    data_key: [u8; 32],
    data_discovery_key: [u8; 32],
    tx_nonce: Nonce,
    tx_offset: u64,
    rx_nonce: Nonce,
    rx_offset: u64,
}

impl Read for DatConnection {

    /// Encrypted TCP read (after connection initialized). Uses XOR of an XSalsa20 stream, using
    /// block offsets.
    fn read(&mut self, buf: &mut [u8]) -> ::std::io::Result<usize> {
        let len = self.tcp.read(buf)?;
        bytewise_stream_xor_ic_inplace(&mut buf[0..len], self.rx_offset, &self.rx_nonce, &self.key);
        self.rx_offset += len as u64;

        Ok(len)
    }
}

impl Write for DatConnection {

    /// Encrypted write to complement `read()`.
    fn write(&mut self, buf: &[u8]) -> ::std::io::Result<usize> {

        // Don't mutate what we've been passed
        let mut enc = vec![0; buf.len()];
        enc.copy_from_slice(buf);

        bytewise_stream_xor_ic_inplace(&mut enc, self.tx_offset, &self.tx_nonce, &self.key);
        self.tx_offset += enc.len() as u64;

        self.tcp.write(&enc)
    }

    fn flush(&mut self) -> ::std::io::Result<()> {
        self.tcp.flush()
    }
}

impl DatConnection {

    pub fn connect(host_port: &str, key: &[u8], live: bool) -> Result<DatConnection> {

        let timeout = Duration::new(7, 0);
        let tx_nonce = gen_nonce();
        let mut local_id = [0; 32];
        let mut rng = OsRng::new()?;
        rng.fill_bytes(&mut local_id);

        let mut dk = [0; 32];
        dk.copy_from_slice(&make_discovery_key(key)[0..32]);

        // Connect to server
        info!("Connecting to {}", host_port);
        // TODO: timeout on connect (socketaddr dance)
        let tcp = TcpStream::connect(host_port)?;
        tcp.set_read_timeout(Some(timeout))?;
        tcp.set_write_timeout(Some(timeout))?;

        let mut dc = DatConnection {
            id: local_id,
            tcp,
            live,
            remote_id: [0; 32],
            key: Key::from_slice(key).unwrap(), // TODO:
            discovery_key: dk,
            data_key: [0; 32],
            data_discovery_key: [0; 32],
            tx_nonce: tx_nonce,
            tx_offset: 0,
            rx_nonce: gen_nonce(),  // dummy
            rx_offset: 0,
        };

        // Exchange register/feed
        dc.tcp.set_nodelay(true)?; // Faster handshake
        // send register
        let mut register_msg = Feed::new();
        register_msg.set_discoveryKey(dc.discovery_key.to_vec());
        register_msg.set_nonce((tx_nonce[0..24]).to_vec());
        dc.send_register(&register_msg)?;

        // read register
        let registration = dc.recv_register()?;
        if registration.get_discoveryKey()[0..32] != dk[..] {
            bail!("Remote peer not sharing same discovery key");
        }
        let rn = registration.get_nonce();
        dc.rx_nonce = Nonce::from_slice(&rn).unwrap();

        // send handshake
        let mut handshake_msg = Handshake::new();
        handshake_msg.set_live(dc.live);
        handshake_msg.set_id(dc.id.to_vec());
        dc.send_msg(&DatNetMessage::Handshake(handshake_msg), false)?;

        // read handshake
        let (was_content, msg) = dc.recv_msg()?;
        if was_content {
            bail!("Expected metadata msg, not content");
        }
        if let DatNetMessage::Handshake(handshake) = msg {
            // TODO: more satisfying way to do this copy
            let hid = handshake.get_id();
            for i in 0..32 {
                dc.remote_id[i] = hid[i];
            }
        } else {
            bail!("Expected Handshake message, got something else");
        }

        // TODO: read data feed register here?

        // Fetch and configure key for data feed
        dc.get_data_key()?;

        // Send (encrypted) Register/Feed message for data feed
        let mut register_msg = Feed::new();
        register_msg.set_discoveryKey(dc.data_discovery_key.to_vec());
        dc.send_msg(&DatNetMessage::Register(register_msg), true)?;

        dc.tcp.set_nodelay(false)?; // Back to normal

        Ok(dc)
    }

    fn send_msg(&mut self, dnm: &DatNetMessage, is_content: bool) -> Result<()> {

        let header_int: u8 = (is_content as u8) << 4 | (msg_code(dnm) & 0x0F);
        let msg: &Message = msg_sugar(dnm);
        let total_message_size = (msg.compute_size() as usize) + 1;

        trace!("SEND total_len={}  header={}  is_content={} type={:?}",
            total_message_size, header_int, is_content, &dnm);

        // send both header varints, and data
        self.write_varint(total_message_size as u64)?;
        self.write_varint(header_int as u32)?;

        match dnm {
            &DatNetMessage::Register(ref m) => m.write_to_writer(self)?,
            &DatNetMessage::Handshake(ref m) => m.write_to_writer(self)?,
            &DatNetMessage::Status(ref m) => m.write_to_writer(self)?,
            &DatNetMessage::Have(ref m) => m.write_to_writer(self)?,
            &DatNetMessage::Unhave(ref m) => m.write_to_writer(self)?,
            &DatNetMessage::Want(ref m) => m.write_to_writer(self)?,
            &DatNetMessage::Unwant(ref m) => m.write_to_writer(self)?,
            &DatNetMessage::Request(ref m) => m.write_to_writer(self)?,
            &DatNetMessage::Cancel(ref m) => m.write_to_writer(self)?,
            &DatNetMessage::Data(ref m) => m.write_to_writer(self)?,
        }
        Ok(())
    }

    fn recv_msg(&mut self) -> Result<(bool, DatNetMessage)> {

        let total_len: u64 = self.read_varint()?;
        let header: u8 = self.read_varint()?;

        let is_content = (header & (1 << 4)) != 0;

        trace!("RECV total_len={}  header={}  is_content={}", total_len, header, is_content);

        if header > 0x1F {
            bail!("Invalid header received: {}", header);
        }

        let msg_len = (total_len - 1) as usize;
        let mut buf = vec![0; msg_len];
        self.read_exact(&mut buf[0..msg_len])?;

        let dnm = match header & 0x0F {
            0 => DatNetMessage::Register(parse_from_bytes::<Feed>(&mut buf)?),
            1 => DatNetMessage::Handshake(parse_from_bytes::<Handshake>(&mut buf)?),
            2 => DatNetMessage::Status(parse_from_bytes::<Info>(&mut buf)?),
            3 => DatNetMessage::Have(parse_from_bytes::<Have>(&mut buf)?),
            4 => DatNetMessage::Unhave(parse_from_bytes::<Unhave>(&mut buf)?),
            5 => DatNetMessage::Want(parse_from_bytes::<Want>(&mut buf)?),
            6 => DatNetMessage::Unwant(parse_from_bytes::<Unwant>(&mut buf)?),
            7 => DatNetMessage::Request(parse_from_bytes::<Request>(&mut buf)?),
            8 => DatNetMessage::Cancel(parse_from_bytes::<Cancel>(&mut buf)?),
            9 => DatNetMessage::Data(parse_from_bytes::<Data>(&mut buf)?),
            other => bail!("Unimplemented message type received: {}", other),
        };
        trace!("\twas: {:?}", &dnm);
        Ok((is_content, dnm))
    }

    /// Special unencrypted variant of `send_msg()`, used only during initial connection
    /// establishment (eg, to check metadata discovery key and exchange nonces). After the
    /// connection is initialized, send Register (aka Feed) messages as normal to add extra feeds.
    fn send_register(&mut self, reg: &Feed) -> Result<()> {
        // TODO: refactor this to take discovery key and nonce directly

        let header_int: u8 = 0;
        let total_message_size = (reg.compute_size() as usize) + 1;

        trace!("SEND total_len={}  header={} msg={:?}",
            total_message_size, header_int, reg);

        self.tcp.write_varint(total_message_size as u64)?;
        self.tcp.write_varint(header_int as u32)?;
        reg.write_to_writer(&mut self.tcp)?;
        Ok(())
    }

    /// Receive complement to `send_register()`.
    fn recv_register(&mut self) -> Result<Feed> {
        // TODO: refactor this to return discovery key and nonce directly

        let total_len: u64 = self.tcp.read_varint()?;
        let header: u8 = self.tcp.read_varint()?;

        if header != 0 {
            bail!("Invalid register header received");
        }

        trace!("RECV total_len={}  header={}", total_len, header);

        let msg_len = (total_len - 1) as usize;
        let mut buf = vec![0; msg_len];
        self.tcp.read_exact(&mut buf[0..msg_len])?;

        let reg = parse_from_bytes::<Feed>(&mut buf)?;
        trace!("\twas: {:?}", reg);
        Ok(reg)
    }

    pub fn get_data_key(&mut self) -> Result<()> {

        // Status: downloading, not uploading
        let mut sm = Info::new();
        sm.set_uploading(false);
        sm.set_downloading(true);
        self.send_msg(&DatNetMessage::Status(sm), false)?;

        // Have: nothing (so far)
        let mut hm = Have::new();
        hm.set_start(0);
        hm.set_length(0);
        self.send_msg(&DatNetMessage::Have(hm), false)?;

        // UnHave: still nothing
        let mut uhm = Unhave::new();
        uhm.set_start(0);
        self.send_msg(&DatNetMessage::Unhave(uhm), false)?;

        // Want: just the first element
        let mut wm = Want::new();
        wm.set_start(0);
        wm.set_length(1);
        self.send_msg(&DatNetMessage::Want(wm), false)?;

        // listen for Have
        loop {
            let (was_content, msg) = self.recv_msg()?;
            if was_content {
                continue;
            }
            if let DatNetMessage::Have(_) = msg {
                break;
            } else {
                info!("Expected Have message, got: {:?}", &msg);
                continue;
            }
        };

        // Request
        let mut rm = Request::new();
        rm.set_index(0);
        self.send_msg(&DatNetMessage::Request(rm), false)?;

        loop {
            let (was_content, msg) = self.recv_msg()?;
            if was_content {
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
                        info!("Got data discovery key");
                        self.data_key.copy_from_slice(&data_key[0..32]);
                        self.data_discovery_key.copy_from_slice(&make_discovery_key(data_key)[0..32]);
                        return Ok(());
                    } else {
                        unimplemented!("non-hyperdrive Index type: {}", index_msg.get_field_type());
                    }
                }
            } else {
                info!("Expected Data message, got: {:?}", &msg);
                continue;
            }
        }
    }

    pub fn receive_all(&mut self, is_content: bool, length: u64) -> Result<()> {

        // Status: downloading, not uploading
        let mut sm = Info::new();
        sm.set_uploading(false);
        sm.set_downloading(true);
        self.send_msg(&DatNetMessage::Status(sm), is_content)?;

        // Have: nothing (so far)
        let mut hm = Have::new();
        hm.set_start(0);
        hm.set_length(0);
        self.send_msg(&DatNetMessage::Have(hm), is_content)?;

        // UnHave: still nothing
        let mut uhm = Unhave::new();
        uhm.set_start(0);
        self.send_msg(&DatNetMessage::Unhave(uhm), is_content)?;

        // Want: everything
        let mut wm = Want::new();
        wm.set_start(0);
        self.send_msg(&DatNetMessage::Want(wm), is_content)?;

        // Request / Data loop
        for i in 0..length {
            let mut rm = Request::new();
            rm.set_index(i);
            self.send_msg(&DatNetMessage::Request(rm), is_content)?;

            loop {
                let (was_content, msg) = self.recv_msg()?;
                if was_content != is_content{
                    info!("Expected other message channel");
                    continue;
                }
                if let DatNetMessage::Data(dm) = msg {
                    info!("Got content: {}", dm.get_index());
                    break;
                } else {
                    info!("Expected Data message, got: {:?}", &msg);
                    continue;
                }
            }
        }

        Ok(())
    }
}
