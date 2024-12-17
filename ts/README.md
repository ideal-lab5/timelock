# Timelock Encyrption TypeScript Wrapper

This is a typescript library for timelock encryption. It is a "thin" wrapper that calls the WebAssembly (WASM) implementation of timelock encryption. It is designed for use in web-based environments and easily integrates with frameworks like React, Vue, etc. The library supports both the experiemental [Ideal Network beacon](https://docs.idealabs.network) as well as [Drand's](https://drand.love) Quicknet.

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

Before using any encryption or decryption methods, initialize the library by creating a Timelock instance:

``` js
import { SupportedBeacon, Timelock } from '@ideallabs/timelock.js'
// Use curve BLS12-381 (e.g. Drand Quicknet)
const timelockBls12_381 = await Timelock.build(SupportedCurve.BLS12_381);
// Use curve BLS12-377 (e.g. IDN Beacon)
const timelockBls12_377 = await Timelock.build(SupportedCurve.BLS12_377);
```

### Encrypting a Message

Messages can be encrypted for future rounds of a supported beacon's protocol by specifying the be acon public key, round number, and message. Internally the library uses AES-GCM by default (this can be customized by implementing a custom [StreamCipherProvider](https://docs.rs/timelock/0.0.1/timelock/stream_ciphers/trait.StreamCipherProvider.html)).

``` js
// import a pre-defined IdentityHandler implementation or create your own
import { Timelock, IdealNetworkIdentityHandler } from '@ideallabs/timelock.js'

import hkdf from 'js-crypto-hkdf'
// 1. Setup parameters for encryption
// use an hkdf to securely generate an ephemeral secret key for AES-GCM encryption
const seed = new TextEncoder().encode('password')
const hash = 'SHA-256'
const length = 32
const esk = await hkdf.compute(seed, hash, length, '')
// the message to encrypt for the future
const message = 'Hello, Timelock!'
const encodedMessage = new TextEncoder().encode(message)
// A randomness beacon public key (ex: IDN public key)
const pkHex =
  'b68da85d953219f84d86c5167481f505edf04ab586f28aefd238475026f5f46ba707f41bd2702f3639a4eddff8cad50041dc53da3d3617a189c85c8cb51a5f4fdfcebda05c50e81595f69e178d240fce3acdafd97b5fd204553e685836393a00b112f5cd78477d79ac8094c608d35bb42bd5091c5bbedd881e2ee0e8492a4361c69bf15250d75aee44035bc5b7553100'
// Convert the hex string to a Uint8Array
const pubkey =  Uint8Array.from(pkHex.match(/.{1,2}/g).map((byte) => parseInt(byte, 16)))
// A FUTURE round number of the randomness beacon
const roundNumber = 10

// 2. Encrypt the message
let ct = await timelock.encrypt(
  encodedMessage,
  roundNumber,
  IdealNetworkIdentityHandler,
  pubkey,
  esk.key,
)
console.log('Timelocked ciphertext: ' + JSON.stringify(ct))
```

#### Identity Handlers

Any given randomness beacon may sign messages in its own unique way. For example, in Drand's Quicknet the beacon signs the sha256 hash of the round number of the procol as a big endian array (8 bytes from a u64 round number). In the Ideal network, the message is the sha256 hash of the round number concatenated with the validator set id of the set of validators that produced the beacon. 

This library offers pre-defined identity handlers for usage with Drand Quicknet and the IDN beacon, the [DrandIdentityHandler](./src/interfaces/DrandIdentityBuilder.ts) and  [IdealNetworkIdentityHandler](./src/interfaces/IDNIdentityBuilder.ts), respectively. For beacons that construct messages differently, a custom identity handler must be implemented. 

### Decrypting a Message

Decrypt data using a beacon signature:

``` js
// Acquire a signature for decryption from he pulse output by the beacon at the given roundNumber
const sigHex =
  'e6cdf6c9d11c13e013b2c6cfd11dab46d8f1ace226ff845ffff4c7d6f64992892c54fb5d1f0f87dd300ce66f53598e01'
const sig = Uint8Array.from(sigHex.match(/.{1,2}/g).map((byte) => parseInt(byte, 16)))
// Decrypt the ciphertext with the signature
const plaintext = await timelock.decrypt(ct, sig)
console.log(`Recovered ${String.fromCharCode(...plaintext)}`)
```

### Force Decrypting a Message

Decrypt a message early with the secret key used for encryption (using AES-GCM):

``` js
// rederive the esk
const seed = new TextEncoder().encode('password')
const hash = 'SHA-256'
const length = 32
const esk = await hkdf.compute(seed, hash, length, '')
const plaintext = await timelock.forceDecrypt(ciphertext, esk.key);
console.log('Plaintext:', plaintext);
```

## License

Apache-2.0