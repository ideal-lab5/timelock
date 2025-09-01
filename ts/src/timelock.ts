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

/** Timelock Encryption TS Wrapper
 * This lib provides a typescript wrapper around the wasm-pack output of the timelock encryption library
*/

import init, {
  tle,
  tld,
  decrypt,
} from '@ideallabs/timelock_wasm_wrapper'
import { IdentityBuilder } from './interfaces/IIdentityBuilder'

/**
 * Critical runtime errors that can be encountered in the Timelock class
 */
export enum TimelockErrors {
  ERR_UNEXPECTED_TYPE = "The wasm returned something that could not be converted to a UInt8Array."
}

/**
 * A wrapper type to handle generic results 
 */
export type Result<T> = T | Error

function ok<T>(data: T | null): Result<T> {
  return data
}

function error<T>(message: string): Result<T> {
  return new Error(message)
}

/**
 * The Timelock class handles initialization of the WASM modules required to use the Timelock library
 * from web based contexts. It gracefully ensures that the WASM is available before attempting to call the respective functions.
 */
export class Timelock {
  /**
   * Indicates if the wasm has been initialized or not
   */
  private wasmReady: boolean

  /**
   * A private constructor to enforce usage of `build`
   */
  private constructor() {
    this.wasmReady = false
  }

  /**
   * Loads the wasm and constructs a new Timelock instance
   * @returns A Timelock instance
   */
  public static async build() {
    await init()
    return new Timelock()
  }

  /**
   * Timelock Encryption: Encrypt the message for the given round of the randomness beacon.
   *
   * @param encodedMessage: The message to encrypt, encoded as a Uint8Array
   * @param roundNumber: The round of the protocol when the message can be decrypted
   * @param identityBuilder: An IdentityBuilder implementation
   * @param beaconPublicKeyHex: The hex-encoded public key of the randomness beacon
   * @param ephemeralSecretKeyHex: A hex-encoded ephemeral secret key passed to AES-GCM
   * @returns The timelocked ciphertext if successful, otherwise an error message.
   */
  public async encrypt(
    encodedMessage: Uint8Array,
    roundNumber: number,
    identityBuilder: IdentityBuilder<number>,
    beaconPublicKeyHex: string,
    ephemeralSecretKeyHex: string
  ): Promise<Result<Uint8Array>> {
    try {
      await this.checkWasm()
      const beaconPublicKey = u8a(beaconPublicKeyHex)
      const ephemeralSecretKey = u8a(ephemeralSecretKeyHex)
      const id = await identityBuilder.build(roundNumber)
      const ciphertext = tle(id, encodedMessage, ephemeralSecretKey, beaconPublicKey)
      const result = new Uint8Array(ciphertext)
      return ok(result)
    } catch (err) {
      return error(err.message)
    }
  }

  /**
   * Timelock decryption: Decrypt the ciphertext using a pulse from the beacon produced at the given block
   * 
   * @param ciphertext: Ciphertext to be decrypted
   * @param blockNumber: Block number that has the signature for decryption
   * @returns: The decrypted message if successful, otherwise an error message.
   */
  public async decrypt(
    ciphertext: Uint8Array,
    signatureHex: string
  ): Promise<Result<Uint8Array>> {
    try {
      await this.checkWasm()
      const signature = u8a(signatureHex)
      return ok(new Uint8Array(tld(ciphertext, signature)))
    } catch (err) {
      return error(err.message)
    }
  }

  /**
   * Decrypt a ciphertext early using the cipher used when encrypting if you know the seed
   * 
   * @param ciphertext The ciphertext to decrypt
   * @param ephemeralSecretKey: An ephemeral secret key (32-bytes)
   * @returns The plaintext
   */
  public async forceDecrypt(
    ciphertext: Uint8Array,
    ephemeralSecretKeyHex: string
  ): Promise<Result<Uint8Array>> {
    try {
      await this.checkWasm()
      const ephemeralSecretKey = u8a(ephemeralSecretKeyHex)
      return ok(new Uint8Array(decrypt(ciphertext, ephemeralSecretKey)))
    } catch (err) {
      return error(err.message)
    }
  }

  /**
   * Check if the wasm has been initialized.
   * If it hasn't, gracefully load it and continue, else throw an error if it is unavailable
   */
  private async checkWasm() {
    if (!this.wasmReady) {
      try {
        await init()
        this.wasmReady = true
      } catch (err) {
        throw new Error("Failed to initialize WASM " + err.message)
      }
    }
  }
}

/**
 * Converts the hex-encoded string to a Uint8Array
 * @param hex A hex-encoded string
 * @returns A Uint8Array
 */
export function u8a(hexString: string): Uint8Array {
  const length = hexString.length;
  if (length % 2 !== 0) {
    throw new Error("Invalid hex string: Length must be even.");
  }

  const bytes = new Uint8Array(length / 2);
  for (let i = 0; i < length; i += 2) {
    bytes[i / 2] = (parseInt(hexString[i], 16) << 4) | parseInt(hexString[i + 1], 16);
  }

  return bytes;
}