# Timelock Encryption TypeScript Wrapper

This is a typescript library for timelock encryption. It is a "thin" wrapper that calls the WebAssembly (WASM) implementation of timelock encryption. It is designed for use in web-based environments and easily integrates with frameworks like React, Vue, etc. The library currently supports randomness beacons using curve BLS12-381m such as [Drand's](https://drand.love) Quicknet.

## Installation

The package can be installed from the npm registry with:

``` shell
npm i @ideallabs/timelock.js
```

## Build

From the root, run

```
npm run build
```

In addition to transpiling the project, this builds the latest wasm and makes it available to the typescript wrapper.

> Note: After running, navigate to the produced dist/index.js file and add `.js` endings to imports. See: https://github.com/ideal-lab5/timelock/issues/8


## Test

From the root, run:

```shell
npm run test
```

## Usage

See the [example](../examples/web/react-tlock-demo/) for a full demonstration.

### Initialization

``` js
import { Timelock } from '@ideallabs/timelock.js'
// Uses curve BLS12-381 (e.g. Drand Quicknet)
const timelock = await Timelock.build();
```

### Encrypting a Message

Messages can be encrypted for future rounds of a supported beacon's protocol by specifying the beacon public key, round number, and message. Internally the library uses AES-GCM by default (this can be customized by implementing a custom [BlockCipherProvider](https://docs.rs/timelock/0.0.1/timelock/block_ciphers/trait.BlockCipherProvider.html)).

``` js
// import a pre-defined IdentityHandler implementation or create your own
import { Timelock } from '@ideallabs/timelock.js'
import hkdf from 'js-crypto-hkdf'
// 1. Setup parameters for encryption
// use an hkdf to generate an ephemeral secret key
const seed = new TextEncoder().encode('my-secret-seed')
const hash = 'SHA-256'
const length = 32
const esk = await hkdf.compute(seed, hash, length, '')
// the message to encrypt for the future
const message = 'Hello, Timelock!'
const encodedMessage = new TextEncoder().encode(message)
// A randomness beacon public key (ex: Drand public key)
const pkhex = '83cf0f2896adee7eb8b5f01fcad3912212c437e0073e911fb90022d3e760183c8c4b450b6a0a6c3ac6a5776a2d1064510d1fec758c921cc22b0e17e63aaf4bcb5ed66304de9cf809bd274ca73bab4af5a6e9c76a4bc09e76eae8991ef5ece45a'
const pubkey = Uint8Array.from(
  pkHex.match(/.{1,2}/g).map((byte) => parseInt(byte, 16))
)
// A future round number of the randomness beacon
const roundNumber = 1000
// 2. Encrypt the message
let ct = await timelockDrand.encrypt(
  encodedMessage,
  roundNumber,
  esk.key,
  pubkey,
)
console.log('Timelocked ciphertext: ' + JSON.stringify(ct))
```

#### Identity Handlers

Any given randomness beacon may sign messages in its own unique way. For example, in Drand's Quicknet the beacon signs the sha256 hash of the round number of the procol as a big endian array (8 bytes from a u64 round number). In the Ideal network, the message is the sha256 hash of the round number concatenated with the validator set id of the set of validators that produced the beacon. 

This library offers a pre-defined identity handler for usage with Drand Quicknet, the [DrandIdentityHandler](./src/interfaces/DrandIdentityBuilder.ts). For beacons that construct messages differently, a custom identity handler must be implemented. 

### Decrypting a Message

Decrypt data using a beacon signature:

``` js
// Acquire a signature for decryption from he pulse output by the beacon at the given roundNumber
const sigHex = ('b44679b9a59af2ec876b1a6b1ad52ea9b1615fc3982b19576350f93447cb1125e342b73a8dd2bacbe47e4b6b63ed5e39')
const sig = Uint8Array.from(
  sigHex.match(/.{1,2}/g).map((byte) => parseInt(byte, 16))
)
// Decrypt the ciphertext with the signature
const plaintext = await timelockDrand.decrypt(ct, sig)
console.log(`Recovered ${String.fromCharCode(...plaintext)}git `)
```

### Force Decrypting a Message

Decrypt a message early with the secret key used for encryption (using AES-GCM):

``` js
// rederive the esk
const seed = new TextEncoder().encode('password')
const hash = 'SHA-256'
const length = 32
const esk = await hkdf.compute(seed, hash, length, '')
const key = Array.from(esk.key)
  .map((byte) => byte.toString(16).padStart(2, '0'))
  .join('')
const plaintext = await timelock.forceDecrypt(ciphertext, key);
console.log('Plaintext:', plaintext);
```

## License

Apache-2.0
