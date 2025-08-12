# Cross-Platform Testing Guide for Timelock FFI

This guide provides comprehensive instructions for testing the timelock FFI build system across different platforms to ensure true cross-platform compatibility.

## 🧪 Testing Strategy

### Prerequisites
- Rust toolchain (1.70+ recommended)
- C compiler (gcc, clang, or MSVC)
- cbindgen (`cargo install cbindgen`)

## 🐧 Linux Testing

### Ubuntu/Debian
```bash
# Install dependencies
sudo apt-get update
sudo apt-get install build-essential

# Build and test
./scripts/build-ffi.sh
cd examples/timelock-ffi
make all
./target/basic_example
./target/enhanced_example "Linux test message" 2000
./target/error_handling_example
```

### Alpine Linux (minimal)
```bash
# Install dependencies
apk add --no-cache build-base

# Build and test
./scripts/build-ffi.sh
cd examples/timelock-ffi
make all
./target/basic_example
```

### CentOS/RHEL/Fedora
```bash
# Install dependencies
sudo dnf install gcc make
# OR: sudo yum install gcc make

# Build and test
./scripts/build-ffi.sh
cd examples/timelock-ffi
make all
./target/basic_example
```

## 🍎 macOS Testing

### With Xcode Command Line Tools
```bash
# Install Xcode command line tools
xcode-select --install

# Build and test
./scripts/build-ffi.sh
cd examples/timelock-ffi
make all
./target/basic_example
./target/enhanced_example "macOS test message" 1500
./target/error_handling_example
```

### With Homebrew GCC
```bash
# Install GCC via Homebrew
brew install gcc

# Build with specific compiler
CC=gcc-13 ./scripts/build-ffi.sh
cd examples/timelock-ffi
CC=gcc-13 make all
./target/basic_example
```

## 🪟 Windows Testing

### Visual Studio (Recommended)
```batch
REM Build FFI library
scripts\build-ffi.bat

REM Test examples
cd examples\timelock-ffi
.\compile_clean.bat
.\compile_enhanced.bat
.\compile_error_handling.bat
```

### MinGW/MSYS2
```bash
# In MSYS2 shell
pacman -S mingw-w64-x86_64-gcc

# Build and test
./scripts/build-ffi.sh
cd examples/timelock-ffi
make all
./target/basic_example.exe
```

## 🐳 Docker Testing

### Quick Cross-Platform Test
```bash
# Test on Ubuntu
docker run --rm -v $(pwd):/workspace -w /workspace ubuntu:22.04 bash -c "
    apt-get update && apt-get install -y build-essential curl
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source ~/.cargo/env
    cargo install cbindgen
    ./scripts/build-ffi.sh
    cd examples/timelock-ffi && make all && ./target/basic_example
"

# Test on Alpine (minimal)
docker run --rm -v $(pwd):/workspace -w /workspace alpine:latest sh -c "
    apk add --no-cache build-base curl bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source ~/.cargo/env
    cargo install cbindgen
    ./scripts/build-ffi.sh
    cd examples/timelock-ffi && make all && ./target/basic_example
"
```

## 🚀 CI/CD Testing

### GitHub Actions
The included `.github/workflows/cross-platform-ffi.yml` will test:
- Ubuntu (gcc)
- macOS (clang) 
- Windows (MSVC)

### Manual CI Testing
```bash
# Trigger workflow manually
gh workflow run cross-platform-ffi.yml

# Check status
gh run list --workflow=cross-platform-ffi.yml
```

## 🔍 Common Issues and Solutions

### Issue: Library Not Found
**Symptoms:** `cannot find -ltimelock_ffi`
**Solution:** Ensure FFI library is built first:
```bash
./scripts/build-ffi.sh
```

### Issue: Header Not Found
**Symptoms:** `timelock.h: No such file or directory`
**Solution:** Check cbindgen is installed and header generated:
```bash
cargo install cbindgen
./scripts/build-ffi.sh
```

### Issue: Runtime Linking Errors
**Symptoms:** `error while loading shared libraries`
**Linux Solution:**
```bash
export LD_LIBRARY_PATH=../../timelock-ffi/target/release:$LD_LIBRARY_PATH
```
**macOS Solution:**
```bash
export DYLD_LIBRARY_PATH=../../timelock-ffi/target/release:$DYLD_LIBRARY_PATH
```

### Issue: Windows MSVC Not Found
**Solution:** Run from Visual Studio Developer Command Prompt or:
```batch
call "C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Auxiliary\Build\vcvars64.bat"
```

## 📊 Verification Checklist

For each platform, verify:
- [ ] FFI library builds successfully
- [ ] C header is generated correctly  
- [ ] All three examples compile without errors
- [ ] Examples execute and produce expected output
- [ ] Build artifacts are organized in target/ directories
- [ ] Clean builds work after `make clean`

## 🧹 Platform-Specific Notes

### Linux
- Uses `.so` dynamic libraries or `.a` static libraries
- Requires `-pthread -ldl -lm` linking flags
- GCC is standard, clang also supported

### macOS  
- Uses `.dylib` dynamic libraries or `.a` static libraries
- Requires `-framework Security -framework CoreFoundation`
- Both Apple clang and Homebrew GCC supported

### Windows
- Uses `.dll` and `.lib` files
- Requires multiple Windows system libraries
- MSVC recommended, MinGW also supported

## 📈 Performance Testing

For comprehensive testing, also verify performance:
```bash
# Build with optimizations
RUSTFLAGS="-C target-cpu=native" cargo build --release --target-dir target

# Run with different message sizes
./target/enhanced_example "Small msg" 1000
./target/enhanced_example "$(head -c 1000 < /dev/zero | tr '\0' 'x')" 1000
```

This testing guide ensures your FFI bindings work reliably across all target platforms before submitting your cross-platform PR.
