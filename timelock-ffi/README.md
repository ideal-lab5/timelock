# Timelock FFI Bindings

C-compatible FFI bindings for the timelock encryption library, enabling seamless integration with C, C++, and other systems programming languages.

## Overview

This crate provides a stable C API for timelock encryption operations, making it possible to use timelock encryption in:

- **C/C++ Applications**: Native desktop applications, system software, embedded systems
- **Game Engines**: Unreal Engine, custom C++ game engines
- **System Software**: Operating system components, device drivers, network services

## Features

- ✅ **C-Compatible API**: Standard C calling conventions and types
- ✅ **Memory Safety**: Proper memory management with clear ownership semantics
- ✅ **Cross-Platform**: Works on Linux, macOS, and Windows
- ✅ **Auto-Generated Headers**: C headers generated with cbindgen
- ✅ **Static and Dynamic Libraries**: Both library types supported
- ✅ **Multiple Build Systems**: CMake and Makefile integration examples
- ✅ **Comprehensive Tests**: Extensive test suite including memory leak detection
- ✅ **Example Code**: Complete examples with multiple build systems

## Examples

Working C examples are available in the `../examples/timelock-ffi/` directory:

- **`basic_example.c`**: Simple 4-step demonstration showing core timelock encryption workflow
- **`basic_cpp_example.cpp`**: C++ example demonstrating modern C++ integration with RAII and error handling

These examples demonstrate real-world usage patterns and can be compiled using the provided Makefile or CMakeLists.txt.

## Prerequisites

Before building the FFI library, ensure you have the following installed:

### Required
- **Rust toolchain** (1.70.0 or later)
  ```bash
  # Install via rustup (recommended)
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  # Or visit: https://rustup.rs/
  ```

- **cbindgen** for C header generation
  ```bash
  cargo install cbindgen
  ```

### Platform-Specific Build Tools

#### Linux
```bash
# Ubuntu/Debian
sudo apt-get install build-essential

# CentOS/RHEL/Fedora  
sudo dnf install gcc make
# Or: sudo yum install gcc make
```



#### macOS
```bash
# Install Xcode Command Line Tools
xcode-select --install

# Or via Homebrew (optional)
brew install gcc
```

#### Windows
- **Visual Studio 2019/2022** with C++ build tools
- Or **Visual Studio Build Tools** (lighter option)
- Or **MinGW-w64** via MSYS2

## Quick Start

### Building the Library

```bash
# Build the FFI library with component-level targets
cd timelock-ffi
cargo build --release --target-dir target

# Generate C header (requires cbindgen)
cargo install cbindgen  # If not already installed
cbindgen --config cbindgen.toml --crate timelock-ffi --output timelock.h

# This generates:
# - timelock-ffi/target/release/libtimelock_ffi.a (static library, Unix)
# - timelock-ffi/target/release/libtimelock_ffi.so (dynamic library, Linux)
# - timelock-ffi/target/release/libtimelock_ffi.dylib (dynamic library, macOS)
# - timelock-ffi/target/release/timelock_ffi.lib (static library, Windows)
# - timelock-ffi/target/release/timelock_ffi.dll (dynamic library, Windows)
# - timelock-ffi/timelock.h (C header file)
```

> **Note**: The build system uses component-level targets to keep build artifacts organized within the FFI component directory.

### Basic Usage

```c
#include "timelock.h"
#include <stdio.h>

int main() {
    // Create identity for round 1000
    uint8_t identity[32];
    TimelockResult result = timelock_create_drand_identity(1000, identity, sizeof(identity));
    if (result != Success) {
        printf("Failed to create identity\n");
        return 1;
    }

    // Encrypt message
    const char* message = "Hello, Timelock!";
    const char* public_key = "83cf0f2896adee7eb8b5f01fcad3912212c437e0073e911fb90022d3e760183c8c4b450b6a0a6c3ac6a5776a2d1064510d1fec758c921cc22b0e17e63aaf4bcb5ed66304de9cf809bd274ca73bab4af5a6e9c76a4bc09e76eae8991ef5ece45a";
    uint8_t secret_key[32] = {1}; // Your secret key here
    
    TimelockCiphertext* ciphertext = NULL;
    result = timelock_encrypt(
        (const uint8_t*)message, strlen(message),
        identity, sizeof(identity),
        public_key, secret_key,
        &ciphertext
    );
    
    if (result != Success) {
        printf("Encryption failed\n");
        return 1;
    }

    printf("Encryption successful! Ciphertext length: %zu\n", ciphertext->len);

    // Later, decrypt with beacon signature...
    // timelock_decrypt(ciphertext, signature, plaintext, &plaintext_len);

    // Always free allocated memory
    timelock_ciphertext_free(ciphertext);
    return 0;
}
```

## API Reference

### Core Types

```c
// Result codes for all operations
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

// Opaque handle for encrypted data
typedef struct {
    uint8_t* data;
    size_t len;
} TimelockCiphertext;
```

### Primary Functions

#### Identity Management

```c
// Create a Drand-style identity for encryption
TimelockResult timelock_create_drand_identity(
    uint64_t round_number,      // Future round number
    uint8_t* identity_out,      // Output buffer (32 bytes)
    size_t identity_len         // Buffer length (must be 32)
);
```

#### Encryption

```c
// Encrypt a message using timelock encryption
TimelockResult timelock_encrypt(
    const uint8_t* message,           // Message to encrypt
    size_t message_len,               // Message length
    const uint8_t* identity,          // Identity (32 bytes)
    size_t identity_len,              // Identity length (must be 32)
    const char* public_key_hex,       // Beacon public key (hex string)
    const uint8_t* secret_key,        // Ephemeral secret key (32 bytes)
    TimelockCiphertext** ciphertext_out // Output ciphertext (must free)
);
```

