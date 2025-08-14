@echo off
REM TriageIR USB Deployment Testing Script
REM Tests USB portable package functionality

setlocal enabledelayedexpansion

echo ========================================
echo TriageIR USB Deployment Testing
echo ========================================
echo.

set USB_PATH=%1
if "%USB_PATH%"=="" (
    echo Usage: test-usb-deployment.bat [usb-path]
    echo Example: test-usb-deployment.bat F:\
    echo Example: test-usb-deployment.bat build\TriageIR-USB-Portable\
    exit /b 1
)

if not exist "%USB_PATH%" (
    echo ERROR: USB path not found: %USB_PATH%
    exit /b 1
)

echo Testing USB deployment at: %USB_PATH%
echo.

set TESTS_PASSED=0
set TESTS_FAILED=0

REM Test 1: Check required files
echo [1/10] Checking required files...
set REQUIRED_FILES=TriageIR-USB.bat TriageIR-CLI.bat TriageIR-GUI.bat Quick-Scan.bat CLI\triageir-cli.exe

for %%F in (%REQUIRED_FILES%) do (
    if exist "%USB_PATH%\%%F" (
        echo   ✓ %%F
        set /a TESTS_PASSED+=1
    ) else (
        echo   ✗ %%F - MISSING
        set /a TESTS_FAILED+=1
    )
)

REM Test 2: Check directory structure
echo.
echo [2/10] Checking directory structure...
set REQUIRED_DIRS=CLI GUI Output Logs Tools docs examples

for %%D in (%REQUIRED_DIRS%) do (
    if exist "%USB_PATH%\%%D" (
        echo   ✓ %%D\
        set /a TESTS_PASSED+=1
    ) else (
        echo   ✗ %%D\ - MISSING
        set /a TESTS_FAILED+=1
    )
)

REM Test 3: Test CLI executable
echo.
echo [3/10] Testing CLI executable...
if exist "%USB_PATH%\CLI\triageir-cli.exe" (
    "%USB_PATH%\CLI\triageir-cli.exe" --version >nul 2>&1
    if !ERRORLEVEL! equ 0 (
        echo   ✓ CLI executable runs successfully
        set /a TESTS_PASSED+=1
    ) else (
        echo   ✗ CLI executable failed to run
        set /a TESTS_FAILED+=1
    )
) else (
    echo   ✗ CLI executable not found
    set /a TESTS_FAILED+=1
)

REM Test 4: Test CLI help
echo.
echo [4/10] Testing CLI help system...
if exist "%USB_PATH%\CLI\triageir-cli.exe" (
    "%USB_PATH%\CLI\triageir-cli.exe" --help | find "TriageIR" >nul
    if !ERRORLEVEL! equ 0 (
        echo   ✓ CLI help system working
        set /a TESTS_PASSED+=1
    ) else (
        echo   ✗ CLI help system failed
        set /a TESTS_FAILED+=1
    )
) else (
    echo   ✗ CLI executable not available for testing
    set /a TESTS_FAILED+=1
)

REM Test 5: Test portable mode detection
echo.
echo [5/10] Testing portable mode detection...
set TRIAGEIR_PORTABLE=1
set TRIAGEIR_USB_DRIVE=%USB_PATH%
set TRIAGEIR_OUTPUT_DIR=%USB_PATH%\Output

if exist "%USB_PATH%\CLI\triageir-cli.exe" (
    "%USB_PATH%\CLI\triageir-cli.exe" --only system --output "%USB_PATH%\Output\test_portable.json" >nul 2>&1
    if !ERRORLEVEL! equ 0 (
        if exist "%USB_PATH%\Output\test_portable.json" (
            echo   ✓ Portable mode working
            set /a TESTS_PASSED+=1
            del "%USB_PATH%\Output\test_portable.json" >nul 2>&1
        ) else (
            echo   ✗ Portable mode output failed
            set /a TESTS_FAILED+=1
        )
    ) else (
        echo   ✗ Portable mode execution failed
        set /a TESTS_FAILED+=1
    )
) else (
    echo   ✗ CLI not available for portable mode test
    set /a TESTS_FAILED+=1
)

REM Test 6: Test launcher scripts
echo.
echo [6/10] Testing launcher scripts...
set LAUNCHER_SCRIPTS=TriageIR-USB.bat TriageIR-CLI.bat TriageIR-GUI.bat Quick-Scan.bat

