@echo off
REM TriageIR Deployment Validation Script
REM Validates deployment package integrity and functionality

setlocal enabledelayedexpansion

echo ========================================
echo TriageIR Deployment Validation
echo ========================================
echo.

set VALIDATION_PASSED=0
set VALIDATION_FAILED=0
set PACKAGE_DIR=%1

if "%PACKAGE_DIR%"=="" (
    echo Usage: validate-deployment.bat [package-directory]
    echo Example: validate-deployment.bat build\TriageIR-v1.0.0
    exit /b 1
)

if not exist "%PACKAGE_DIR%" (
    echo ERROR: Package directory not found: %PACKAGE_DIR%
    exit /b 1
)

echo Validating package: %PACKAGE_DIR%
echo.

REM Test 1: Check required files
echo [1/10] Checking required files...
set REQUIRED_FILES=CLI\triageir-cli.exe GUI\TriageIR.exe docs\USER_MANUAL.md README.md VERSION.txt

for %%F in (%REQUIRED_FILES%) do (
    if exist "%PACKAGE_DIR%\%%F" (
        echo   ✓ %%F
        set /a VALIDATION_PASSED+=1
    ) else (
        echo   ✗ %%F - MISSING
        set /a VALIDATION_FAILED+=1
    )
)

REM Test 2: Check CLI executable
echo.
echo [2/10] Testing CLI executable...
if exist "%PACKAGE_DIR%\CLI\triageir-cli.exe" (
    "%PACKAGE_DIR%\CLI\triageir-cli.exe" --version >nul 2>&1
    if !ERRORLEVEL! equ 0 (
        echo   ✓ CLI executable runs successfully
        set /a VALIDATION_PASSED+=1
    ) else (
        echo   ✗ CLI executable failed to run
        set /a VALIDATION_FAILED+=1
    )
) else (
    echo   ✗ CLI executable not found
    set /a VALIDATION_FAILED+=1
)

REM Test 3: Check CLI help output
echo.
echo [3/10] Testing CLI help system...
if exist "%PACKAGE_DIR%\CLI\triageir-cli.exe" (
    "%PACKAGE_DIR%\CLI\triageir-cli.exe" --help | find "TriageIR" >nul
    if !ERRORLEVEL! equ 0 (
        echo   ✓ CLI help system working
        set /a VALIDATION_PASSED+=1
    ) else (
        echo   ✗ CLI help system failed
        set /a VALIDATION_FAILED+=1
    )
) else (
    echo   ✗ CLI executable not available for testing
    set /a VALIDATION_FAILED+=1
)

REM Test 4: Test CLI basic functionality
echo.
echo [4/10] Testing CLI basic scan...
if exist "%PACKAGE_DIR%\CLI\triageir-cli.exe" (
    cd /d "%PACKAGE_DIR%\CLI"
    triageir-cli.exe --only system --output test_output.json >nul 2>&1
    if !ERRORLEVEL! equ 0 (
        if exist "test_output.json" (
            echo   ✓ CLI basic scan successful
            set /a VALIDATION_PASSED+=1
            del test_output.json >nul 2>&1
        ) else (
            echo   ✗ CLI scan completed but no output file
            set /a VALIDATION_FAILED+=1
        )
    ) else (
        echo   ✗ CLI basic scan failed
        set /a VALIDATION_FAILED+=1
    )
    cd /d "%~dp0.."
) else (
    echo   ✗ CLI executable not available for testing
    set /a VALIDATION_FAILED+=1
)

REM Test 5: Check GUI files
echo.
echo [5/10] Checking GUI application files...
if exist "%PACKAGE_DIR%\GUI" (
    dir "%PACKAGE_DIR%\GUI" | find "TriageIR.exe" >nul
    if !ERRORLEVEL! equ 0 (
        echo   ✓ GUI executable found
        set /a VALIDATION_PASSED+=1
    ) else (
        echo   ✗ GUI executable not found
        set /a VALIDATION_FAILED+=1
    )
) else (
    echo   ✗ GUI directory not found
    set /a VALIDATION_FAILED+=1
)

