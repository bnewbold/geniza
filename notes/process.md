
This is an ordered list of testable steps to get to a minimal dat client in
just about any language.

sleep
  read/write headers
  read/write individual elements as raw bytes
  read/write contiguous batches

registers
  read elements by index
  verify signature by index (not leaves)
  append elements

drive
  data register to a single file
  single file to a data register
  print metadata tree ("ls")
  create metadata tree for a directory
  directory to metadata+data registers
  registers to directory

sync
  send/receive messages to a known host
  pull register from a known host
  wrapper command




Shortcuts:
  key/value store instead of SLEEP files
  pull/read/to-file only
  don't verify any hashes (!)
