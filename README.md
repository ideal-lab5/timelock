# Timelock Encryption

Timelock is an implemention of [timelock encryption](https://docs.idealabs.network/docs/learn/crypto/timelock_encryption) using the [Boneh Franklin -Idenity Based Encryption](https://crypto.stanford.edu/~dabo/papers/bfibe.pdf) scheme. Designed for versatility, it provides support for both Rust and JavaScript. In addition, it is capable of supporting multiple types of randomness beacons, including the [Ideal Network](https://docs.idealabs.network) and [drand](https://drand.love).

## Getting Started

Timelock is organized into core components and language-specific bindings to support WASM:

- **Core Library**: The [timelock](./timelock/) crate implements the core encryption algorithms and provides support for native Rust applications.
- **WASM bindings**: The [wasm](./wasm/) lib provides wasm bindings for the timelock encryption implementation, enabling usage of timelock encryption in javascript-based applications in a web-enabled context.
- **TypeScript Wrapper**: The [ts](./ts/) library is a typescript wrapper to adapt the wasm for easy integration in JavaScript projects. 

### For Rust developers
Navigate to the core timelock implementation [readme](./timelock/README.md) for details on building and using it in Rust.

### For Javascript developers
Navigate to the typescript bindings [readme](./ts/README.md) for more information on integration of `@ideallabs/timelock.js` in javascript apps.

## Contributing and Code of Conduct

Contributions are welcome! Feel free to open issues for problems or feature requests while we work on setting up our contributors guidelines.

## License

Apache-2.0