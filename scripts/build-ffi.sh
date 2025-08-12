#!/bin/bash

set -e

echo "Building timelock-ffi..."

# Get the directory where this script is located and navigate to timelock-ffi
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR/../timelock-ffi"

# Build the FFI library using component-level target directory
cargo build --release --target-dir target

# Generate C header using cbindgen
if command -v cbindgen &> /dev/null; then
    echo "Generating C header file..."
    cbindgen --config cbindgen.toml --crate timelock-ffi --output timelock.h
else
    echo "cbindgen not found. Please install it with: cargo install cbindgen"
fi

cd "$SCRIPT_DIR/.."

echo "Build completed successfully!"
echo "Library: timelock-ffi/target/release/libtimelock_ffi.so (Linux) or timelock-ffi/target/release/libtimelock_ffi.dylib (macOS)"
echo "Header: timelock-ffi/timelock.h"
