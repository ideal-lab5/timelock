# Timelock.js 

This library provides a TypeScript wrapper around a WebAssembly (WASM) module for timelock encryption, compiled from [timelock](../timelock/). It is designed for use in cryptographic protocols that rely on time-based encryption and decryption mechanisms, particularly for integration with verifiable randomness beacons such as the [Ideal Network](https://docs.idealabs.network). Currently this library ONLY supports the IDN for timelock encryption the browser, with support for drand in the future.

## Installation

For now, this library c**an only be installed by cloning this repo and building it locally**. In the near future this package will be available on npm. 

``` shell
npm install @ideallabs/timelock.js
```

## Build  

To build the typescript wrapper locally, clone the project and execute the following:

``` shell
git clone git@github.com:ideal-lab5/timelock.git
cd ts
npm run build
```

The build script is configured to build the wasm, install it, and then transpile the typescript.

## Usage

### Encryption

``` js

```

### Decryption

``` js

```

## 

## License

Apache-2.0
