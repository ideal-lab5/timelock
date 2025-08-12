@echo off
setlocal enabledelayedexpansion

echo Building timelock-ffi...

pushd "%~dp0..\timelock-ffi"

echo Building FFI library...
cargo build --release --target-dir target
if !errorlevel! neq 0 (
    echo Error: Failed to build FFI library
    popd
    exit /b !errorlevel!
)

echo Generating C header file...
cbindgen --config cbindgen.toml --crate timelock-ffi --output timelock.h
if !errorlevel! neq 0 (
    echo Warning: Failed to generate header file
)

popd

echo Build completed successfully!
echo Library: timelock-ffi\target\release\timelock_ffi.dll
echo Header: timelock-ffi\timelock.h