REM Test 6: Check documentation completeness
echo.
echo [6/10] Checking documentation...
set DOC_FILES=docs\USER_MANUAL.md docs\DEVELOPER_GUIDE.md docs\API_REFERENCE.md docs\INSTALLATION_GUIDE.md docs\QUICK_START_GUIDE.md

for %%D in (%DOC_FILES%) do (
    if exist "%PACKAGE_DIR%\%%D" (
        echo   ✓ %%D
        set /a VALIDATION_PASSED+=1
    ) else (
        echo   ✗ %%D - MISSING
        set /a VALIDATION_FAILED+=1
    )
)

REM Test 7: Check launcher scripts
echo.
echo [7/10] Checking launcher scripts...
set LAUNCHER_FILES=TriageIR-CLI.bat TriageIR-GUI.bat Quick-Start.bat

for %%L in (%LAUNCHER_FILES%) do (
    if exist "%PACKAGE_DIR%\%%L" (
        echo   ✓ %%L
        set /a VALIDATION_PASSED+=1
    ) else (
        echo   ✗ %%L - MISSING
        set /a VALIDATION_FAILED+=1
    )
)

REM Test 8: Check test scripts
echo.
echo [8/10] Checking test scripts...
if exist "%PACKAGE_DIR%\test-scripts" (
    dir "%PACKAGE_DIR%\test-scripts\*.bat" >nul 2>&1
    if !ERRORLEVEL! equ 0 (
        echo   ✓ Test scripts directory populated
        set /a VALIDATION_PASSED+=1
    ) else (
        echo   ✗ Test scripts directory empty
        set /a VALIDATION_FAILED+=1
    )
) else (
    echo   ✗ Test scripts directory not found
    set /a VALIDATION_FAILED+=1
)

REM Test 9: Check examples
echo.
echo [9/10] Checking examples...
if exist "%PACKAGE_DIR%\examples" (
    if exist "%PACKAGE_DIR%\examples\usage-examples.md" (
        echo   ✓ Examples directory with documentation
        set /a VALIDATION_PASSED+=1
    ) else (
        echo   ✗ Examples directory missing documentation
        set /a VALIDATION_FAILED+=1
    )
) else (
    echo   ✗ Examples directory not found
    set /a VALIDATION_FAILED+=1
)

REM Test 10: Check version information
echo.
echo [10/10] Checking version information...
if exist "%PACKAGE_DIR%\VERSION.txt" (
    find "Version:" "%PACKAGE_DIR%\VERSION.txt" >nul
    if !ERRORLEVEL! equ 0 (
        echo   ✓ Version information present
        set /a VALIDATION_PASSED+=1
    ) else (
        echo   ✗ Version information malformed
        set /a VALIDATION_FAILED+=1
    )
) else (
    echo   ✗ Version file not found
    set /a VALIDATION_FAILED+=1
)

REM Calculate totals
set /a TOTAL_TESTS=!VALIDATION_PASSED! + !VALIDATION_FAILED!

echo.
echo ========================================
echo Validation Results
echo ========================================
echo.
echo Total Tests: %TOTAL_TESTS%
echo Passed: %VALIDATION_PASSED%
echo Failed: %VALIDATION_FAILED%
echo.

if %VALIDATION_FAILED% equ 0 (
    echo ✓ ALL VALIDATIONS PASSED
    echo Package is ready for deployment!
    echo.
    
    REM Display package summary
    echo Package Summary:
    echo ================
    if exist "%PACKAGE_DIR%\VERSION.txt" (
        type "%PACKAGE_DIR%\VERSION.txt"
    )
    
    echo.
    echo Package Size:
    for /f "tokens=3" %%A in ('dir "%PACKAGE_DIR%" ^| find "File(s)"') do echo   Directory: %%A bytes
    
    exit /b 0
) else (
    echo ✗ VALIDATION FAILED
    echo %VALIDATION_FAILED% test(s) failed. Please fix issues before deployment.
    echo.
    
    echo Common Issues:
    echo - Missing files: Check build process completed successfully
    echo - CLI execution failed: Verify dependencies and permissions
    echo - GUI files missing: Check Electron build process
    echo - Documentation missing: Verify documentation generation
    echo.
    
    exit /b 1
)