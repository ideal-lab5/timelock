
from timelock import Timelock

print("Timelock Encryption in Python!")
# Setup encryption input
# The drand quicknet public key
pk_hex = "83cf0f2896adee7eb8b5f01fcad3912212c437e0073e911fb90022d3e760183c8c4b450b6a0a6c3ac6a5776a2d1064510d1fec758c921cc22b0e17e63aaf4bcb5ed66304de9cf809bd274ca73bab4af5a6e9c76a4bc09e76eae8991ef5ece45a"
timelock = Timelock(pk_hex)
# An ephemeral secret key
sk = bytearray([0x01, 0x02, 0x03, 0x04] * 8)
# A "future" round number
round_number = 1000
# The message to encrypt
plaintext = "Hello, Timelock!"
# timelock encrypt
ct = timelock.tle(round_number, plaintext, sk)

print("Timelock Encryption: Success!")

# get a signature at some point in the future
signature_hex = "b44679b9a59af2ec876b1a6b1ad52ea9b1615fc3982b19576350f93447cb1125e342b73a8dd2bacbe47e4b6b63ed5e39"
sig = bytearray.fromhex(signature_hex)

# and finally decrypt the message
maybe_plaintext = timelock.tld(ct, sig)
maybe_plaintext = maybe_plaintext.decode("utf-8")
assert plaintext == maybe_plaintext

print("Timelock decryption successful!")