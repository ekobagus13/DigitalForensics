@echo off
echo Building TriageIR Professional Edition
echo =====================================

REM Check if Rust is installed
where cargo >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo ERROR: Rust/Cargo not found!
    echo Please install Rust from https://rustup.rs/
    pause
    exit /b 1
)

echo ✓ Rust/Cargo found

REM Clean previous builds
echo Cleaning previous builds...
cargo clean

REM Copy professional main to replace simple main
echo Preparing professional build...
copy src\main_professional.rs src\main_temp.rs >nul
copy src\main.rs src\main_simple_backup.rs >nul
copy src\main_professional.rs src\main.rs >nul

REM Build professional version
echo Building professional version...
cargo build --release --features professional

if %ERRORLEVEL% NEQ 0 (
    echo ERROR: Professional build failed!
    REM Restore original main
    copy src\main_simple_backup.rs src\main.rs >nul
    del src\main_temp.rs >nul 2>nul
    del src\main_simple_backup.rs >nul 2>nul
    pause
    exit /b 1
)

REM Restore original main
copy src\main_simple_backup.rs src\main.rs >nul
del src\main_temp.rs >nul 2>nul
del src\main_simple_backup.rs >nul 2>nul

echo.
echo ===================================
echo PROFESSIONAL BUILD COMPLETED!
echo ===================================
echo.

REM Show build results
if exist "target\release\triageir-cli.exe" (
    echo Professional executable: target\release\triageir-cli.exe
    echo Size:
    dir "target\release\triageir-cli.exe" | find "triageir-cli.exe"
    echo.
    
    echo Testing professional build...
    target\release\triageir-cli.exe --version
    
    if %ERRORLEVEL% EQU 0 (
        echo ✓ Professional build test successful
    ) else (
        echo ✗ Professional build test failed
    )
) else (
    echo ERROR: No executable found
)

echo.
echo Professional TriageIR Features:
echo • Advanced forensic artifact collection
echo • Prefetch file analysis
echo • Shimcache parsing
echo • Scheduled tasks enumeration
echo • Secure evidence packaging
echo • Chain of custody documentation
echo • Password-protected archives
echo • SHA-256 integrity verification
echo • Professional audit logging
echo.

echo Usage Examples:
echo   Basic collection:
echo   target\release\triageir-cli.exe -c CASE001 --collector-name "John Doe" --collector-org "ACME Corp" --collector-contact "john@acme.com"
echo.
echo   Verbose collection with custom output:
echo   target\release\triageir-cli.exe -c CASE001 --collector-name "John Doe" --collector-org "ACME Corp" --collector-contact "john@acme.com" -v -o C:\Evidence
echo.
echo   Quick collection (skip time-intensive artifacts):
echo   target\release\triageir-cli.exe -c CASE001 --collector-name "John Doe" --collector-org "ACME Corp" --collector-contact "john@acme.com" -q
echo.

pause