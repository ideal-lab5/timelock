# Timelock Encyrption TypeScript Wrapper

This library provides a TypeScript wrapper for a WebAssembly (WASM) implementation of timelock encryption, designed for seamless use in web-based environments. It integrates easily with frameworks like React and supports timelock encryption, timelock decryption, and AES-GCM decryption functionality.

## Installation

This library is not yet available on NPM. When available, we will update the readme with installation guides. For now, it can only be installed by building the project locally.

## Usage

### Initialization

Before using any encryption or decryption methods, initialize the library by creating a Timelock instance:

``` js
import { Timelock } from 'timelock-encryption';

const timelock = await Timelock.build();
```

### Encrypting a Message

Encrypt data for a specific protocol round:

``` js
const encodedMessage = new Uint8Array([1, 2, 3]); // Data to encrypt
const beaconPublicKey = new Uint8Array([4, 5, 6]); // Randomness beacon's public key
const ephemeralSecretKey = new Uint8Array([7, 8, 9]); // AES-GCM secret key
const roundNumber = 42;

const ciphertext = await timelock.encrypt(
  encodedMessage,
  roundNumber,
  IdealNetworkIdentityBuilder, // IdentityBuilder implementation
  beaconPublicKey,
  ephemeralSecretKey
);

console.log('Ciphertext:', ciphertext);
```

### Decrypting a Message

Decrypt data using a beacon signature:

``` js
const ciphertext = new Uint8Array([10, 11, 12]);
const signature = new Uint8Array([13, 14, 15]);

const plaintext = await timelock.decrypt(ciphertext, signature);

console.log('Decrypted Message:', plaintext);
```

### Force Decrypting a Message

Decrypt a message early with the secret key used for encryption (using AES-GCM):

``` js
const ephemeralSecretKey = new Uint8Array([19, 20, 21]);

const plaintext = await timelock.forceDecrypt(ciphertext, ephemeralSecretKey);

console.log('Plaintext:', plaintext);
```

## Build

Build the project with:

``` shell
npm run build
```

## Test

From the root, run:

```shell
npm run test
```

## License

Apache-2.0