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
import { Result, SupportedCurve, Timelock, u8a } from './timelock'
import init, {
  tle,
  tld,
  decrypt,
} from '@ideallabs/timelock_wasm_wrapper'
import { DrandIdentityBuilder } from '../dist'

jest.mock('timelock-wasm-wrapper')

describe('Timelock Encryption', () => {
  beforeEach(() => {
    jest.clearAllMocks()
  })

  test('should initialize WASM and create an instance', async () => {
    const instance = await Timelock.build(SupportedCurve.BLS12_377)
    expect(init).toHaveBeenCalledTimes(1)
    expect(instance).toBeInstanceOf(Timelock)
  })

  test('should encrypt data using tle', async () => {
    const instance = await Timelock.build(SupportedCurve.BLS12_377)
    const encodedMessage = new Uint8Array([1, 2, 3])

    const beaconPublicKeyHex = 'abcdef'
    const ephemeralSecretKeyHex = '123456'

    const beaconPublicKey = u8a(beaconPublicKeyHex)
    const ephemeralSecretKey = u8a(ephemeralSecretKeyHex)

    const expectedResult = new Uint8Array(1);

    const result = await instance.encrypt(
      encodedMessage,
      42,
      DrandIdentityBuilder,
      beaconPublicKeyHex,
      ephemeralSecretKeyHex,
    )

    expect(result).toStrictEqual(expectedResult)
    expect(tle).toHaveBeenCalledWith(
      0,
      encodedMessage,
      ephemeralSecretKey,
      beaconPublicKey,
      SupportedCurve.BLS12_377
    )
  })

  test('should decrypt data using tld', async () => {
    const instance = await Timelock.build(SupportedCurve.BLS12_377)
    const ciphertext = new Uint8Array([10, 11, 12])
    const signatureHex = '123456'
    const signature = u8a(signatureHex)

    const expectedResult = new Uint8Array(2);
    const result = await instance.decrypt(ciphertext, signatureHex)

    expect(result).toStrictEqual(expectedResult)
    expect(tld).toHaveBeenCalledWith(ciphertext, signature, SupportedCurve.BLS12_377)
  })

  test('should force decrypt data using decrypt', async () => {
    const instance = await Timelock.build(SupportedCurve.BLS12_377)
    const ciphertext = new Uint8Array([16, 17, 18])
    const ephemeralSecretKey = 'qwerty'
    const esk = u8a(ephemeralSecretKey);

    const expectedResult = new Uint8Array(3)
    const result = await instance.forceDecrypt(ciphertext, ephemeralSecretKey)

    expect(result).toStrictEqual(expectedResult)

    expect(decrypt).toHaveBeenCalledWith(ciphertext, esk, SupportedCurve.BLS12_377)
  })
})
