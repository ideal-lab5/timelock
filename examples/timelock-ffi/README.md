# Timelock Encryption C FFI Examples

This directory contains examples demonstrating how to use the timelock encryption library from C and C++ applications.

## Overview

The timelock-ffi crate provides C-compatible bindings for the core timelock encryption functionality, enabling:

- **Native Integration**: Use timelock encryption directly in C/C++ projects
- **Cross-Platform Support**: Works on Linux, macOS, and Windows
- **Memory Safety**: Proper memory management with clear ownership semantics
- **Simple API**: Easy-to-use C functions for encrypt/decrypt operations

## Building the Examples

### Prerequisites

1. **Rust toolchain** (for building the FFI library)
2. **C compiler** (GCC, Clang, or MSVC)
3. **CMake** (version 3.20+ recommended for CMake-based builds)

#### Installing CMake

**Windows:**
- Download from [cmake.org](https://cmake.org/download/) and add to PATH
- Or use package manager: `winget install Kitware.CMake` or `choco install cmake`

**Linux:**
```bash
# Ubuntu/Debian
sudo apt-get update && sudo apt-get install cmake

# CentOS/RHEL
sudo yum install cmake
```

**macOS:**
```bash
# Homebrew
brew install cmake

# MacPorts
sudo port install cmake
```

### Option 1: Using CMake (Recommended)

```bash
# From this directory
mkdir build
cd build
cmake ..
cmake --build . --config Release

# Run the examples
./bin/basic_example          # Linux/macOS
./bin/basic_cpp_example      # Linux/macOS

# Windows:
./bin/Release/basic_example.exe
./bin/Release/basic_cpp_example.exe
```

### Option 2: Using Make (Unix/Linux)

```bash
# From this directory
make

# Run the examples
./target/basic_example
./target/basic_cpp_example
```

### Option 3: Manual Build

```bash
# First, build the Rust FFI library
cd ../../timelock-ffi
cargo build --release

# Then compile the C example
cd ../examples/timelock-ffi
gcc -std=c11 -I../../timelock-ffi \
    -o basic_example basic_example.c \
    -L../../timelock-ffi/target/release -ltimelock_ffi \
    -pthread -ldl -lm  # Linux

# Compile the C++ example
g++ -std=c++17 -I../../timelock-ffi \
    -o basic_cpp_example basic_cpp_example.cpp \
    -L../../timelock-ffi/target/release -ltimelock_ffi \
    -pthread -ldl -lm  # Linux
```

## Examples

### basic_example.c

Demonstrates the fundamental timelock encryption workflow:

1. **Identity Creation**: Generate an identity for a specific round number
2. **Encryption**: Encrypt a message using timelock encryption  
3. **Decryption**: Decrypt the message using a beacon signature

Uses round 2000 with the corresponding Drand QuickNet signature for a complete working example.

### basic_cpp_example.cpp

Demonstrates modern C++ integration with the timelock FFI:

1. **RAII Pattern**: Modern C++ resource management
2. **Exception Safety**: Proper error handling and cleanup
3. **Type Safety**: C++ wrappers around C FFI calls
4. **Performance Metrics**: Timing and overhead analysis

Uses round 1000 with the corresponding Drand QuickNet signature.

## API Reference

### Core Functions

```c
// Create a Drand-style identity for a round number
TimelockResult timelock_create_drand_identity(
    uint64_t round_number,
    uint8_t* identity_out,
    size_t identity_len
);

// Encrypt a message
TimelockResult timelock_encrypt(
    const uint8_t* message,
    size_t message_len,
    const uint8_t* identity,
    size_t identity_len,
    const char* public_key_hex,
    const uint8_t* secret_key,
    TimelockCiphertext** ciphertext_out
);

// Decrypt a message
TimelockResult timelock_decrypt(
    const TimelockCiphertext* ciphertext,
    const char* signature_hex,
    uint8_t* plaintext_out,
    size_t* plaintext_len
);

// Cleanup
void timelock_ciphertext_free(TimelockCiphertext* ciphertext);
```

### Result Codes

```c
typedef enum {
    Success = 0,
    InvalidInput = 1,
    EncryptionFailed = 2,
    DecryptionFailed = 3,
    MemoryError = 4,
    SerializationError = 5,
    InvalidPublicKey = 6,
    InvalidSignature = 7
} TimelockResult;
```

## Integration Guide

### Using pkg-config

If you've installed the library system-wide:

```bash
# Compile with pkg-config
gcc $(pkg-config --cflags timelock-ffi) \
    -o my_app my_app.c \
    $(pkg-config --libs timelock-ffi)
```

### CMake Integration

Add to your `CMakeLists.txt`:

```cmake
# Find the timelock-ffi library
find_package(PkgConfig REQUIRED)
pkg_check_modules(TIMELOCK_FFI REQUIRED timelock-ffi)

# Link against your target
target_link_libraries(my_target ${TIMELOCK_FFI_LIBRARIES})
target_include_directories(my_target PRIVATE ${TIMELOCK_FFI_INCLUDE_DIRS})
```

### Direct Integration

```c
#include "timelock.h"

int main() {
    // Your code here
    return 0;
}
```

## Platform-Specific Notes

### Linux
- Link with: `-pthread -ldl -lm`
- Use static library: `libtimelock_ffi.a`

### macOS
- Link with: `-framework Security -framework CoreFoundation`
- Use static library: `libtimelock_ffi.a`

### Windows
- Link with: `-lws2_32 -luserenv -ladvapi32 -lkernel32 -lntdll`
- Use static library: `timelock_ffi.lib` or dynamic: `timelock_ffi.dll`

## Security Considerations

1. **Memory Management**: Always call `timelock_ciphertext_free()` to prevent memory leaks
2. **Key Storage**: Securely handle secret keys and don't leave them in memory longer than necessary
3. **Input Validation**: The library performs basic validation, but additional application-level checks are recommended
4. **Timing Attacks**: Be aware of potential timing side-channels in your usage patterns

## Troubleshooting

### Common Issues

1. **Library not found**: Ensure the Rust library is built and in the expected location
2. **Header not found**: Make sure `timelock.h` is generated and accessible
3. **Linking errors**: Verify platform-specific libraries are linked correctly

### Building the FFI Library

```bash
# From the timelock-ffi directory
cd ../../timelock-ffi
cargo build --release

# This generates:
# - timelock-ffi/target/release/libtimelock_ffi.a (static library, Unix)
# - timelock-ffi/target/release/libtimelock_ffi.so (dynamic library, Linux) 
# - timelock-ffi/target/release/libtimelock_ffi.dylib (dynamic library, macOS)
# - timelock-ffi/target/release/timelock_ffi.lib (static library, Windows)
# - timelock-ffi/target/release/timelock_ffi.dll (dynamic library, Windows)
# - timelock-ffi/timelock.h (C header file, generated automatically)
```

## License

This project is licensed under the Apache License 2.0 - see the LICENSE file for details.
