#!/bin/bash
# Copyright 2025 by Ideal Labs, LLC
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

set -e

echo "Building timelock-ffi and examples..."

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "ERROR: Please run this script from the timelock repository root"
    exit 1
fi

if [ ! -d "timelock-ffi" ]; then
    echo "ERROR: timelock-ffi directory not found"
    exit 1
fi

# Check for Cargo
if ! command -v cargo &> /dev/null; then
    echo "ERROR: cargo is required but not found in PATH"
    exit 1
fi

echo "[INFO] Building timelock-ffi library..."
cargo build --release --manifest-path timelock-ffi/Cargo.toml

echo "[INFO] Running FFI tests..."
cargo test --manifest-path timelock-ffi/Cargo.toml

# Check that header was generated
if [ ! -f "target/release/timelock.h" ]; then
    echo "ERROR: Header file not generated! Check cbindgen configuration."
    exit 1
fi

echo "[INFO] Generated header file: target/release/timelock.h"

# Try to build C examples if compiler is available
if command -v gcc &> /dev/null; then
    echo "[INFO] Building C examples with GCC..."
    cd examples/timelock-ffi
    
    # Use Make if available
    if command -v make &> /dev/null; then
        echo "[INFO] Building with make..."
        make clean
        make all
        echo "[INFO] Running basic example..."
        if [ -x "./basic_example" ]; then
            ./basic_example
        else
            echo "WARNING: basic_example not found or not executable"
        fi
    else
        # Manual compilation
        echo "[INFO] Building manually with GCC..."
        
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
        
        gcc -std=c11 -I../../target/release -o basic_example basic_example.c -L../../target/release -ltimelock_ffi $PLATFORM_LIBS
        if [ $? -eq 0 ]; then
            echo "[INFO] Running GCC-built example..."
            ./basic_example
        fi
    fi
    cd ../..
elif command -v clang &> /dev/null; then
    echo "[INFO] Building C examples with Clang..."
    cd examples/timelock-ffi
    
    # Platform detection for Clang
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
    
    clang -std=c11 -I../../target/release -o basic_example basic_example.c -L../../target/release -ltimelock_ffi $PLATFORM_LIBS
    if [ $? -eq 0 ]; then
        echo "[INFO] Running Clang-built example..."
        ./basic_example
    fi
    cd ../..
else
    echo "WARNING: No C compiler found, skipping C examples"
fi

echo "[INFO] Build and test completed successfully!"
echo "[INFO] Generated files:"

# Platform-specific output
case "$(uname -s)" in
    Linux*)
        echo "  - target/release/libtimelock_ffi.a (static library)"
        if [ -f "target/release/libtimelock_ffi.so" ]; then
            echo "  - target/release/libtimelock_ffi.so (dynamic library)"
        fi
        ;;
    Darwin*)
        echo "  - target/release/libtimelock_ffi.a (static library)"
        if [ -f "target/release/libtimelock_ffi.dylib" ]; then
            echo "  - target/release/libtimelock_ffi.dylib (dynamic library)"
        fi
        ;;
    *)
        echo "  - target/release/libtimelock_ffi.a (static library)"
        ;;
esac

echo "  - target/release/timelock.h (C header)"

echo ""
echo "[INFO] To use the library in your C project:"
echo "  1. Include target/release/timelock.h in your source"

case "$(uname -s)" in
    Linux*)
        echo "  2. Link against target/release/libtimelock_ffi.a"
        echo "  3. Add platform libraries: -pthread -ldl -lm"
        ;;
    Darwin*)
        echo "  2. Link against target/release/libtimelock_ffi.a"
        echo "  3. Add platform libraries: -framework Security -framework CoreFoundation"
        ;;
    *)
        echo "  2. Link against target/release/libtimelock_ffi.a"
        echo "  3. Add platform libraries: -pthread -ldl -lm"
        ;;
esac

echo ""
echo "[INFO] See examples/timelock-ffi/README.md for detailed usage instructions"
