@echo off
REM Compile enhanced_example.c with Visual Studio toolchain
REM This batch file ensures proper environment setup

echo Compiling enhanced_example.c...
call "C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Auxiliary\Build\vcvars64.bat"

echo.
echo Compiling enhanced_example.c...
cl enhanced_example.c timelock_ffi.lib /I. /Fe:enhanced_example.exe ntdll.lib bcrypt.lib advapi32.lib

echo.
if exist enhanced_example.exe (
    echo Compilation successful! Created enhanced_example.exe
    echo.
    echo Testing enhanced example...
    echo ========================
    enhanced_example.exe
    echo.
    echo Testing with custom arguments...
    echo ===============================
    enhanced_example.exe "Windows Custom Message" 1500
    echo.
    echo Testing help flag...
    echo ===================
    enhanced_example.exe --help
) else (
    echo Compilation failed!
    exit /b 1
)
