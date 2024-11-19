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

import init, { timelockEncrypt, timelockDecrypt, IdealNetworkIdentityHandler } from '../../dist/timelock.js'
import * as fs from 'fs/promises';
import * as path from 'path';

// Example parameters
const exampleMessage = "Hello, Timelock!";
// The round number of the beacon protocol for which we use a signautre
const roundNumber = 10;
// The drand quicknet pubkey
const beaconPublicKey = Uint8Array.from(Buffer.from('aced2f1f580f863d9144844275e76c8efdb255299c8188c7a63292ea5ebd40d3dde2bdf7466f57053e843a1100bf8a80a8cbebf43a11477763e3f02f31a5656aef2231888c71593e8c460e344cf4f6f03d3cbb342718b41b89d51efcfdf91a00082ea50f652239aaf2c6c7c7a87d9e76f6b4337da8f0c4ebc50cd79ae781da0dcfaad6d8b9cdc6f087eac7bffb164101', 'hex'))
// Simulated signature (in a real scenario, this comes from the randomness beacon)
const signature = Uint8Array.from(Buffer.from('38552cb0180ae083d73f12ca75e680f2a597f8f8cb40bbb07c0fe57786f14b44ef3c1351d7aab135dfa5e096b3f82181', 'hex'))
// A seed passed to the rng within the protcool
const seed = "my-secure-seed";

async function main() {
    try {
        // this is only required for node usage of the lib?
        // explicitly load the wasm
        const wasmPath = path.resolve('../../../wasm/pkg/timelock_wasm_wrapper_bg.wasm')
        // Read the .wasm file as a buffer
        const wasmBytes = await fs.readFile(wasmPath);
        // Initialize the WebAssembly module with the buffer
        await init(wasmBytes);

        console.log("Original Message:", exampleMessage);

        // Encrypt the message
        const encodedMessage = new TextEncoder().encode(exampleMessage);
        const ciphertext = await timelockEncrypt(
            encodedMessage,
            roundNumber,
            IdealNetworkIdentityHandler,
            beaconPublicKey,
            seed
        );
        console.log("Ciphertext:", ciphertext);
        // Decrypt the message
        const plaintext = await timelockDecrypt(ciphertext, signature);
        console.log("Decrypted Message:", plaintext);
    } catch (error) {
        console.error("Error:", error);
    }
}

main();
