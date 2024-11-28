# WASM Bindings for Timelock

This crate provides wasm compatibility for the timelock crate. It provides compatibility for both javascript and python (3+).

## Build

### For JavaScript Compatibility
To compile to wasm, first build the project and them run wasm-pack

``` shell
cargo build
wasm-pack build --target web --out-dir pkg
```

### For Python Compatibility

Python compatible wasm output is generated with [PyO3](https://pyo3.rs/v0.23.2/).

First create a virtual env, then run:
``` sh
pip install maturin
# specify your python version
export PYO3_CROSS_PYTHON_VERSION="3.10"
maturin develop
```

#### Publish

``` sh
# creat a release build
maturin build --release
# publish to testpypi

```