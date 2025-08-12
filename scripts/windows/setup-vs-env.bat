@echo off
REM Setup Visual Studio 2022 environment for C++ development
REM This batch file can be run to set up the complete VS environment

echo Setting up Visual Studio 2022 environment...
call "C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Auxiliary\Build\vcvars64.bat"

echo.
echo Environment setup complete!
echo You can now use cl, link, and other VS tools.
echo.
echo Test compilation example:
echo cl /I. your_program.c your_library.lib /Fe:your_program.exe
echo.
