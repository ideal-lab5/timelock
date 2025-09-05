/*
 * Copyright 2025 by Ideal Labs, LLC
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

import { expect, describe, test } from '@jest/globals'
import { Result, Timelock, TimelockErrors } from './timelock'
import init, {
  tle,
  tld,
  decrypt,
} from '@ideallabs/timelock_wasm_wrapper'
import { DrandIdentityBuilder } from './interfaces/DrandIdentityBuilder'

// jest.mock('timelock-wasm-wrapper')

jest.mock('@ideallabs/timelock_wasm_wrapper', () => ({
  __esModule: true,
  default: jest.fn().mockResolvedValue(undefined),
  tle: jest.fn().mockReturnValue(1),
  tld: jest.fn().mockReturnValue(2),
  decrypt: jest.fn().mockReturnValue(3),
}));

describe('Timelock Encryption', () => {
  beforeEach(() => {
    jest.clearAllMocks()
    jest.useFakeTimers()
  })

  afterEach(() => {
    jest.clearAllTimers()
  })

  test('it should initialize WASM and create an instance', async () => {
    const instance = await Timelock.build()
    expect(init).toHaveBeenCalledTimes(1)
    expect(instance).toBeInstanceOf(Timelock)
  })

  test('it should encrypt data using tle with right-sized inputs', async () => {
    const instance = await Timelock.build()
    const encodedMessage = new Uint8Array([1, 2, 3])

    let beaconPublicKey = new Uint8Array(96)
    beaconPublicKey.fill(1)

    let ephemeralSecretKey = new Uint8Array(32)
    ephemeralSecretKey.fill(2)

    const expectedResult = new Uint8Array(1);

    const roundNumber = 10000000

    const result = await instance.encrypt(
      encodedMessage,
      roundNumber,
      ephemeralSecretKey,
      beaconPublicKey,
    )

    const expectedId = await DrandIdentityBuilder.build(roundNumber);
    expect(result).toStrictEqual(expectedResult)
    expect(tle).toHaveBeenCalledWith(
      expectedId,
      encodedMessage,
      ephemeralSecretKey,
      beaconPublicKey,

    )
  })

  test('it should handle timelock encryption errors', async () => {
    const timelock = await Timelock.build()
    const timelockEncryptSpy = jest.spyOn(require('@ideallabs/timelock_wasm_wrapper'), 'tle')
    timelockEncryptSpy.mockImplementationOnce(() => { throw new Error("TLE MOCK FAILURE.") })
    // tle.mockImplementation();
    let beaconPublicKey = new Uint8Array(96)
    beaconPublicKey.fill(1)

    let ephemeralSecretKey = new Uint8Array(32)
    ephemeralSecretKey.fill(2)
    const result = await timelock.encrypt(
      new Uint8Array(32),
      100,
      ephemeralSecretKey,
      beaconPublicKey
    )

    expect(result).toBeInstanceOf(Error)
    expect((result as Error).message).toBe('TLE MOCK FAILURE.')
  })

  test('it should throw and error on encrypt with wrong-sized input', async () => {
    const instance = await Timelock.build()
    const encodedMessage = new Uint8Array([1, 2, 3])


    // right sized 
    let beaconPublicKey = new Uint8Array(96)
    beaconPublicKey.fill(1)
    let ephemeralSecretKey = new Uint8Array(32)
    ephemeralSecretKey.fill(2)
    // too small
    let tooSmall_beaconPublicKey = new Uint8Array(95)
    tooSmall_beaconPublicKey.fill(1)
    let tooSmall_ephemeralSecretKey = new Uint8Array(31)
    tooSmall_ephemeralSecretKey.fill(2)
    // too large
    let tooLarge_beaconPublicKey = new Uint8Array(97)
    tooLarge_beaconPublicKey.fill(1)
    let tooLarge_ephemeralSecretKey = new Uint8Array(33)
    tooLarge_ephemeralSecretKey.fill(2)

    const roundNumber = 10000000
    // esk errors
    // too small esk returns error
    let result = await instance.encrypt(
      encodedMessage, roundNumber, tooSmall_ephemeralSecretKey, beaconPublicKey)
    expect(result).toStrictEqual(new Error(TimelockErrors.INVALID_SECRET_KEY_SIZE));
    // too large esk returns error
    result = await instance.encrypt(
      encodedMessage, roundNumber, tooLarge_ephemeralSecretKey, beaconPublicKey)
    expect(result).toStrictEqual(new Error(TimelockErrors.INVALID_SECRET_KEY_SIZE));
    // pubkey errors
    // too small esk returns error
    result = await instance.encrypt(
      encodedMessage, roundNumber, ephemeralSecretKey, tooSmall_beaconPublicKey)
    expect(result).toStrictEqual(new Error(TimelockErrors.INVALID_PUBLIC_KEY_SIZE));
    // too large esk returns error
    result = await instance.encrypt(
      encodedMessage, roundNumber, ephemeralSecretKey, tooLarge_beaconPublicKey)
    expect(result).toStrictEqual(new Error(TimelockErrors.INVALID_PUBLIC_KEY_SIZE));

    // zero round number
    result = await instance.encrypt(
      encodedMessage, 0, ephemeralSecretKey, beaconPublicKey)
    expect(result).toStrictEqual(new Error(TimelockErrors.INVALID_ROUND_NUMBER));
    // negative round number
    result = await instance.encrypt(
      encodedMessage, -1, ephemeralSecretKey, beaconPublicKey)
    expect(result).toStrictEqual(new Error(TimelockErrors.INVALID_ROUND_NUMBER));
  })

  test('it should decrypt data using tld', async () => {
    const instance = await Timelock.build()
    const ciphertext = new Uint8Array([10, 11, 12])
    let signature = new Uint8Array(48)
    signature.fill(1)

    const expectedResult = new Uint8Array(2);
    const result = await instance.decrypt(ciphertext, signature)

    expect(result).toStrictEqual(expectedResult)
    expect(tld).toHaveBeenCalledWith(ciphertext, signature)
  })
  
  test('it should handle timelock deryption errors', async () => {
    const timelock = await Timelock.build()
    const timelockDecryptSpy = jest.spyOn(require('@ideallabs/timelock_wasm_wrapper'), 'tld')
    timelockDecryptSpy.mockImplementationOnce(() => { throw new Error("TLD MOCK FAILURE.") })

    const result = await timelock.decrypt(
      new Uint8Array(32),
      new Uint8Array(48),
    )

    expect(result).toBeInstanceOf(Error)
    expect((result as Error).message).toBe('TLD MOCK FAILURE.')
  })

  test('it should not decrypt with wrong sized inputs', async () => {
    const instance = await Timelock.build()
    const ciphertext = new Uint8Array([10, 11, 12])
    let smallSignature = new Uint8Array(31)
    smallSignature.fill(1)

    let largeSignature = new Uint8Array(31)
    largeSignature.fill(1)

    // too small
    let result = await instance.decrypt(ciphertext, smallSignature)
    expect(result).toStrictEqual(new Error(TimelockErrors.INVALID_SIGNATURE_SIZE));
    // too large
    result = await instance.decrypt(ciphertext, largeSignature)
    expect(result).toStrictEqual(new Error(TimelockErrors.INVALID_SIGNATURE_SIZE));
  })


  test('it should force decrypt data using decrypt', async () => {
    const instance = await Timelock.build()
    const ciphertext = new Uint8Array([16, 17, 18])
    let ephemeralSecretKey = new Uint8Array(32)
    ephemeralSecretKey.fill(1)


    const expectedResult = new Uint8Array(3)
    const result = await instance.earlyDecrypt(ciphertext, ephemeralSecretKey)

    expect(result).toStrictEqual(expectedResult)

    expect(decrypt).toHaveBeenCalledWith(ciphertext, ephemeralSecretKey)
  })

  test('it should not early decrypt data with wrong sized inputs', async () => {
    const instance = await Timelock.build()
    const ciphertext = new Uint8Array([16, 17, 18])
    let smallSignature = new Uint8Array(31)
    smallSignature.fill(1)

    let largeSignature = new Uint8Array(31)
    largeSignature.fill(1)

    let result = await instance.earlyDecrypt(ciphertext, smallSignature)
    expect(result).toStrictEqual(new Error(TimelockErrors.INVALID_SECRET_KEY_SIZE))

    result = await instance.earlyDecrypt(ciphertext, largeSignature)
    expect(result).toStrictEqual(new Error(TimelockErrors.INVALID_SECRET_KEY_SIZE))
  })
})
