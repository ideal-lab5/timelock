# Timelock Encryption

Timelock is an implemention of [timelock encryption](https://docs.idealabs.network/docs/learn/crypto/timelock_encryption) using the [Boneh Franklin -Idenity Based Encryption](https://crypto.stanford.edu/~dabo/papers/bfibe.pdf) scheme. Designed for versatility, it provides support for both Rust and JavaScript. In addition, it is capable of supporting multiple types of randomness beacons, including the [Ideal Network](https://docs.idealabs.network) and [drand](https://drand.love).

## Getting Started

Timelock is organized into core components and language-specific bindings:

- **Core Library**: The [timelock](./timelock/) crate implements the core encryption algorithms and provides support for native Rust applications.
- **C/C++ FFI**: The [timelock-ffi](./timelock-ffi/) crate provides C/C++ bindings for native integration in system software.
- **WASM bindings**: The [wasm](./wasm/) lib provides wasm bindings for the timelock encryption implementation, enabling usage of timelock encryption in JavaScript-based applications in a web-enabled context.
- **TypeScript Bindings**: The [ts](./ts/) library is a TypeScript wrapper to adapt the wasm for easy integration in JavaScript projects. 
- **Python Bindings**: The [py](./py) library provides Python bindings for timelock encryption.
- **C FFI Bindings**: The [timelock-ffi](./timelock-ffi/) crate provides C-compatible FFI bindings for integration with C/C++ projects, embedded systems, game engines, and other system software.

### For Rust Developers
Navigate to the core timelock implementation [readme](./timelock/README.md) for details on building and using it in Rust.

``` toml
timelock = { git = "https://github.com/ideal-lab5/timelock.git", default-features = false }
```

### For Javascript Developers
Navigate to the typescript bindings [readme](./ts/README.md) for more information on integration of `@ideallabs/timelock.js` in javascript apps.

``` sh
npm i @ideallabs/timelock.js
```

### For Python Developers
The [python bindings](./py/) are enabled with [PyO3](https://pyo3.rs) and allow timelock encryption and decryption to be computed in Python. 

``` sh
pip install timelock
```

### For C/C++ Developers
The [C FFI bindings](./timelock-ffi/) provide a stable C API for integration with C/C++ projects, embedded systems, game engines, and other system software.

```c
// Use in your C project
#include "timelock.h"
// Link against libtimelock_ffi.a (Unix) or timelock_ffi.lib (Windows)
```

See the [FFI documentation](./timelock-ffi/README.md) for build instructions and examples.

## Contributing and Code of Conduct

Contributions are welcome! Feel free to open issues for problems or feature requests while we work on setting up our contributors guidelines.

## License

Apache-2.0