for %%L in (%LAUNCHER_SCRIPTS%) do (
    if exist "%USB_PATH%\%%L" (
        REM Check if script contains required environment variables
        find "TRIAGEIR_PORTABLE" "%USB_PATH%\%%L" >nul
        if !ERRORLEVEL! equ 0 (
            echo   ✓ %%L (portable mode configured)
            set /a TESTS_PASSED+=1
        ) else (
            echo   ✗ %%L (missing portable mode configuration)
            set /a TESTS_FAILED+=1
        )
    ) else (
        echo   ✗ %%L - MISSING
        set /a TESTS_FAILED+=1
    )
)

REM Test 7: Test GUI files
echo.
echo [7/10] Testing GUI files...
if exist "%USB_PATH%\GUI" (
    dir "%USB_PATH%\GUI\*.exe" >nul 2>&1
    if !ERRORLEVEL! equ 0 (
        echo   ✓ GUI executable found
        set /a TESTS_PASSED+=1
    ) else (
        echo   ✗ GUI executable not found
        set /a TESTS_FAILED+=1
    )
) else (
    echo   ✗ GUI directory not found
    set /a TESTS_FAILED+=1
)

REM Test 8: Test documentation
echo.
echo [8/10] Testing documentation...
set DOC_FILES=README-USB.md DEPLOY-TO-USB.md docs\USB_DEPLOYMENT_GUIDE.md

for %%D in (%DOC_FILES%) do (
    if exist "%USB_PATH%\%%D" (
        echo   ✓ %%D
        set /a TESTS_PASSED+=1
    ) else (
        echo   ✗ %%D - MISSING
        set /a TESTS_FAILED+=1
    )
)

REM Test 9: Test utilities
echo.
echo [9/10] Testing utilities...
if exist "%USB_PATH%\Tools" (
    dir "%USB_PATH%\Tools\*.bat" >nul 2>&1
    if !ERRORLEVEL! equ 0 (
        echo   ✓ Utility scripts found
        set /a TESTS_PASSED+=1
    ) else (
        echo   ✗ Utility scripts not found
        set /a TESTS_FAILED+=1
    )
) else (
    echo   ✗ Tools directory not found
    set /a TESTS_FAILED+=1
)

REM Test 10: Test autorun configuration
echo.
echo [10/10] Testing autorun configuration...
if exist "%USB_PATH%\autorun.inf" (
    find "TriageIR" "%USB_PATH%\autorun.inf" >nul
    if !ERRORLEVEL! equ 0 (
        echo   ✓ Autorun configuration present
        set /a TESTS_PASSED+=1
    ) else (
        echo   ✗ Autorun configuration malformed
        set /a TESTS_FAILED+=1
    )
) else (
    echo   ⚠ Autorun configuration not found (optional)
    set /a TESTS_PASSED+=1
)

REM Calculate results
set /a TOTAL_TESTS=!TESTS_PASSED! + !TESTS_FAILED!

echo.
echo ========================================
echo USB Deployment Test Results
echo ========================================
echo.
echo Total Tests: %TOTAL_TESTS%
echo Passed: %TESTS_PASSED%
echo Failed: %TESTS_FAILED%
echo.

if %TESTS_FAILED% equ 0 (
    echo ✅ ALL TESTS PASSED
    echo USB deployment is ready for use!
    echo.
    
    echo Usage Instructions:
    echo ==================
    echo 1. Insert USB drive into target system
    echo 2. Navigate to USB drive (e.g., F:\)
    echo 3. Run TriageIR-USB.bat for interactive mode
    echo 4. Or run Quick-Scan.bat for immediate scan
    echo 5. Results will be saved to Output folder
    echo.
    
    echo Features Verified:
    echo - Zero installation required
    echo - Portable mode detection working
    echo - All launchers configured properly
    echo - CLI and GUI components present
    echo - Documentation included
    echo - Utilities available
    echo.
    
    exit /b 0
) else (
    echo ❌ USB DEPLOYMENT TEST FAILED
    echo %TESTS_FAILED% test(s) failed. Please fix issues before deployment.
    echo.
    
    echo Common Issues:
    echo - Missing files: Check USB package creation process
    echo - CLI execution failed: Verify static linking and dependencies
    echo - Launcher scripts: Check environment variable configuration
    echo - Directory structure: Ensure all required folders exist
    echo.
    
    exit /b 1
)

REM Cleanup environment variables
set TRIAGEIR_PORTABLE=
set TRIAGEIR_USB_DRIVE=
set TRIAGEIR_OUTPUT_DIR=