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

/** Timelock Encryption TS Wrapper
 * This lib provides a typescript wrapper around the wasm-pack output of the timelock encryption library
*/

import init, {
  tle,
  tld,
  decrypt,
} from 'timelock-wasm-wrapper'
import { IdentityBuilder } from './interfaces/IIdentityBuilder'

/**
 * Critical runtime errors that can be encountered in the Timelock class
 */
export enum TimelockErrors {
  ERR_UNEXPECTED_TYPE = "The wasm returned something that could not be converted to a UInt8Array."
}

/**
 * The Timelock class handles initialization of the WASM modules required to use the Timelock library
 * from web based contexts. It gracefully ensures that the WASM is available before attempting to call the respective functions.
 */
export class Timelock {
  /**
   * Indicates if the wasm has been initialized or not
   */
  private wasmReady: false

  /**
   * A private constructor to enforce usage of `build`
   */
  private constructor() { }

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
   * @param beaconPublicKey: The public key of the randomness beacon
   * @param ephemeralSecretKey: An ephemeral secret key passed to AES-GCM
   * @returns The timelocked ciphertext if successful, otherwise an error message.
   */
  public async encrypt(
    encodedMessage: Uint8Array,
    roundNumber: number,
    identityBuilder: IdentityBuilder<number>,
    beaconPublicKey: Uint8Array,
    ephemeralSecretKey: Uint8Array
  ): Promise<Uint8Array> {
    await this.checkWasm()
    let id = identityBuilder.build(roundNumber)
    return new Uint8Array(tle(id, encodedMessage, ephemeralSecretKey, beaconPublicKey))
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
    signature: Uint8Array
  ): Promise<Uint8Array> {
    await this.checkWasm()
    return new Uint8Array(tld(ciphertext, signature))
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
    ephemeralSecretKey: Uint8Array
  ): Promise<Uint8Array> {
    await this.checkWasm()
    return new Uint8Array(decrypt(ciphertext, ephemeralSecretKey))
  }

  /**
   * Check if the wasm has been initialized.
   * If it hasn't, gracefully load it and continue.
   */
  async checkWasm() {
    if (!this.wasmReady) {
      await init()
    }
  }
}
