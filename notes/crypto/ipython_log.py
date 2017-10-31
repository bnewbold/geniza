# IPython log file

import pysodium
import base64
import pyblake2

pub_key = "yx6NZfPGJC25C/2ZrMr0eTDqP/bIUvtQyqpUjJzcuLc="
base64.b16encode(base64.b64decode(pub_key)).lower()
pub_key = base64.b64decode(pub_key)
secret_key = "qzvSkJMQeWw2rGb4W+xbGNTgk5dEkiViOtq5niOkEiTLHo1l88YkLbkL/ZmsyvR5MOo/9shS+1DKqlSMnNy4tw=="
base64.b16encode(base64.b64decode(secret_key)).lower()
secret_key = base64.b64decode(secret_key)

h = pyblake2.blake2b(data=b"hypercore", key=pub_key, digest_size=32)
h.hexdigest()

base64.b16encode(bytearray([43, 235, 230, 134, 54, 24, 197, 134, 8, 109, 37, 32, 170, 13, 19, 218, 102, 107, 0, 46, 90, 210, 177, 98, 35, 161, 91, 193, 85, 134, 241, 228, 0, 0, 0, 0, 0, 0, 0, 204])).lower()

h = pyblake2.blake2b(digest_size=32)
h.update([2])
h.update(bytearray([0x02]))
h.update(base64.b16decode("67179c243b387b7c7c420bd7bdc69d35c061b979bb365f9cc07b4527eeddeefa".upper()))
h.update(bytearray([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]))
h.update(bytearray([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x42]))
h.hexdigest()

base64.b16encode(pysodium.crypto_sign_detached(h.digest(), secret_key)).lower()

h = pyblake2.blake2b(digest_size=32)
h.update(bytearray([0x02]))
h.update(base64.b16decode("ab27d45f509274ce0d08f4f09ba2d0e0d8df61a0c2a78932e81b5ef26ef398df".upper()))
h.update(bytearray([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]))
h.update(bytearray([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01]))
h.hexdigest()

sk = base64.b16decode("53729c0311846cca9cc0eded07aaf9e6689705b6a0b1bb8c3a2a839b72fda3839718a1ff1c4ca79feac551c0c7212a65e4091278ec886b88be01ee4039682238".upper())
base64.b16encode(pysodium.crypto_sign_detached(h.digest(), sk)).lower()

lh = pyblake2.blake2b(digest_size=32)
lh.update(bytearray([0x00]))
lh.update(bytearray([0, 0, 0, 0, 0, 0, 0, 1]))
hex(ord('a'))
lh.update(bytearray([0x61]))
lh.hexdigest()
