#!/bin/bash
# Enhanced compilation script for all timelock-ffi C examples
# Copyright 2025 by Ideal Labs, LLC
# Licensed under Apache License, Version 2.0

set -e

echo "Timelock Encryption C FFI Examples - Build Script"
echo "=================================================="

cd "$(dirname "$0")/../../examples/timelock-ffi"

echo ""
echo "Building all examples..."

# Check if library exists
if [ ! -f "../../timelock-ffi/target/release/libtimelock_ffi.a" ] && [ ! -f "../../timelock-ffi/target/release/libtimelock_ffi.so" ] && [ ! -f "../../timelock-ffi/target/release/libtimelock_ffi.dylib" ]; then
    echo "ERROR: timelock_ffi library not found!"
    echo "Please build it first with: ./scripts/build-ffi.sh"
    exit 1
fi

# Check if header exists
if [ ! -f "../../timelock-ffi/target/release/timelock.h" ]; then
    echo "ERROR: timelock.h not found!"
    echo "Please build the FFI library first to generate the header"
    exit 1
fi

# Platform detection
case "$(uname -s)" in
    Linux*)
        PLATFORM_LIBS="-pthread -ldl -lm"
        LIB_NAME="libtimelock_ffi.a"
        ;;
    Darwin*)
        PLATFORM_LIBS="-framework Security -framework CoreFoundation"
        LIB_NAME="libtimelock_ffi.a"
        ;;
    *)
        PLATFORM_LIBS="-pthread -ldl -lm"
        LIB_NAME="libtimelock_ffi.a"
        ;;
esac

# Set compiler
CC=${CC:-gcc}
if ! command -v "$CC" &> /dev/null; then
    if command -v clang &> /dev/null; then
        CC=clang
    else
        echo "ERROR: No C compiler found (tried gcc and clang)"
        exit 1
    fi
fi

echo "Using compiler: $CC"

# Create target directory if it doesn't exist
mkdir -p target

# Build basic example
echo ""
echo "[1/2] Compiling basic_example.c..."
$CC -std=c11 -I../../timelock-ffi -o target/basic_example basic_example.c -L../../timelock-ffi/target/release -ltimelock_ffi $PLATFORM_LIBS
if [ $? -eq 0 ]; then
    echo "[SUCCESS] target/basic_example created successfully"
else
    echo "[ERROR] Failed to compile basic_example.c"
    exit 1
fi

# Build enhanced example
echo ""
echo "[2/2] Compiling enhanced_example.c..."
$CC -std=c11 -I../../timelock-ffi -o target/enhanced_example enhanced_example.c -L../../timelock-ffi/target/release -ltimelock_ffi $PLATFORM_LIBS
if [ $? -eq 0 ]; then
    echo "[SUCCESS] target/enhanced_example created successfully"
else
    echo "[ERROR] Failed to compile enhanced_example.c"
    exit 1
fi

echo ""
echo "=================================================="
echo "All examples compiled successfully!"
echo ""
echo "Available executables:"
echo "  - ./target/basic_example"
echo "  - ./target/enhanced_example"
echo ""
echo "Run examples with:"
echo "  ./target/basic_example"
echo "  ./target/enhanced_example --help"
echo ""
echo "Or use 'make test' to run all examples automatically"
