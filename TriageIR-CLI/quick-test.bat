@echo off
echo Quick Test of TriageIR CLI
echo ==========================

REM Check if release executable exists
if not exist "target\release\triageir-cli.exe" (
    echo Release executable not found. Building first...
    call build.bat
    if %ERRORLEVEL% NEQ 0 (
        echo Build failed!
        pause
        exit /b 1
    )
)

echo.
echo Testing CLI functionality...
echo.

REM Test 1: Help option
echo Test 1: Help option
echo --------------------
target\release\triageir-cli.exe --help
echo.

REM Test 2: Version option
echo Test 2: Version option
echo ----------------------
target\release\triageir-cli.exe --version
echo.

REM Test 3: Basic scan with output file
echo Test 3: Basic scan with output file
echo ---------------------------------------------
echo Running: target\release\triageir-cli.exe --output quick-test.json
target\release\triageir-cli.exe --output quick-test.json
if %ERRORLEVEL% EQU 0 (
    echo ✓ Quick scan completed successfully
    if exist "quick-test.json" (
        echo ✓ Output file created
        echo File size:
        dir quick-test.json | find "quick-test.json"
        echo.
        echo First few lines of output:
        powershell "Get-Content quick-test.json | Select-Object -First 10"
    ) else (
        echo ✗ Output file not created
    )
) else (
    echo ✗ Quick scan failed with exit code %ERRORLEVEL%
)
echo.

REM Test 4: Validate output if Python is available
echo Test 4: Validate output
echo -----------------------
if exist "quick-test.json" (
    where python >nul 2>nul
    if %ERRORLEVEL% EQU 0 (
        echo Running validation...
        python validate_output.py quick-test.json
    ) else (
        echo Python not found, skipping validation
        echo You can manually check the JSON structure in quick-test.json
    )
) else (
    echo No output file to validate
)

echo.
echo ===========================
echo QUICK TEST COMPLETED
echo ===========================
echo.
echo If all tests passed, the CLI is working correctly!
echo You can now test with the GUI or run more comprehensive scans.
echo.
pause