@echo off
REM TriageIR Project Cleanup Script
REM Removes build artifacts, temporary files, and unnecessary directories

echo ========================================
echo TriageIR Project Cleanup
echo ========================================
echo.

echo Cleaning build artifacts...

REM Remove Rust build artifacts
if exist "TriageIR-CLI\target" (
    echo Removing Rust target directory...
    rmdir /s /q "TriageIR-CLI\target"
)

REM Remove Node.js dependencies (can be reinstalled with npm install)
if exist "TriageIR-GUI\node_modules" (
    echo Removing Node.js modules...
    rmdir /s /q "TriageIR-GUI\node_modules"
)

REM Remove GUI build artifacts
if exist "TriageIR-GUI\dist" (
    echo Removing GUI build directory...
    rmdir /s /q "TriageIR-GUI\dist"
)

REM Remove deployment build directory
if exist "build" (
    echo Removing deployment build directory...
    rmdir /s /q "build"
)

REM Remove test results
if exist "test-results" (
    echo Removing test results...
    rmdir /s /q "test-results"
)

echo.
echo Cleaning temporary files...

REM Remove temporary files
del /q /s *.tmp 2>nul
del /q /s *.temp 2>nul
del /q /s *.log 2>nul
del /q /s *.pdb 2>nul

REM Remove test executables (but not the main CLI)
del /q "TriageIR-CLI\test_*.exe" 2>nul
del /q "TriageIR-CLI\test_*.pdb" 2>nul

echo.
echo ========================================
echo Project Cleanup Complete
echo ========================================
echo.
echo Removed:
echo - Build artifacts (target/, node_modules/, dist/)
echo - Temporary files (*.tmp, *.temp, *.log)
echo - Test executables and debug symbols
echo - Test results directory
echo.
echo To rebuild:
echo - CLI: cd TriageIR-CLI && cargo build --release
echo - GUI: cd TriageIR-GUI && npm install && npm run build
echo.
pause