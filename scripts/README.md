# Build Scripts

This directory contains platform-specific build scripts for the timelock-ffi library and examples.

## Directory Structure

```
scripts/
├── unix/                    # Unix/Linux/macOS scripts
│   ├── build-ffi.sh         # Main FFI library build script
│   ├── build-all-examples.sh # Build all C examples
│   ├── compile-basic.sh      # Compile basic example only
│   ├── compile-enhanced.sh   # Compile enhanced example only
│   └── compile-error-handling.sh # Compile error handling example only
└── windows/                 # Windows scripts
    ├── build-ffi.ps1        # PowerShell build script (recommended)
    ├── build-ffi-legacy.bat # Legacy batch script
    ├── setup-vs-env.bat     # Visual Studio environment setup
    ├── build_all_examples.bat # Build all C examples
    ├── compile_clean.bat    # Compile basic example only
    ├── compile_enhanced.bat # Compile enhanced example only
    └── compile_error_handling.bat # Compile error handling example only
```

## Usage

### From Repository Root

The simplest way to build is using the convenience scripts in the repository root:

**Unix/Linux/macOS:**
```bash
./build-ffi.sh
```

**Windows:**
```batch
.\build-ffi.bat
```

These scripts automatically detect your platform and call the appropriate platform-specific script.

### Platform-Specific Scripts

#### Unix/Linux/macOS

```bash
# Build FFI library and examples
./scripts/unix/build-ffi.sh

# Build only examples (after library is built)
./scripts/unix/build-all-examples.sh

# Build individual examples
./scripts/unix/compile-basic.sh
./scripts/unix/compile-enhanced.sh
./scripts/unix/compile-error-handling.sh
```

#### Windows

**PowerShell (Recommended):**
```powershell
# Build FFI library and examples
.\scripts\windows\build-ffi.ps1

# Build only examples (after library is built)
.\scripts\windows\build_all_examples.bat
```

**Command Prompt:**
```batch
# Setup Visual Studio environment (run once per session)
.\scripts\windows\setup-vs-env.bat

# Build FFI library and examples
.\scripts\windows\build-ffi-legacy.bat

# Build individual examples
.\scripts\windows\compile_clean.bat
.\scripts\windows\compile_enhanced.bat
.\scripts\windows\compile_error_handling.bat
```

## Requirements

### Unix/Linux/macOS

- Rust toolchain (cargo)
- C compiler (gcc or clang)
- make (optional, for using Makefile)

**Platform-specific libraries:**
- Linux: pthread, dl, m
- macOS: Security framework, CoreFoundation framework

### Windows

- Rust toolchain (cargo)
- Visual Studio Build Tools or Visual Studio Community
- Windows SDK

**Required for linking:**
- ws2_32.lib, userenv.lib, advapi32.lib, kernel32.lib, ntdll.lib

## Features

- **Cross-platform compatibility**: Scripts work on Windows, Linux, and macOS
- **Automatic dependency detection**: Scripts check for required tools
- **Platform-specific linking**: Automatically includes correct libraries for each platform
- **Error handling**: Scripts exit with meaningful error messages
- **Progress reporting**: Clear indication of build progress and results

## Adding New Scripts

When adding new build scripts:

1. Create both Unix (`.sh`) and Windows (`.bat` or `.ps1`) versions
2. Place them in the appropriate platform directory
3. Use consistent naming conventions
4. Include proper error handling and dependency checks
5. Add documentation to this README

## Migration from Legacy Scripts

The old batch files have been moved from the repository root and examples directories to maintain better organization:

- `build_ffi.bat` → `scripts/windows/build-ffi-legacy.bat`
- `setup_vs_env.bat` → `scripts/windows/setup-vs-env.bat`
- `examples/timelock-ffi/*.bat` → `scripts/windows/`

The new PowerShell script `scripts/windows/build-ffi.ps1` is the recommended way to build on Windows, as it provides better error handling and more modern PowerShell features.
