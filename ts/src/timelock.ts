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
import { DrandIdentityBuilder } from './interfaces/DrandIdentityBuilder'

// the size (in bytes) of a public key
const PUBLIC_KEY_SIZE = 96
// the size (in bytes) of a secret key
const SECRET_KEY_SIZE = 32
// the size (in bytes) of a signature
const SIGNATURE_SIZE = 48

/**
 * Critical runtime errors that can be encountered in the Timelock class
 */
export enum TimelockErrors {
  INVALID_SECRET_KEY_SIZE = "Ephemeral secret key must be exactly 32 bytes.",
  INVALID_SIGNATURE_SIZE = "Signatures must be exactly 48 bytes.",
  INVALID_PUBLIC_KEY_SIZE = "The beacon public key must be exactly 96 bytes.",
  INVALID_ROUND_NUMBER = "The round number must be a positive integer.",
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
    let timelock = new Timelock()
    // initialize the wasm 
    await init()
    timelock.wasmReady = true
    return timelock
  }

  /**
   * Timelock Encryption: Encrypt the message for the given round of the randomness beacon.
   *
   * @param encodedMessage: The message to encrypt, encoded as a Uint8Array
   * @param roundNumber: The round of the protocol when the message can be decrypted
   * @param beaconPublicKeyHex: The hex-encoded public key of the randomness beacon
   * @param ephemeralSecretKeyHex: A hex-encoded ephemeral secret key passed to AES-GCM
   * @returns The timelocked ciphertext if successful, otherwise an error message.
   */
  public async encrypt(
    encodedMessage: Uint8Array,
    roundNumber: number,
    ephemeralSecretKey: Uint8Array,
    beaconPublicKey: Uint8Array,
  ): Promise<Result<Uint8Array>> {
    // validations
    if (ephemeralSecretKey.length !== SECRET_KEY_SIZE) {
      return error(TimelockErrors.INVALID_SECRET_KEY_SIZE)
    }

    if (beaconPublicKey.length !== PUBLIC_KEY_SIZE) {
      return error(TimelockErrors.INVALID_PUBLIC_KEY_SIZE)
    }

    if (!Number.isInteger(roundNumber) || roundNumber <= 0) {
      return error(TimelockErrors.INVALID_ROUND_NUMBER)
    }

    try {
      await this.checkWasm()
      const id = await DrandIdentityBuilder.build(roundNumber)
      const ciphertext = tle(id, encodedMessage, ephemeralSecretKey, beaconPublicKey)
      return ok(new Uint8Array(ciphertext))
    } catch (err) {
      const message = err instanceof Error ? err.message : String(err)
      return error(message)
    }
  }

  /**
   * Timelock decryption: Decrypt the ciphertext using a pulse from the beacon produced at the given block
   * 
   * @param ciphertext: Ciphertext to be decrypted
   * @param secretKey: The secret key for decryption (BLS sig output from a beacon)
   * @returns: The decrypted message if successful, otherwise an error message.
   */
  public async decrypt(
    ciphertext: Uint8Array,
    signature: Uint8Array
  ): Promise<Result<Uint8Array>> {

    if (signature.length !== SIGNATURE_SIZE) {
      return error(TimelockErrors.INVALID_SIGNATURE_SIZE)
    }

    try {
      await this.checkWasm()
      const result = tld(ciphertext, signature)
      return ok(new Uint8Array(result))
    } catch (err) {
      const message = err instanceof Error ? err.message : String(err)
      return error(message)
    }
  }

  /**
   * Decrypt a ciphertext early using the cipher used when encrypting if you know the seed
   * 
   * @param ciphertext The ciphertext to decrypt
   * @param ephemeralSecretKey: An ephemeral secret key (32-bytes)
   * @returns The plaintext
   */
  public async earlyDecrypt(
    ciphertext: Uint8Array,
    ephemeralSecretKey: Uint8Array
  ): Promise<Result<Uint8Array>> {

    if (ephemeralSecretKey.length !== 32) {
      return error(TimelockErrors.INVALID_SECRET_KEY_SIZE)
    }

    try {
      await this.checkWasm()
      const res = decrypt(ciphertext, ephemeralSecretKey)
      return ok(new Uint8Array(res))
    } catch (err) {
      const message = err instanceof Error ? err.message : String(err)
      return error(message)
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
        const message = err instanceof Error ? err.message : String(err)
        throw new Error("Failed to initialize WASM " + message)
      }
    }
  }
}
