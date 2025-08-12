# Copyright 2025 by Ideal Labs, LLC
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

<#
.SYNOPSIS
    Build script for timelock-ffi library and examples on Windows

.DESCRIPTION
    This script builds the timelock-ffi library, runs tests, and optionally builds C examples.
    It automatically detects available compilers and sets up the build environment.

.PARAMETER SkipTests
    Skip running the Rust test suite

.PARAMETER SkipExamples
    Skip building C examples

.PARAMETER Compiler
    Force a specific compiler (msvc, gcc, clang)

.EXAMPLE
    .\build-ffi.ps1
    Build everything with default settings

.EXAMPLE
    .\build-ffi.ps1 -SkipExamples -Verbose
    Build library and run tests, skip examples, with verbose output
#>

[CmdletBinding()]
param(
    [switch]$SkipTests,
    [switch]$SkipExamples,
    [ValidateSet("msvc", "gcc", "clang")]
    [string]$Compiler
)

# Set error action preference
$ErrorActionPreference = "Stop"

# Color functions for better output
function Write-Info {
    param([string]$Message)
    Write-Host "[INFO] $Message" -ForegroundColor Green
}

function Write-Warning {
    param([string]$Message)
    Write-Host "[WARN] $Message" -ForegroundColor Yellow
}

function Write-Error {
    param([string]$Message)
    Write-Host "[ERROR] $Message" -ForegroundColor Red
}

function Test-Command {
    param([string]$Command)
    try {
        Get-Command $Command -ErrorAction Stop | Out-Null
        return $true
    } catch {
        return $false
    }
}

# Check if we're in the right directory
if (!(Test-Path "Cargo.toml") -or !(Test-Path "timelock-ffi")) {
    Write-Error "Please run this script from the timelock repository root"
    exit 1
}

Write-Info "Building timelock-ffi library and examples..."

# Check for Cargo
if (!(Test-Command "cargo")) {
    Write-Error "cargo is required but not found in PATH"
    exit 1
}

# Build the FFI library
Write-Info "Building timelock-ffi library..."
try {
    if ($Verbose) {
        cargo build --release --manifest-path timelock-ffi/Cargo.toml --verbose
    } else {
        cargo build --release --manifest-path timelock-ffi/Cargo.toml
    }
    Write-Info "Library build completed successfully"
} catch {
    Write-Error "Failed to build timelock-ffi library"
    exit 1
}

# Run tests unless skipped
if (!$SkipTests) {
    Write-Info "Running FFI tests..."
    try {
        if ($Verbose) {
            cargo test --manifest-path timelock-ffi/Cargo.toml --verbose
        } else {
            cargo test --manifest-path timelock-ffi/Cargo.toml
        }
        Write-Info "All tests passed"
    } catch {
        Write-Error "FFI tests failed"
        exit 1
    }
}

# Check that header was generated
$HeaderPath = "target\release\timelock.h"
if (!(Test-Path $HeaderPath)) {
    Write-Error "Header file not generated! Check cbindgen configuration."
    exit 1
}
Write-Info "Generated header file: $HeaderPath"

# Build C examples unless skipped
if (!$SkipExamples) {
    Write-Info "Building C examples..."
    
    # Detect available compilers
    $AvailableCompilers = @()
    if (Test-Command "cl") { $AvailableCompilers += "msvc" }
    if (Test-Command "gcc") { $AvailableCompilers += "gcc" }
    if (Test-Command "clang") { $AvailableCompilers += "clang" }
    
    if ($AvailableCompilers.Count -eq 0) {
        Write-Warning "No C compilers found, skipping C examples"
    } else {
        # Select compiler
        $SelectedCompiler = $Compiler
        if (!$SelectedCompiler) {
            $SelectedCompiler = $AvailableCompilers[0]
        }
        
        if ($SelectedCompiler -notin $AvailableCompilers) {
            Write-Warning "Requested compiler '$Compiler' not available, using '$($AvailableCompilers[0])'"
            $SelectedCompiler = $AvailableCompilers[0]
        }
        
        Write-Info "Building examples with $SelectedCompiler compiler..."
        
        Push-Location "examples\timelock-ffi"
        try {
            switch ($SelectedCompiler) {
                "msvc" {
                    # Setup MSVC environment if not already done
                    if (!(Test-Command "cl")) {
                        Write-Info "Setting up Visual Studio environment..."
                        $VSPath = "${env:ProgramFiles}\Microsoft Visual Studio\2022\Community\VC\Auxiliary\Build\vcvars64.bat"
                        if (Test-Path $VSPath) {
                            cmd /c "`"$VSPath`" && set" | ForEach-Object {
                                if ($_ -match "^([^=]+)=(.*)$") {
                                    [Environment]::SetEnvironmentVariable($matches[1], $matches[2])
                                }
                            }
                        }
                    }
                    
                    # Build with MSVC
                    Write-Info "Compiling with MSVC..."
                    & cmd /c "cl basic_example.c ..\..\target\release\timelock_ffi.lib /I..\..\target\release /Fe:basic_example.exe ntdll.lib bcrypt.lib advapi32.lib"
                    if ($LASTEXITCODE -eq 0) {
                        Write-Info "Running basic example..."
                        .\basic_example.exe
                    }
                }
                "gcc" {
                    Write-Info "Compiling with GCC..."
                    & gcc -std=c11 -I..\..\target\release -o basic_example.exe basic_example.c -L..\..\target\release -ltimelock_ffi -lws2_32 -luserenv -ladvapi32 -lkernel32 -lntdll
                    if ($LASTEXITCODE -eq 0) {
                        Write-Info "Running basic example..."
                        .\basic_example.exe
                    }
                }
                "clang" {
                    Write-Info "Compiling with Clang..."
                    & clang -std=c11 -I..\..\target\release -o basic_example.exe basic_example.c -L..\..\target\release -ltimelock_ffi -lws2_32 -luserenv -ladvapi32 -lkernel32 -lntdll
                    if ($LASTEXITCODE -eq 0) {
                        Write-Info "Running basic example..."
                        .\basic_example.exe
                    }
                }
            }
        } catch {
            Write-Warning "Failed to build C examples with $SelectedCompiler"
        } finally {
            Pop-Location
        }
    }
}

Write-Info "Build completed successfully!"
Write-Info "Generated files:"
Write-Info "  - target\release\timelock_ffi.lib (static library)"
if (Test-Path "target\release\timelock_ffi.dll") {
    Write-Info "  - target\release\timelock_ffi.dll (dynamic library)"
}
Write-Info "  - target\release\timelock.h (C header)"

Write-Host ""
Write-Info "To use the library in your C project:"
Write-Info "  1. Include target\release\timelock.h in your source"
Write-Info "  2. Link against target\release\timelock_ffi.lib"
Write-Info "  3. Add platform libraries: -lws2_32 -luserenv -ladvapi32 -lkernel32 -lntdll"
Write-Host ""
Write-Info "See examples\timelock-ffi\README.md for detailed usage instructions"
