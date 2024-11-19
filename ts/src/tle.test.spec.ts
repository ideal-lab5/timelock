import { expect, test, vi } from 'vitest'
import {
  timelockEncrypt,
  timelockDecrypt,
  forceDecrypt,
  IdealNetworkIdentityHandler,
} from './tle'

test('it should timelock encrypt a message for an IDN style network when params are valid', async () => {
  const seed = 'seed'
  const roundNumber = 123
  const message = 'Hello, world!'
  let idnBeaconPK =
    '471ba929a4e2ef2790fb5f2a65ebe86598a28cbb8a58e49c6cc7292cf40cecbdf10152394ba938367ded5355ae373e01a99567467bc816864774e84b984fc16e2ae2232be6481cd4db0e378e1d6b0c2265d2aa8e0fa4e2c76958ce9f12df8e0134c431c181308a68b94b9cfba5176c3a8dd22ead9a68a077ecce7facfe4adb9e0e0a71c94a0c436d8049b03fa5352301'
  const publicKey = Uint8Array.from(Buffer.from(idnBeaconPK, 'hex'))
  
  await timelockEncrypt(
    new TextEncoder().encode(message),
    roundNumber,
    IdealNetworkIdentityHandler,
    publicKey,
    seed
  ).then((result) => {
    expect(result).toEqual('tle-ciphertext')
  })
})

test('it should timelock decrypt a message', async () => {

  const ciphertext = new Uint8Array(1);
  const signature = new Uint8Array(2);
  const result = await timelockDecrypt(ciphertext, signature);

  expect(result).toEqual('tld-plaintext')
})

test('it should decrypt a message on demand if the user knows the secret', async () => {
  const plaintext = 'plaintext'

  const secret = "shhh, it's a secret"
  const ciphertext = new Uint8Array(1)

  const result = await forceDecrypt(ciphertext, secret)
  expect(result).toEqual(plaintext)
})
