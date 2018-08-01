
## HOWTO: clone drive from scratch

- create empty metadata register (on disk) from key (to ensure we can?)
- do discovery to find peers
- from first peer, discover data key
- create data register
- create drive from empty registers
- create synchronizer from all the above
- ... off it goes

## HOWTO: sync existing drive

- open the drive (on disk)
- create synchronizer from drive
- synchronizer does discovery of peers using metadata key (based on config)
- ... off it goes
