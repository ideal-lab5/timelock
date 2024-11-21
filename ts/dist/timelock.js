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
import init, { build_encoded_commitment, tle, tld, decrypt, } from 'timelock-wasm-wrapper';
const HASH = 'SHA-256';
const HASH_LENGTH = 32;
/**
 * An IdentityBuilder for the Ideal Network
 */
export const IdealNetworkIdentityHandler = {
    build: (bn) => build_encoded_commitment(bn, 0),
};
/**
 * The Timelock class handles initialization of the WASM modules required to use the Timelock library
 * from web based contexts. It is a thin wrapper around the output of running `wasm-pack --target web` from the wasm directory
 * It gracefully ensures that the WASM is available before attempting to call the respective functions.
 */
export class Timelock {
    /**
     * Indicates if the wasm has been initialized or not
     */
    wasmReady;
    /**
     * A private constructor to enforce usage of `build`
     */
    constructor() { }
    /**
     * Loads the wasm and constructs a new Timelock instance
     * @returns
     */
    static async build() {
        await init();
        return new Timelock();
    }
    /**
     * Timelock Encryption: Encrypt the message for the given block
     * The HKDF used satisfies RFC5869
     *
     * @param encodedMessage: The message to encrypt, encoded as a Uint8Array
     * @param roundNumber: The round of the protocol
     * @param identityBuilder: Something that imlement IdentityBuilder (e.g. IdealNetworkIdentityHandler)
     * @param beaconPublicKey: The public key of the randomness beacon
     * @param ephemeralSecretKey: An ephemeral secret key passed to AES-GCM
     * @returns The Timelocked ciphertext
     */
    async encrypt(encodedMessage, roundNumber, identityBuilder, beaconPublicKey, ephemeralSecretKey) {
        await this.checkWasm();
        let id = identityBuilder.build(roundNumber);
        return tle(id, encodedMessage, ephemeralSecretKey, beaconPublicKey);
    }
    /**
     * Timelock decryption: Decrypt the ciphertext using a pulse from the beacon produced at the given block
     * @param ciphertext: Ciphertext to be decrypted
     * @param blockNumber: Block number that has the signature for decryption
     * @returns: The decrypted message (if successful), else nothing (is that what I want?)
     */
    async decrypt(ciphertext, signature) {
        await this.checkWasm();
        return tld(ciphertext, signature);
    }
    /**
     * Decrypt a ciphertext early using the cipher used when encrypting if you know the seed
     * @param ciphertext The ciphertext to decrypt
     * @param ephemeralSecretKey: An ephemeral secret key passed to AES-GCM
     * @returns The plaintext
     */
    async forceDecrypt(ciphertext, ephemeralSecretKey) {
        await this.checkWasm();
        return decrypt(ciphertext, ephemeralSecretKey);
    }
    /**
     * Check if the wasm has been initialized.
     * If it hasn't, gracefully load the wasm and continue.
     * Q: In which scenarios would the wasm be unavailable after initially being loaded?
     */
    async checkWasm() {
        if (!this.wasmReady) {
            await init();
        }
    }
}
