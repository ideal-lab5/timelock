@echo off
REM Compile basic_example.c with Visual Studio toolchain
REM This batch file ensures proper environment setup

echo Setting up Visual Studio environment...
call "C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Auxiliary\Build\vcvars64.bat"

echo.
echo Compiling basic_example.c...
cl basic_example.c timelock_ffi.lib /I. /Fe:basic_example_clean.exe ntdll.lib bcrypt.lib advapi32.lib

echo.
if exist basic_example_clean.exe (
    echo Compilation successful! Created basic_example_clean.exe
    echo Running the example...
    echo.
    basic_example_clean.exe
) else (
    echo Compilation failed!
    exit /b 1
)
