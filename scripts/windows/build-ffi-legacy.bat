@echo off
REM Copyright 2025 by Ideal Labs, LLC
REM
REM Licensed under the Apache License, Version 2.0 (the "License");
REM you may not use this file except in compliance with the License.
REM You may obtain a copy of the License at
REM
REM     http://www.apache.org/licenses/LICENSE-2.0
REM
REM Unless required by applicable law or agreed to in writing, software
REM distributed under the License is distributed on an "AS IS" BASIS,
REM WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
REM See the License for the specific language governing permissions and
REM limitations under the License.

setlocal enabledelayedexpansion

echo Building timelock-ffi and examples...

REM Check if we're in the right directory
if not exist "Cargo.toml" (
    echo ERROR: Please run this script from the timelock repository root
    exit /b 1
)

if not exist "timelock-ffi" (
    echo ERROR: timelock-ffi directory not found
    exit /b 1
)

REM Check for Cargo
where cargo >nul 2>nul
if errorlevel 1 (
    echo ERROR: cargo is required but not found in PATH
    exit /b 1
)

echo [INFO] Building timelock-ffi library...
cargo build --release --manifest-path timelock-ffi/Cargo.toml
if errorlevel 1 (
    echo ERROR: Failed to build timelock-ffi library
    exit /b 1
)

echo [INFO] Running FFI tests...
cargo test --manifest-path timelock-ffi/Cargo.toml
if errorlevel 1 (
    echo ERROR: FFI tests failed
    exit /b 1
)

REM Check that header was generated
if not exist "target\release\timelock.h" (
    echo ERROR: Header file not generated! Check cbindgen configuration.
    exit /b 1
)

echo [INFO] Generated header file: target\release\timelock.h

REM Try to build C examples if compiler is available
where cl >nul 2>nul
if not errorlevel 1 (
    echo [INFO] Building C examples with MSVC...
    cd examples\timelock-ffi
    
    REM Try CMake
    where cmake >nul 2>nul
    if not errorlevel 1 (
        echo [INFO] Building with CMake...
        if not exist build mkdir build
        cd build
        cmake .. -G "Visual Studio 16 2019" 2>nul || cmake .. -G "Visual Studio 17 2022" 2>nul || cmake ..
        if not errorlevel 1 (
            cmake --build . --config Release
            echo [INFO] Running CMake-built example...
            if exist "Release\basic_example.exe" (
                Release\basic_example.exe
            ) else (
                echo WARNING: CMake example not found
            )
        )
        cd ..
    )
    cd ..\..
) else (
    where gcc >nul 2>nul
    if not errorlevel 1 (
        echo [INFO] Building C examples with GCC...
        cd examples\timelock-ffi
        
        REM Manual compilation with GCC (MinGW)
        echo [INFO] Compiling with GCC...
        gcc -std=c11 -I..\..\target\release -o basic_example.exe basic_example.c -L..\..\target\release -ltimelock_ffi -lws2_32 -luserenv -ladvapi32 -lkernel32 -lntdll
        if not errorlevel 1 (
            echo [INFO] Running GCC-built example...
            basic_example.exe
        )
        cd ..\..
    ) else (
        echo WARNING: No C compiler found, skipping C examples
    )
)

echo [INFO] Build and test completed successfully!
echo [INFO] Generated files:
echo   - target\release\timelock_ffi.lib (static library)
if exist "target\release\timelock_ffi.dll" (
    echo   - target\release\timelock_ffi.dll (dynamic library)
)
echo   - target\release\timelock.h (C header)

echo.
echo [INFO] To use the library in your C project:
echo   1. Include target\release\timelock.h in your source
echo   2. Link against target\release\timelock_ffi.lib
echo   3. Add platform-specific libraries: -lws2_32 -luserenv -ladvapi32 -lkernel32 -lntdll
echo.
echo [INFO] See examples\timelock-ffi\README.md for detailed usage instructions

endlocal
