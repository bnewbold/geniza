
peers wrap connections with given clients (any transport type). each peer can
be talking about multiple feeds. state consists of which feeds, what the remote
has (feed, bitmap), and what chunks (feed, index) have been requested
("in-flight"). connection logic runs in threads, passes back and forth entire
messages in channels.

    ConnCommand
        Send(DatNetMessage)
        Terminate()

    ConnReply
        Received(DatNetMessage)
        Disconnected()

A synchronizer event loop has local feeds (storage) and a bunch of peers. Extra
state includes mode configuration, feeds, "wanted" bitmap for each feed.
Callbacks for certain feed events allow for extension (eg, hyperdrive stuff).
Needs a timer event. 

Use chan and chan-signal for now (MPSC style)... tokio will be the future but
seems too unstable for now. mio is too low-level to use on it's own (eg, no
timers, no threads). Use two threads per peer: a "main" thread that does
connection setup, `chan_select`, and blocking writes, and a "reader" thread
that does blocking reads of entire messages (writing either into the main
thread or skipping straight to the output channel). Got this pattern from the
`rusty_torrent` package. Need to set some write timeout. Synchronizer handles
cleanup if peer isn't replying/heartbeating.

Probably want a dedicated discovery thread/worker in the future, but for now
just do it synchronously and connect to up to N peers to start.

Synchronize will select on:

    signal
        graceful shutdown
    unified_inbound
        feed: depends on state; ignore for now
        handshake: error, close connection?
        have, info: update PeerState bitmap
        unhave, info, want, unwant: ignore for now
        request: depends on state; read from storage
        cancel: ignore for now
        data: write to storage, clear pending; send new request
    tick
        check peer states, kill if too slow
        dump status to command line

Synchronize needs state:
- "tried" peers, so we don't retry in a loop if they fail

Synchronize options:
- whether or not to hook signals

Where does drive logic live? Plugin, via callback, to Synchronize? Let's just
fold it in to synchronizer for now, as a mode flag.
