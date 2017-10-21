
use std::io::Read;
use std::net::TcpStream;
use integer_encoding::{VarIntReader, VarIntWriter, VarInt};
use std::time::Duration;
use crypto::digest::Digest;
use crypto::blake2b::Blake2b;
use rand::{Rng, OsRng};
use protobuf::Message;
use protobuf::core::parse_from_reader;
use protobuf::core::parse_from_bytes;

use errors::*;
use network_proto::*;

// TCP stream
// two varints

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

/// Represents a bi-directional connection to a network peer
///
/// Spec says nonce is 32 bytes, by dat implementation (hypercore-protocol) is 24 bytes.
pub struct DatConnection {
    nonce: [u8; 24],
    remote_nonce: [u8; 24],
    id: [u8; 32],
    stream: TcpStream,
}

impl DatConnection {

    pub fn connect(host_port: &str, key: &[u8]) -> Result<DatConnection> {

        let timeout = Duration::new(7, 0);
        let mut rng = OsRng::new()?;
        let mut nonce = [0; 24];
        rng.fill_bytes(&mut nonce);
        let mut local_id = [0; 32];
        rng.fill_bytes(&mut local_id);

        // Connect to server
        println!("Connecting to {}", host_port);
        // TODO: timeout on connect (socketaddr dance)
        let stream = TcpStream::connect(host_port)?;
        stream.set_read_timeout(Some(timeout))?;
        stream.set_write_timeout(Some(timeout))?;

        let mut dc = DatConnection {
            nonce: nonce,
            remote_nonce: [0; 24],
            id: local_id,
            stream,
        };

        // Exchange register/feed
        dc.stream.set_nodelay(true)?; // Faster handshake
        // calculate discovery key
        let mut discovery_key = [0; 32];
        let mut hash = Blake2b::new_keyed(32, key);
        hash.input(&"hypercore".as_bytes());
        hash.result(&mut discovery_key);
        // send register
        let mut register_msg = Feed::new();
        register_msg.set_discoveryKey(discovery_key.to_vec());
        register_msg.set_nonce(nonce.to_vec());
        dc.send_msg(false, &DatNetMessage::Register(register_msg))?;

        // read register
        let (was_content, msg) = dc.recv_msg()?;
        if was_content {
            bail!("Expected metadata msg, not content");
        }
        if let DatNetMessage::Register(registration) = msg {
            if registration.get_discoveryKey()[0..32] != discovery_key[..] {
                bail!("Remote peer not sharing same discovery key");
            }
            // TODO: more satisfying way to do this transfer
            let rn = registration.get_nonce();
            for i in 0..24 {
                dc.remote_nonce[i] = rn[i];
            }
        } else {
            bail!("Expected Registration message, got something else");
        }

        // send handshake
        // read handshake

        dc.stream.set_nodelay(false)?; // Back to normal

        Ok(dc)
    }

    fn send_msg(&mut self, is_content: bool, dnm: &DatNetMessage) -> Result<()> {

        let header_int: u8 = (is_content as u8) << 4 | (msg_code(dnm) & 0x0F);
        let msg: &Message = msg_sugar(dnm);
        let total_message_size = (msg.compute_size() as usize) + header_int.required_space();

        println!("SEND total_len={}  header={}  is_content={}", total_message_size, header_int, is_content);

        // send both header varints, and data
        self.stream.write_varint(total_message_size)?;
        self.stream.write_varint(header_int)?;
        msg.write_to_writer(&mut self.stream)?;

        Ok(())
    }

    fn recv_msg(&mut self) -> Result<(bool, DatNetMessage)> {
        let total_len: u64 = self.stream.read_varint()?;
        let header: u8 = self.stream.read_varint()?;

        let is_content = (header & (1 << 4)) != 0;

        println!("RECV total_len={}  header={}  is_content={}", total_len, header, is_content);

        // XXX: replace with coded, buffered streams
        let mut buf = [0; 1024];
        let msg_len = (total_len - 1) as usize;
        let len = self.stream.read(&mut buf[0..msg_len])?;
        println!("raw read: {} first={}", len, buf[0]);

        let dnm = match header & 0x0F {
            0 => DatNetMessage::Register(parse_from_bytes::<Feed>(&mut buf[0..msg_len])?),
            1 => DatNetMessage::Handshake(parse_from_reader::<Handshake>(&mut self.stream)?),
            2 => DatNetMessage::Status(parse_from_reader::<Info>(&mut self.stream)?),
            3 => DatNetMessage::Have(parse_from_reader::<Have>(&mut self.stream)?),
            4 => DatNetMessage::Unhave(parse_from_reader::<Unhave>(&mut self.stream)?),
            5 => DatNetMessage::Want(parse_from_reader::<Want>(&mut self.stream)?),
            6 => DatNetMessage::Unwant(parse_from_reader::<Unwant>(&mut self.stream)?),
            7 => DatNetMessage::Request(parse_from_reader::<Request>(&mut self.stream)?),
            8 => DatNetMessage::Cancel(parse_from_reader::<Cancel>(&mut self.stream)?),
            9 => DatNetMessage::Data(parse_from_reader::<Data>(&mut self.stream)?),
            _ => bail!("Unimplemented message type received"),
        };
        Ok((is_content, dnm))
    }

    fn receive_all(&self) -> Result<()> {

        // Status: downloading, not uploading
        // Have: nothing
        // Want: everything
        unimplemented!();
    }
}
