
use std::net::TcpStream;
use std::time::Duration;
use std::io::{Read, Write};
use crypto::digest::Digest;
use crypto::blake2b::Blake2b;
use sodiumoxide::crypto::stream::*;
use rand::{OsRng, Rng};
use protobuf::Message;
use protobuf::parse_from_bytes;
use integer_encoding::{VarIntReader, VarIntWriter};

use errors::*;
use network_proto::*;

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

pub struct TcpSodiumReader<'a> {
    dc: &'a mut DatConnection,
}

impl<'a> Read for TcpSodiumReader<'a> {

    fn read(&mut self, buf: &mut [u8]) -> ::std::io::Result<usize> {
        let len = self.dc.tcp.read(buf)?;
        if len > 0 {
            stream_xor_inplace(&mut buf[0..len], &self.dc.rx_nonce, &self.dc.key);
        }
        self.dc.rx_offset += len as u64;
        Ok(len)
    }
}

pub struct TcpSodiumWriter<'a> {
    dc: &'a mut DatConnection,
}

impl<'a> Write for TcpSodiumWriter<'a> {

    fn write(&mut self, buf: &[u8]) -> ::std::io::Result<usize> {
        //let enc = stream_xor_ic(buf, self.dc.tx_offset, &self.dc.tx_nonce, &self.dc.key);
        let enc = stream_xor(buf, &self.dc.tx_nonce, &self.dc.key);
        self.dc.tx_offset += buf.len() as u64;
        self.dc.tcp.write(&enc)
    }

    fn flush(&mut self) -> ::std::io::Result<()> {
        self.dc.tcp.flush()
    }
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
    tx_nonce: Nonce,
    tx_offset: u64,
    rx_nonce: Nonce,
    rx_offset: u64,
}

impl DatConnection {

    pub fn connect(host_port: &str, key: &[u8], live: bool) -> Result<DatConnection> {

        let timeout = Duration::new(7, 0);
        let tx_nonce = gen_nonce();
        let mut local_id = [0; 32];
        let mut rng = OsRng::new()?;
        rng.fill_bytes(&mut local_id);

        // Connect to server
        println!("Connecting to {}", host_port);
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
            tx_nonce: tx_nonce,
            tx_offset: 0,
            rx_nonce: gen_nonce(),  // dummy
            rx_offset: 0,
        };

        // Exchange register/feed
        dc.tcp.set_nodelay(true)?; // Faster handshake
        // calculate discovery key
        let mut discovery_key = [0; 32];
        let mut hash = Blake2b::new_keyed(32, key);
        hash.input(&"hypercore".as_bytes());
        hash.result(&mut discovery_key);
        // send register
        let mut register_msg = Feed::new();
        register_msg.set_discoveryKey(discovery_key.to_vec());
        register_msg.set_nonce((tx_nonce[0..24]).to_vec());
        dc.send_register(&register_msg)?;

        // read register
        let registration = dc.recv_register()?;
        if registration.get_discoveryKey()[0..32] != discovery_key[..] {
            bail!("Remote peer not sharing same discovery key");
        }
        let rn = registration.get_nonce();
        dc.rx_nonce = Nonce::from_slice(&rn).unwrap();

        // send handshake
        let mut handshake_msg = Handshake::new();
        handshake_msg.set_live(live);
        handshake_msg.set_id(local_id.to_vec());
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

        dc.tcp.set_nodelay(false)?; // Back to normal

