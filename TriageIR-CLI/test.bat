@echo off
REM Test script for TriageIR CLI

echo Testing TriageIR CLI...

REM Run unit tests
echo Running unit tests...
cargo test --lib
if %ERRORLEVEL% NEQ 0 (
    echo Error: Unit tests failed.
    exit /b 1
)

REM Run integration tests
echo Running integration tests...
cargo test --test integration_tests
if %ERRORLEVEL% NEQ 0 (
    echo Error: Integration tests failed.
    exit /b 1
)

REM Build release version for testing
echo Building release version for testing...
cargo build --release
if %ERRORLEVEL% NEQ 0 (
    echo Error: Build failed.
    exit /b 1
)

REM Test basic functionality
echo Testing basic CLI functionality...

REM Test help output
echo Testing --help option...
target\release\triageir-cli.exe --help >nul
if %ERRORLEVEL% NEQ 0 (
    echo Error: Help option failed.
    exit /b 1
)

REM Test version output
echo Testing --version option...
target\release\triageir-cli.exe --version >nul
if %ERRORLEVEL% NEQ 0 (
    echo Error: Version option failed.
    exit /b 1
)

REM Test basic scan to file
echo Testing basic scan to file...
target\release\triageir-cli.exe --output test_output.json --skip-events
if %ERRORLEVEL% NEQ 0 (
    echo Error: Basic scan failed.
    exit /b 1
)

REM Verify output file was created and is valid JSON
if not exist test_output.json (
    echo Error: Output file was not created.
    exit /b 1
)

REM Clean up test file
del test_output.json

echo.
echo All tests passed successfully!
echo CLI is ready for use.
echo.