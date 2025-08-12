@echo off
REM Enhanced compilation script for all timelock-ffi C examples
REM Copyright 2025 by Ideal Labs, LLC
REM Licensed under Apache License, Version 2.0

echo Timelock Encryption C FFI Examples - Build Script
echo ==================================================

REM Set up Visual Studio environment
echo Setting up Visual Studio environment...
call "C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Auxiliary\Build\vcvars64.bat"

echo.
echo Building all examples...

REM Navigate to examples directory
cd /d "%~dp0..\..\examples\timelock-ffi"
if errorlevel 1 (
    echo ERROR: Could not navigate to examples directory
    exit /b 1
)

REM Create target directory if it doesn't exist
if not exist "target" mkdir target

REM Check if library exists
if not exist "..\..\timelock-ffi\target\release\timelock_ffi.lib" (
    echo ERROR: timelock_ffi.lib not found!
    echo Please build the FFI library first with: cargo build --release --target-dir target --manifest-path timelock-ffi/Cargo.toml
    exit /b 1
)

REM Check if header exists
if not exist "..\..\timelock-ffi\timelock.h" (
    echo ERROR: timelock.h not found!
    echo Please generate the header with: cbindgen --config cbindgen.toml --crate timelock-ffi --output timelock.h
    exit /b 1
)

REM Build basic example
echo.
echo [1/2] Compiling basic_example.c...
cl /TC basic_example.c /I..\..\timelock-ffi /Fo:target\ /Fe:target\basic_example.exe /link ..\..\timelock-ffi\target\release\timelock_ffi.lib ws2_32.lib userenv.lib advapi32.lib kernel32.lib ntdll.lib bcrypt.lib
if errorlevel 1 (
    echo ERROR: Failed to compile basic_example.c
    exit /b 1
)
echo ✅ target\basic_example.exe created successfully

REM Build enhanced example
echo.
echo [2/2] Compiling enhanced_example.c...
cl /TC enhanced_example.c /I..\..\timelock-ffi /Fo:target\ /Fe:target\enhanced_example.exe /link ..\..\timelock-ffi\target\release\timelock_ffi.lib ws2_32.lib userenv.lib advapi32.lib kernel32.lib ntdll.lib bcrypt.lib
if errorlevel 1 (
    echo ERROR: Failed to compile enhanced_example.c
    exit /b 1
)
echo ✅ target\enhanced_example.exe created successfully

echo.
echo ============================================
echo ✅ All examples compiled successfully!
echo ============================================
echo.
echo Available executables:
echo   target\basic_example.exe        - Basic timelock encryption demo
echo   target\enhanced_example.exe     - Enhanced demo with CLI arguments
echo.
echo Usage examples:
echo   target\basic_example.exe
echo   target\enhanced_example.exe "My message" 2000
echo   target\enhanced_example.exe --help
echo.

REM Ask if user wants to run examples
set /p run_examples="Run all examples now? (y/n): "
if /i "%run_examples%" == "y" (
    echo.
    echo Running basic example...
    echo ========================
    target\basic_example.exe
    
    echo.
    echo Running enhanced example...
    echo ===========================
    target\enhanced_example.exe
    
    echo.
    echo Running enhanced example with custom arguments...
    echo ================================================
    target\enhanced_example.exe "Windows Test Message" 1500
    
    echo.
    echo ✅ All examples completed successfully!
)

echo.
echo Build script completed.
pause
