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

/**
 * @file basic_example.c
 * @brief Basic example demonstrating timelock encryption/decryption using C FFI bindings
 *
 * This example shows how to:
 * 1. Create an identity for a specific round number (Drand-style)
 * 2. Encrypt a message using timelock encryption
 * 3. Decrypt the message using a beacon signature
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

// Include the generated header (this will be created by cbindgen)
#include "timelock.h"

void print_hex(const char* label, const uint8_t* data, size_t len) {
    printf("%s: ", label);
    for (size_t i = 0; i < len; i++) {
        printf("%02x", data[i]);
    }
    printf("\n");
}

int main() {
    printf("Timelock Encryption C Example\n");
    printf("=============================\n\n");

    // Example parameters
    const char* message = "Hello, Timelock Encryption!";
    const uint64_t round_number = 2000;
    const uint8_t secret_key[32] = {
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10,
        0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18,
        0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20
    };

    // Drand quicknet public key (for testing)
    const char* public_key_hex = "83cf0f2896adee7eb8b5f01fcad3912212c437e0073e911fb90022d3e760183c8c4b450b6a0a6c3ac6a5776a2d1064510d1fec758c921cc22b0e17e63aaf4bcb5ed66304de9cf809bd274ca73bab4af5a6e9c76a4bc09e76eae8991ef5ece45a";

    // The signature for round 2000 (48-byte compressed G1 signature from QuickNet)
    // This signature was obtained from the Drand Quicknet public randomness beacon for round 2000.
    // Note: Drand QuickNet uses the "bls-unchained-g1-rfc9380" scheme, which places BLS signatures
    // on G1 (48 bytes) instead of the typical G2 (96 bytes). This is intentional and matches
    // the QuickNet protocol specification. There is no protocol mismatch here.
    // You can verify or fetch this value using the Drand HTTP API for QuickNet, e.g.:
    // curl https://api.drand.sh/52db9ba70e0cc0f6eaf7803dd07447a1f5477735fd3f661792ba94600c84e971/public/latest | jq .signature
    // Or for a specific round:
    // curl https://api.drand.sh/52db9ba70e0cc0f6eaf7803dd07447a1f5477735fd3f661792ba94600c84e971/public/2000 | jq .signature
    // The value here is the hex-encoded BLS signature for round 2000 as returned by the Drand QuickNet network.
    // Last verified: August 14, 2025
    /*
     * WARNING: This is a hardcoded test signature for demo purposes only.
     * In production, fetch the actual signature from the Drand API for your specific round.
     * Verification date: August 14, 2025
     */
    const char* signature_hex = "b6cb8f482a0b15d45936a4c4ea08e98a087e71787caee3f4d07a8a9843b1bc5423c6b3c22f446488b3137eaca799c77e";

    printf("Message: %s\n", message);
    printf("Round number: %llu\n", (unsigned long long)round_number);
    printf("Public key: %s\n", public_key_hex);
    printf("Signature: %s\n\n", signature_hex);

    // Step 1: Create identity for the round number
    uint8_t identity[32];
    TimelockResult result = timelock_create_drand_identity(round_number, identity, sizeof(identity));
    if (result != Success) {
        printf("Error: Failed to create identity (code: %d)\n", result);
        return 1;
    }
    print_hex("Identity", identity, sizeof(identity));

    // Step 2: Encrypt the message
    printf("\nEncrypting message...\n");
    TimelockCiphertext* ciphertext = NULL;
    result = timelock_encrypt(
        (const uint8_t*)message,
        strlen(message),
        identity,
        sizeof(identity),
        public_key_hex,
        secret_key,
        &ciphertext
    );

    if (result != Success) {
        printf("Error: Encryption failed (code: %d)\n", result);
        return 1;
    }

    printf("Encryption successful! Ciphertext length: %zu bytes\n", ciphertext->len);

    // Step 3: Decrypt the message
    printf("\nDecrypting message...\n");
    uint8_t plaintext[1024]; // Buffer for decrypted message
    size_t plaintext_len = sizeof(plaintext);

    result = timelock_decrypt(
        ciphertext,
        signature_hex,
        plaintext,
        &plaintext_len
    );

    if (result != Success) {
        printf("Error: Decryption failed (code: %d)\n", result);
        if (result == MemoryError) {
            printf("Required buffer size: %zu bytes\n", plaintext_len);
        }
    } else {
        printf("Decryption successful!\n");
        printf("Decrypted message: %.*s\n", (int)plaintext_len, plaintext);
        
        // Verify the message matches
        if (plaintext_len == strlen(message) && 
            memcmp(plaintext, message, plaintext_len) == 0) {
            printf("SUCCESS: Message verification successful!\n");
        } else {
            printf("✗ Message verification failed!\n");
        }
    }

    // Step 4: Cleanup
    timelock_ciphertext_free(ciphertext);

    // Print library version
    const char* version = timelock_get_version();
    if (version) {
        printf("\nTimelock library version: %s\n", version);
    }

    return 0;
}
