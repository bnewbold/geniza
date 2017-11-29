
### dat-specific configuration

Default servers are specified in the `dat-swarm-defaults` module. As of
November 2017, they are:

    Domain (for mDNS, centralized DNS):
        dat.local

    DNS Servers (centralized lookup):
        discovery1.publicbits.org
        discovery2.publicbits.org

    DHT Bootstrap Servers:
        bootstrap1.publicbits.org:6881
        bootstrap2.publicbits.org:6881
        bootstrap3.publicbits.org:6881
        bootstrap4.publicbits.org:6881

### mDNS and Centralized DNS Discovery

`dns-discovery` node module.

Refresh period: 1 minute

Take the first 40 hex characters (lowercase; 20 bytes if decoded) of the
discovery key (a BLAKE2b hash) instead of a SHA1 hash (which is what bittorrent
uses). Prepend this to the domain (`dat.local` in the case of dat).

There are both SRV and TXT records. SRV is straightforward, a response like:

    0 0 44113 172.19.0.4.

TXT uses some un-documented token scheme... maybe mostly for mDNS, to update or
"unannounce" records? Eg:

    "token=kZabfUWLUw5A4E/EXM3+ka7UybMN95QJjqPk1iGmb0M=" "peers=rBMABKxR"

Example lookup:

    dig @discovery1.publicbits.org 905fd1b6504698425e8bec3dbb77d757e281d505.dat.local SRV

Note that you might need to do lookup on *both* servers? Results can be
different from either individually.

### DHT Discovery

`discovery-channel`

Refresh period: 10 minutes

