@echo off
REM Test compile basic_example.c with component-level target
REM This batch file ensures proper environment setup

echo Testing compilation with component-level target...
call "C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Auxiliary\Build\vcvars64.bat"

cd examples\timelock-ffi

echo.
echo Compiling basic_example.c with component-level target...
cl basic_example.c /I..\..\timelock-ffi /Fe:basic_example_component_test.exe /link ..\..\timelock-ffi\target\release\timelock_ffi.lib ws2_32.lib userenv.lib advapi32.lib kernel32.lib ntdll.lib bcrypt.lib

echo.
if exist basic_example_component_test.exe (
    echo Compilation successful! Created basic_example_component_test.exe
    echo Running the example...
    echo.
    basic_example_component_test.exe
) else (
    echo Compilation failed!
    exit /b 1
)
