@echo off
echo Testing TriageIR CLI Build Process
echo ===================================

REM Check if Rust is installed
echo Checking for Rust installation...
where cargo >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo.
    echo ERROR: Rust/Cargo not found!
    echo.
    echo Please install Rust from: https://rustup.rs/
    echo.
    echo After installation, restart your command prompt and run this script again.
    echo.
    pause
    exit /b 1
)

echo ✓ Rust/Cargo found

REM Check Rust version
echo.
echo Rust version:
cargo --version
rustc --version

REM Check if we're on Windows
echo.
echo Checking platform...
if "%OS%"=="Windows_NT" (
    echo ✓ Windows platform detected
) else (
    echo ⚠ Non-Windows platform - some features may not work
)

REM Clean previous builds
echo.
echo Cleaning previous builds...
cargo clean

REM Check dependencies
echo.
echo Checking dependencies...
cargo check
if %ERRORLEVEL% NEQ 0 (
    echo.
    echo ERROR: Dependency check failed!
    echo Please check your Cargo.toml and internet connection.
    pause
    exit /b 1
)

echo ✓ Dependencies OK

REM Run tests
echo.
echo Running tests...
cargo test --lib
if %ERRORLEVEL% NEQ 0 (
    echo.
    echo ERROR: Tests failed!
    echo Please fix the failing tests before building.
    pause
    exit /b 1
)

echo ✓ Tests passed

REM Build debug version
echo.
echo Building debug version...
cargo build
if %ERRORLEVEL% NEQ 0 (
    echo.
    echo ERROR: Debug build failed!
    pause
    exit /b 1
)

echo ✓ Debug build successful

REM Test debug executable
echo.
echo Testing debug executable...
if exist "target\debug\triageir-cli.exe" (
    echo ✓ Debug executable created
    echo.
    echo Testing --help option...
    target\debug\triageir-cli.exe --help
    echo.
    echo Testing --version option...
    target\debug\triageir-cli.exe --version
) else (
    echo ✗ Debug executable not found
    exit /b 1
)

REM Build release version
echo.
echo Building optimized release version...
cargo build --release
if %ERRORLEVEL% NEQ 0 (
    echo.
    echo ERROR: Release build failed!
    pause
    exit /b 1
)

echo ✓ Release build successful

REM Test release executable
echo.
echo Testing release executable...
if exist "target\release\triageir-cli.exe" (
    echo ✓ Release executable created
    echo.
    echo File size:
    dir target\release\triageir-cli.exe | find "triageir-cli.exe"
    echo.
    echo Testing --help option...
    target\release\triageir-cli.exe --help
    echo.
    echo Testing --version option...
    target\release\triageir-cli.exe --version
) else (
    echo ✗ Release executable not found
    exit /b 1
)

echo.
echo ===================================
echo BUILD SUCCESSFUL!
echo ===================================
echo.
echo Debug executable:   target\debug\triageir-cli.exe
echo Release executable: target\release\triageir-cli.exe
echo.
echo You can now test the CLI with:
echo   target\release\triageir-cli.exe --help
echo   target\release\triageir-cli.exe --verbose --output test-results.json
echo.
echo To validate output:
echo   python validate_output.py test-results.json
echo.
pause