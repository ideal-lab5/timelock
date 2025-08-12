#!/bin/bash
# Compilation script for enhanced timelock-ffi example
# Copyright 2025 by Ideal Labs, LLC
# Licensed under Apache License, Version 2.0

set -e

cd "$(dirname "$0")/../../examples/timelock-ffi"

echo "Compiling enhanced_example.c..."

# Platform detection
case "$(uname -s)" in
    Linux*)
        PLATFORM_LIBS="-pthread -ldl -lm"
        ;;
    Darwin*)
        PLATFORM_LIBS="-framework Security -framework CoreFoundation"
        ;;
    *)
        PLATFORM_LIBS="-pthread -ldl -lm"
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

# Check dependencies
if [ ! -f "../../timelock-ffi/target/release/timelock.h" ]; then
    echo "ERROR: timelock.h not found!"
    echo "Please build the FFI library first with: ./scripts/build-ffi.sh"
    exit 1
fi

# Create target directory if it doesn't exist
mkdir -p target

# Compile
$CC -std=c11 -I../../timelock-ffi -o target/enhanced_example enhanced_example.c -L../../timelock-ffi/target/release -ltimelock_ffi $PLATFORM_LIBS

if [ $? -eq 0 ]; then
    echo "[SUCCESS] target/enhanced_example compiled successfully"
    echo "Run with: ./target/enhanced_example --help"
else
    echo "[ERROR] Compilation failed"
    exit 1
fi
