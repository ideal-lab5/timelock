/*
 * Copyright 2024 by Ideal Labs, LLC
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

import { expect, describe } from '@jest/globals'
import { Timelock } from './timelock'
import { IdealNetworkIdentityBuilder } from './interfaces/IDNIdentityBuilder'
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

    const expectedResult = new Uint8Array(1);

    const result = await instance.encrypt(
      encodedMessage,
      42,
      IdealNetworkIdentityBuilder,
      beaconPublicKey,
      ephemeralSecretKey
    )

    expect(result).toStrictEqual(expectedResult)
    expect(build_encoded_commitment).toHaveBeenCalledWith(42, 0)
    expect(tle).toHaveBeenCalledWith(
      0,
      encodedMessage,
      ephemeralSecretKey,
      beaconPublicKey
    )
  })

  test('should decrypt data using tld', async () => {
    const instance = await Timelock.build()
    const ciphertext = new Uint8Array([10, 11, 12])
    const signature = new Uint8Array([13, 14, 15])

    const expectedResult = new Uint8Array(2);
    const result = await instance.decrypt(ciphertext, signature)

    expect(tld).toHaveBeenCalledWith(ciphertext, signature)
    expect(result).toStrictEqual(expectedResult)
  })

  test('should force decrypt data using decrypt', async () => {
    const instance = await Timelock.build()
    const ciphertext = new Uint8Array([16, 17, 18])
    const ephemeralSecretKey = new Uint8Array([19, 20, 21])

    const expectedResult = new Uint8Array(3)
    const result = await instance.forceDecrypt(ciphertext, ephemeralSecretKey)

    expect(decrypt).toHaveBeenCalledWith(ciphertext, ephemeralSecretKey)
    expect(result).toStrictEqual(expectedResult)
  })
})
