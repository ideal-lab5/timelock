# Building Guide

## Quick Start

The simplest way to build and test the examples:

### Windows
1. Install CMake system-wide:
   ```powershell
   winget install Kitware.CMake
   ```
   Or download from [cmake.org](https://cmake.org/download/)

2. Build and run:
   ```powershell
   mkdir build
   cd build
   cmake ..
   cmake --build . --config Release
   .\bin\Release\basic_example.exe
   ```

### Linux/macOS
1. Install CMake:
   ```bash
   # Ubuntu/Debian
   sudo apt install cmake
   
   # macOS
   brew install cmake
   ```

2. Build and run:
   ```bash
   mkdir build
   cd build
   cmake ..
   cmake --build . --config Release
   ./bin/basic_example
   ```

## Alternative: Make (Unix-like systems)

```bash
make
./target/basic_example
```

## Note

**Do not install CMake locally in the project directory** as it will create thousands of untracked files in git. Always use system-wide installations.