#### Decryption

```c
// Decrypt a timelock-encrypted message
TimelockResult timelock_decrypt(
    const TimelockCiphertext* ciphertext, // Encrypted data
    const char* signature_hex,            // Beacon signature (hex string)
    uint8_t* plaintext_out,              // Output buffer
    size_t* plaintext_len                // Buffer length (updated)
);
```

#### Memory Management

```c
// Free allocated ciphertext (REQUIRED)
void timelock_ciphertext_free(TimelockCiphertext* ciphertext);
```

#### Utility Functions

```c
// Get library version
const char* timelock_get_version(void);

// Get last error message (reserved for future use)
const char* timelock_get_last_error(void);
```

## Integration

### CMake

```cmake
# Find the library in component-level target directory
set(TIMELOCK_FFI_DIR "${CMAKE_CURRENT_SOURCE_DIR}/../timelock-ffi/target/release")
find_library(TIMELOCK_FFI_LIB timelock_ffi PATHS ${TIMELOCK_FFI_DIR})
find_path(TIMELOCK_FFI_INCLUDE timelock.h PATHS "${CMAKE_CURRENT_SOURCE_DIR}/../timelock-ffi")

# Link to your target
target_link_libraries(your_target ${TIMELOCK_FFI_LIB})
target_include_directories(your_target PRIVATE ${TIMELOCK_FFI_INCLUDE})

# Platform-specific libraries
if(WIN32)
    target_link_libraries(your_target ws2_32 userenv advapi32 kernel32 ntdll bcrypt)
elseif(APPLE)
    target_link_libraries(your_target "-framework Security" "-framework CoreFoundation")
else()
    target_link_libraries(your_target pthread dl m)
endif()
```

### Manual Compilation

```bash
# Direct compilation (adjust paths as needed)
gcc -I../timelock-ffi -L../timelock-ffi/target/release -o app app.c -ltimelock_ffi

# With platform-specific libraries
# Linux:
gcc -I../timelock-ffi -L../timelock-ffi/target/release -o app app.c -ltimelock_ffi -lpthread -ldl -lm

# macOS:
gcc -I../timelock-ffi -L../timelock-ffi/target/release -o app app.c -ltimelock_ffi -framework Security -framework CoreFoundation

# Windows (with Visual Studio):
cl /I..\timelock-ffi app.c ..\timelock-ffi\target\release\timelock_ffi.lib ws2_32.lib userenv.lib advapi32.lib kernel32.lib ntdll.lib bcrypt.lib
```

## Platform Support

| Platform | Static Library | Dynamic Library | Tested |
|----------|---------------|-----------------|---------|
| Linux x86_64 | ✅ `.a` | ✅ `.so` | ✅ |
| macOS x86_64 | ✅ `.a` | ✅ `.dylib` | ✅ |
| macOS ARM64 | ✅ `.a` | ✅ `.dylib` | ✅ |
| Windows x86_64 | ✅ `.lib` | ✅ `.dll` | ✅ |
| Linux ARM64 | ✅ `.a` | ✅ `.so` | 🧪 |
| Linux ARM32 | ✅ `.a` | ✅ `.so` | 🧪 |

## Security Considerations

### Memory Safety

- Always call `timelock_ciphertext_free()` to prevent memory leaks
- The library handles memory allocation internally and transfers ownership to C
- Input validation is performed, but additional application-level checks are recommended

### Key Management

- Secret keys should be securely generated and stored
- Consider using secure memory allocation for sensitive data
- Clear sensitive data from memory when no longer needed

### Side-Channel Resistance

- The library attempts to prevent timing attacks in critical operations
- Consider your application's threat model when using in security-critical contexts

## Error Handling

All functions return `TimelockResult` status codes:

- **`Success`**: Operation completed successfully
- **`InvalidInput`**: Null pointers or invalid parameter values
- **`EncryptionFailed`**: Encryption operation failed
- **`DecryptionFailed`**: Decryption operation failed (wrong signature/corrupted data)
- **`MemoryError`**: Memory allocation failed or buffer too small
- **`SerializationError`**: Data serialization/deserialization failed
- **`InvalidPublicKey`**: Public key format is invalid or malformed
- **`InvalidSignature`**: Signature format is invalid or malformed

## Testing

```bash
# Run Rust tests
cargo test --manifest-path timelock-ffi/Cargo.toml --target-dir timelock-ffi/target

# Run with memory leak detection (Linux)
valgrind --leak-check=full cargo test --manifest-path timelock-ffi/Cargo.toml --target-dir timelock-ffi/target

# Build FFI library (from workspace root)
cd timelock-ffi
cargo build --release --target-dir target
cbindgen --config cbindgen.toml --crate timelock-ffi --output timelock.h
```

## Contributing

1. Ensure all tests pass: `cargo test`
2. Check formatting: `cargo fmt --check`
3. Run clippy: `cargo clippy -- -D warnings`
4. Test C examples on your platform
5. Update documentation if needed

## Supported Beacons

- **Drand Quicknet**: BLS12-381 curve, 48-byte signatures ✅ **Currently Supported**

### Current Implementation

The FFI currently supports **Drand QuickNet** using the BLS12-381 curve. This provides immediate access to a live randomness beacon for timelock encryption applications.

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](../LICENSE) for details.
