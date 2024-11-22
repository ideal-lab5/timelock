import { expect, describe } from '@jest/globals'
import { IdealNetworkIdentityHandler, Timelock } from './timelock'
import init, {
  build_encoded_commitment,
  tle,
  tld,
  decrypt,
} from 'timelock-wasm-wrapper'

jest.mock('timelock-wasm-wrapper')

describe('Timelock Class', () => {
  beforeEach(() => {
    jest.clearAllMocks()
  })

  test('should initialize WASM and create an instance', async () => {
    const instance = await Timelock.build()
    expect(init).toHaveBeenCalledTimes(1)
    expect(instance).toBeInstanceOf(Timelock)
  })

  test('should encrypt data using tle', async () => {
    const instance = await Timelock.build()
    const encodedMessage = new Uint8Array([1, 2, 3])
    const beaconPublicKey = new Uint8Array([4, 5, 6])
    const ephemeralSecretKey = new Uint8Array([7, 8, 9])

    const result = await instance.encrypt(
      encodedMessage,
      42,
      IdealNetworkIdentityHandler,
      beaconPublicKey,
      ephemeralSecretKey
    )

    expect(result).toStrictEqual(new Uint8Array(1))
    expect(build_encoded_commitment).toHaveBeenCalledWith(42, 0)
    expect(tle).toHaveBeenCalledWith(
      new Uint8Array(0),
      encodedMessage,
      ephemeralSecretKey,
      beaconPublicKey
    )
  })

  // test('should decrypt data using tld', async () => {
  //   const instance = await Timelock.build()
  //   const ciphertext = new Uint8Array([10, 11, 12])
  //   const signature = new Uint8Array([13, 14, 15])

  //   const result = await instance.decrypt(ciphertext, signature)

  //   expect(tld).toHaveBeenCalledWith(ciphertext, signature)
  //   expect(result).toBe([2])
  // })

  // test('should force decrypt data using decrypt', async () => {
  //   const instance = await Timelock.build()
  //   const ciphertext = new Uint8Array([16, 17, 18])
  //   const ephemeralSecretKey = new Uint8Array([19, 20, 21])

  //   const result = await instance.forceDecrypt(ciphertext, ephemeralSecretKey)

  //   expect(decrypt).toHaveBeenCalledWith(ciphertext, ephemeralSecretKey)
  //   expect(result).toBe([3])
  // })
})
