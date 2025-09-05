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
import { IdentityBuilder } from './IIdentityBuilder'
import { Buffer } from "buffer";

/**
 * Compute the sha256 hash of the data
 * @param data: Some Uint8Array
 * @returns The hash
 */
async function sha256(data: Uint8Array): Promise<string> {
    const hashBuffer = await crypto.subtle.digest('SHA-256', data);
    const hashArray = Array.from(new Uint8Array(hashBuffer));
    const hashHex = hashArray.map(b => b.toString(16).padStart(2, '0')).join('');
    return hashHex;
}

/**
 * Build a message that is signed by Drand Quicknet: sha256(round_number as big endian)
 * @param round: The round number
 * @returns The message
 */
function generateMessage(round: number): Promise<Uint8Array> {
    const buffer = Buffer.alloc(8);
    buffer.writeBigUInt64BE(BigInt(round), 0);
    return sha256(buffer).then(result => u8a(result))
}

/**
 * Converts the hex-encoded string to a Uint8Array
 * @param hex A hex-encoded string
 * @returns A Uint8Array
 */
function u8a(hexString: string): Uint8Array {
    // filter out invalid chars
    if (!/^[0-9a-fA-F]*$/.test(hexString)) {
        throw new Error("Invalid hex string: Contains non-hex characters")
    }

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

/**
 * An IdentityBuilder for the Drand Quicknet beacon
 */
export const DrandIdentityBuilder: IdentityBuilder<number> = {
    build: (roundNumber) => generateMessage(roundNumber),
}