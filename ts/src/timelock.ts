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
/// Timelock Encryption TS Wrapper
/// This lib provides a typescript wrapper around the wasm-pack output of the timelock encryption library
import init, {
  build_encoded_commitment,
  tle,
  tld,
  decrypt,
} from 'timelock-wasm-wrapper'
import hkdf from 'js-crypto-hkdf' // for npm

const HASH = 'SHA-256'
const HASH_LENGTH = 32

/**
 * The IdentityBuilder is used to build identities for IBE
 * In relation to the verifiable randomness beacon used, the implementation
 * should correspond to however that beacon constructs messages for signing.
 */
interface IdentityBuilder<X> {
  /**
   * Build an identity based on the input 'x'
   * @param x : The identity data
   * @returns : The constructed identity
   */
  build: (x: X) => any
}

/**
 * An IdentityBuilder for the Ideal Network
 */
export const IdealNetworkIdentityHandler: IdentityBuilder<number> = {
  build: (bn) => build_encoded_commitment(bn, 0),
}

export class Timelock {
  /**
   * A private constructor to enforce usage of waitReady
   */
  private constructor() { }

  public static async waitReady() {
    await init()
    return new Timelock()
  }

  /**
 * Timelock Encryption: Encrypt the message for the given block
 * The HKDF used satisfies RFC5869
 *
 * @param encodedMessage: The message to encrypt, encoded as a Uint8Array
 * @param roundNumber: The round of the protocol
 * @param identityBuilder: Something that imlement IdentityBuilder (e.g. idealNetworkIdentityHandler)
 * @param beaconPublicKey: The public key of the randomness beacon used
 * @param seed: A seed to derive crypto keys
 * @returns the ciphertext
 */
  public async encrypt(
    encodedMessage: Uint8Array,
    roundNumber: number,
    identityBuilder: IdentityBuilder<number>,
    beaconPublicKey: Uint8Array,
    seed: string
  ): Promise<any> {
    await init()
    let t = new TextEncoder()
    let masterSecret = t.encode(seed)
    return hkdf
      .compute(masterSecret, HASH, HASH_LENGTH, '')
      .then((derivedKey) => {
        let id = identityBuilder.build(roundNumber)
        return tle(id, encodedMessage, derivedKey.key, beaconPublicKey)
      })
  }


  /**
   * Timelock decryption: Decrypt the ciphertext using a pulse from the beacon produced at the given block
   * @param ciphertext: Ciphertext to be decrypted
   * @param blockNumber: Block number that has the signature for decryption
   * @returns: Plaintext of encrypted message
   */
  public async decrypt(
    ciphertext: Uint8Array,
    signature: Uint8Array
  ): Promise<any> {
    return tld(ciphertext, signature)
  }


  /**
   * Decrypt a ciphertext early using the cipher used when encrypting if you know the seed
   * @param ciphertext The ciphertext to decrypt
   * @param seed The ciphertext seed
   * @returns The plaintext
   */
  public async forceDecrypt(
    ciphertext: Uint8Array,
    seed: string
  ): Promise<any> {
    await init()
    let t = new TextEncoder()
    let masterSecret = t.encode(seed)
    return hkdf
      .compute(masterSecret, HASH, HASH_LENGTH, '')
      .then((derivedKey) => {
        return decrypt(ciphertext, derivedKey)
      })
  }
}

// /**
//  * Timelock Encryption: Encrypt the message for the given block
//  * The HKDF used satisfies RFC5869
//  *
//  * @param encodedMessage: The message to encrypt, encoded as a Uint8Array
//  * @param roundNumber: The round of the protocol
//  * @param identityBuilder: Something that imlement IdentityBuilder (e.g. idealNetworkIdentityHandler)
//  * @param beaconPublicKey: The public key of the randomness beacon used
//  * @param seed: A seed to derive crypto keys
//  * @returns the ciphertext
//  */
// export async function timelockEncrypt(
//   encodedMessage: Uint8Array,
//   roundNumber: number,
//   identityBuilder: IdentityBuilder<number>,
//   beaconPublicKey: Uint8Array,
//   seed: string
// ): Promise<any> {
//   await init()
//   let t = new TextEncoder()
//   let masterSecret = t.encode(seed)
//   return hkdf
//     .compute(masterSecret, HASH, HASH_LENGTH, '')
//     .then((derivedKey) => {
//       let id = identityBuilder.build(roundNumber)
//       let ct = tle(id, encodedMessage, derivedKey.key, beaconPublicKey)
//       return ct
//     })
// }


// /**
//  * Decrypt a ciphertext early if you know the seed
//  * @param ciphertext The ciphertext to decrypt
//  * @param seed The ciphertext seed
//  * @returns The plaintext
//  */
// export async function forceDecrypt(
//   ciphertext: Uint8Array,
//   seed: string
// ): Promise<any> {
//   await init()
//   let t = new TextEncoder()
//   let masterSecret = t.encode(seed)
//   return hkdf
//     .compute(masterSecret, HASH, HASH_LENGTH, '')
//     .then((derivedKey) => {
//       let pt = decrypt(ciphertext, derivedKey)
//       return pt
//     })
// }
