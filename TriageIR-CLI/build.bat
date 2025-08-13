@echo off
REM Build script for TriageIR CLI on Windows

echo Building TriageIR CLI...

REM Check if Rust is installed
where cargo >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo Error: Rust/Cargo not found. Please install Rust from https://rustup.rs/
    exit /b 1
)

REM Clean previous builds
echo Cleaning previous builds...
cargo clean

REM Run tests first
echo Running tests...
cargo test
if %ERRORLEVEL% NEQ 0 (
    echo Error: Tests failed. Please fix issues before building.
    exit /b 1
)

REM Build debug version
echo Building debug version...
cargo build
if %ERRORLEVEL% NEQ 0 (
    echo Error: Debug build failed.
    exit /b 1
)

REM Build release version
echo Building optimized release version...
cargo build --release
if %ERRORLEVEL% NEQ 0 (
    echo Error: Release build failed.
    exit /b 1
)

echo.
echo Build completed successfully!
echo.
echo Debug executable:   target\debug\triageir-cli.exe
echo Release executable: target\release\triageir-cli.exe
echo.
echo To test the executable:
echo   target\release\triageir-cli.exe --help
echo   target\release\triageir-cli.exe --verbose --output test_results.json
echo.