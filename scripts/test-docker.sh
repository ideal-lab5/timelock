# Test script for Docker-based cross-platform testing
#!/bin/bash

echo "Testing timelock-ffi in Docker containers..."

# Test Ubuntu
echo "Testing on Ubuntu..."
docker run --rm -v $(pwd):/workspace -w /workspace ubuntu:22.04 bash -c "
    apt-get update && apt-get install -y build-essential curl
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source ~/.cargo/env
    cargo install cbindgen
    ./scripts/build-ffi.sh
    cd examples/timelock-ffi
    make all
    ./target/basic_example
"

# Test Alpine (minimal Linux)
echo "Testing on Alpine..."
docker run --rm -v $(pwd):/workspace -w /workspace alpine:latest sh -c "
    apk add --no-cache build-base curl bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source ~/.cargo/env
    cargo install cbindgen
    ./scripts/build-ffi.sh
    cd examples/timelock-ffi
    make all
    ./target/basic_example
"
