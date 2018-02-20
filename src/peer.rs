
use errors::*;
use std::thread;
use std::net::ToSocketAddrs;
use std::fmt::Display;
use std::time::Duration;
use protocol::{DatConnection, DatNetMessage};
use network_msgs::*;
use sodiumoxide::crypto::stream::Key;
use make_discovery_key;
use chan;

/// Wraps a low-level DatConnection in a thread (or two). Contains very little context about
/// itself.
pub struct DatPeerThread {
    pub handle: u64,
    feeds: Vec<(u8, Key)>,
    outbound_chan: chan::Sender<(DatNetMessage, u8)>,
}

pub struct PeerMsg {
    pub peer_handle: u64,
    pub feed_index: u8,
    pub msg: DatNetMessage,
}

/// This is what the "receive" loop does: simply blocking reads on the TCP socket, passing any
/// received messages into a channel back to the worker thread.
fn receiver_loop(mut dc: DatConnection, peer_rx: chan::Sender<Result<(DatNetMessage, u8)>>) {
    loop {
        match dc.recv_msg() {
            Ok((msg, feed_index)) => {
                peer_rx.send(Ok((msg, feed_index)));
            },
            Err(e) => {
                // XXX: check if this was due to socket closing cleanly, in which case don't pass
                // error along
                peer_rx.send(Err(e));
                return
            },
        }
    }
}

/// Worker thread. After initializing the connection, loops endlessly. Looks for outgoing PeerMsg
/// on the command channel, and sends these directly (blocking). Also looks for raw received
/// messages (via a spawned receiver thread), and enhances these with extra context then passes
/// upwards on the unified peer message channel.
fn worker_thread(mut dc: DatConnection, handle: u64, outbound_chan: chan::Receiver<(DatNetMessage, u8)>, unified_chan: chan::Sender<Result<PeerMsg>>) {

    dc.tcp.set_write_timeout(Some(Duration::new(2, 0))).unwrap();

    let rx_dc = dc.clone();
    let (receiver_chan, raw_peer_rx) = chan::async();
    thread::spawn(move || {
        receiver_loop(rx_dc, receiver_chan);
    });

    loop {
        chan_select!{
            outbound_chan.recv() -> val => {
                if let Some((msg, feed_index)) = val {
                    match dc.send_msg(&msg, feed_index) {
                        Ok(_) => {},
                        Err(e) => {
                            // TODO: error chain!
                            unified_chan.send(Err(e));
                            return
                        }
                    }
                };
            },
            raw_peer_rx.recv() -> val => {
                match val {
                    Some(Ok((msg, feed_index))) => {
                        // do mapping between feed index and feed pubkey here
                        let pm = PeerMsg {
                            peer_handle: handle,
                            feed_index: feed_index,
                            msg,
                        };
                        unified_chan.send(Ok(pm));
                    },
                    Some(Err(err)) => {
                        println!("remote socket error: {:?}", err);
                        // XXX: Need to send something so we know to close
                        unified_chan.send(Err(err));
                        dc.close();
                        return;
                    },
                    None => {
                        println!("remote socket closed");
                        // XXX: Need to send something so we know to close
                        //unified_chan.send(Err(err));
                        dc.close();
                        return;
                    }
                };
            }
        };
    }
}

impl DatPeerThread {

    pub fn connect<A: ToSocketAddrs + Display>(addr: A, feed_key: Key, handle: u64, is_live: bool, local_id: Option<&[u8]>, unified_chan: chan::Sender<Result<PeerMsg>>) -> Result<DatPeerThread> {

        let addr = addr.to_socket_addrs().unwrap().nth(0).unwrap();
        let (outbound_chan, tx_chan) = chan::async();
        let feed_key2 = feed_key.clone();

        // Do an ugly little dance to copy local_id across thread barrier
        let mut id_buf = [0; 32];
        let local_id = match local_id {
            None => None,
            Some(val) => {
                id_buf.copy_from_slice(val);
                Some(id_buf)
            },
        };
        thread::spawn(move || {

            let local_id = match local_id {
                None => None,
                Some(val) => {
                    Some(&id_buf[..])
                },
            };
            let dc = match DatConnection::connect(addr, &feed_key, is_live, local_id) {
                Ok(c) => c,
                Err(e) => {
                    // TODO: error chain!
                    unified_chan.send(Err(e));
                    return;
                },
            };

            worker_thread(dc, handle, tx_chan, unified_chan);
        });

        let dp = DatPeerThread {
            handle,
            outbound_chan,
            feeds: vec![(0, feed_key2)],
        };
        Ok(dp)
    }

    pub fn send(&mut self, net_msg: DatNetMessage, feed_index: u8) -> Result<()> {
        self.outbound_chan.send((net_msg, feed_index));
        Ok(())
    }

    pub fn add_feed(&mut self, key: &Key) -> Result<()> {

        let key_bytes = &key[0..32];
        let key = Key::from_slice(key_bytes).unwrap();

        for k in self.feeds.iter() {
            if (*k).1 == key {
                warn!("tried to add existing feed/key on a DatPeerThread connection");
                return Ok(())
            }
        }

        let index = self.feeds.len();
        assert!(index < 256);
        let discovery_key = make_discovery_key(key_bytes);;

        // Send (encrypted) Feed message for data feed
        let mut feed_msg = Feed::new();
        feed_msg.set_discoveryKey(discovery_key.to_vec());
        self.outbound_chan.send((DatNetMessage::Feed(feed_msg), index as u8));

        self.feeds.push((index as u8, key.clone()));
        Ok(())
    }

    pub fn close(&mut self) -> Result<()> {
        unimplemented!();
    }
}