        Ok(dc)
    }

    fn send_msg(&mut self, dnm: &DatNetMessage, is_content: bool) -> Result<()> {

        let mut tx_stream = TcpSodiumWriter { dc:  self };
        let header_int: u8 = (is_content as u8) << 4 | (msg_code(dnm) & 0x0F);
        let msg: &Message = msg_sugar(dnm);
        let total_message_size = (msg.compute_size() as usize) + 1;

        println!("SEND total_len={}  header={}  is_content={}", total_message_size, header_int, is_content);

        // send both header varints, and data
        tx_stream.write_varint(total_message_size as u64)?;
        tx_stream.write_varint(header_int as u32)?;

        match dnm {
            &DatNetMessage::Register(ref m) => m.write_to_writer(&mut tx_stream)?,
            &DatNetMessage::Handshake(ref m) => m.write_to_writer(&mut tx_stream)?,
            &DatNetMessage::Status(ref m) => m.write_to_writer(&mut tx_stream)?,
            &DatNetMessage::Have(ref m) => m.write_to_writer(&mut tx_stream)?,
            &DatNetMessage::Unhave(ref m) => m.write_to_writer(&mut tx_stream)?,
            &DatNetMessage::Want(ref m) => m.write_to_writer(&mut tx_stream)?,
            &DatNetMessage::Unwant(ref m) => m.write_to_writer(&mut tx_stream)?,
            &DatNetMessage::Request(ref m) => m.write_to_writer(&mut tx_stream)?,
            &DatNetMessage::Cancel(ref m) => m.write_to_writer(&mut tx_stream)?,
            &DatNetMessage::Data(ref m) => m.write_to_writer(&mut tx_stream)?,
        }
        Ok(())
    }

    fn recv_msg(&mut self) -> Result<(bool, DatNetMessage)> {

        let mut rx_stream = TcpSodiumReader { dc: self };
        let total_len: u64 = rx_stream.read_varint()?;
        let header: u8 = rx_stream.read_varint()?;

        let is_content = (header & (1 << 4)) != 0;

        println!("RECV total_len={}  header={}  is_content={}", total_len, header, is_content);

        if header > 0x1F {
            bail!("Invalid header received: {}", header);
        }

        let msg_len = (total_len - 1) as usize;
        let mut buf = vec![0; msg_len];
        rx_stream.read_exact(&mut buf[0..msg_len])?;

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
        Ok((is_content, dnm))
    }

    fn send_register(&mut self, reg: &Feed) -> Result<()> {

        let header_int: u8 = 0;
        let total_message_size = (reg.compute_size() as usize) + 1;

        println!("SEND total_len={}  header={}", total_message_size, header_int);

        self.tcp.write_varint(total_message_size as u64)?;
        self.tcp.write_varint(header_int as u32)?;
        reg.write_to_writer(&mut self.tcp)?;
        Ok(())
    }

    fn recv_register(&mut self) -> Result<Feed> {

        let total_len: u64 = self.tcp.read_varint()?;
        let header: u8 = self.tcp.read_varint()?;

        if header != 0 {
            bail!("Invalid register header received");
        }

        println!("RECV total_len={}  header={}", total_len, header);

        let msg_len = (total_len - 1) as usize;
        let mut buf = vec![0; msg_len];
        self.tcp.read_exact(&mut buf[0..msg_len])?;

        let reg = parse_from_bytes::<Feed>(&mut buf)?;
        Ok(reg)
    }

    pub fn receive_all(&mut self, is_content: bool) -> Result<()> {

        // Status: downloading, not uploading
        let mut sm = Info::new();
        sm.set_uploading(false);
        sm.set_downloading(true);
        self.send_msg(&DatNetMessage::Status(sm), is_content)?;

        // Want: everything
        let mut wm = Want::new();
        wm.set_start(0);
        self.send_msg(&DatNetMessage::Want(wm), is_content)?;

        // listen for Have
        let length;
        loop {
            let (was_content, msg) = self.recv_msg()?;
            if was_content != is_content{
                continue;
            }
            if let DatNetMessage::Have(have) = msg {
                length = have.get_length();
                break;
            } else {
                info!("Expected Want message, got: {:?}", &msg);
                continue;
            }
        };

        // Request / Data loop
        for i in 0..length {
            let mut rm = Request::new();
            rm.set_index(i);
            self.send_msg(&DatNetMessage::Request(rm), is_content)?;

            let (was_content, msg) = self.recv_msg()?;
            if was_content != is_content{
                info!("Expected other message channel");
            }
            if let DatNetMessage::Data(dm) = msg {
                println!("Got metadata: {}", dm.get_index());
            } else {
                info!("Expected Data message, got: {:?}", &msg);
                continue;
            }
        }

        Ok(())
    }
}
