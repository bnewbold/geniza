#!/usr/bin/env python3

"""This script generates empty.sleep"""

# Create file in binary mode
f = open('empty.sleep', 'wb')

# Move to begining, set size
f.seek(0)
f.truncate(32)

# Write header, with 0xFF as the dat file type
f.write(b'\x05\x02\x57\xFF')

# Version 0
f.write(b'\x00')

# One-byte entry size
f.write(b'\x00\x01')

# No Algorithm name
f.write(b'\x00')
for i in range(24):
    f.write(b'\x00')

assert(f.tell() == 32)
f.close()
