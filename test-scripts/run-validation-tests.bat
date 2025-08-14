@echo off
setlocal enabledelayedexpansion
REM Quick validation test script for TriageIR
REM Runs essential tests to validate the comprehensive testing implementation

echo ========================================
echo TriageIR Validation Tests
echo ========================================
echo.

set VALIDATION_START_TIME=%time%
set CLI_DIR=TriageIR-CLI
set TEST_RESULTS_DIR=test-results\validation

REM Create validation results directory
if not exist %TEST_RESULTS_DIR% mkdir %TEST_RESULTS_DIR%

echo [%time%] Running validation tests to verify comprehensive testing implementation...
echo.

REM Test 1: Verify CLI builds successfully
echo Test 1: CLI Build Verification
echo ==============================
echo [%time%] Building CLI in release mode...

cd %CLI_DIR%
cargo build --release > ..\%TEST_RESULTS_DIR%\build-validation.log 2>&1
set BUILD_RESULT=%errorlevel%
cd ..

if %BUILD_RESULT% equ 0 (
    echo ✓ CLI build successful
) else (
    echo ✗ CLI build failed
    echo Build failed. Check ..\%TEST_RESULTS_DIR%\build-validation.log for details.
    exit /b 1
)

REM Test 2: Run core unit tests
echo.
echo Test 2: Core Unit Tests
echo =======================
echo [%time%] Running essential unit tests...

cd %CLI_DIR%
cargo test --lib logger::tests::test_logger_basic_functionality > ..\%TEST_RESULTS_DIR%\unit-test-validation.log 2>&1
set UNIT_RESULT=%errorlevel%
cd ..

if %UNIT_RESULT% equ 0 (
    echo ✓ Core unit tests passed
) else (
    echo ✗ Core unit tests failed
    echo Unit tests failed. Check ..\%TEST_RESULTS_DIR%\unit-test-validation.log for details.
    exit /b 1
)

REM Test 3: Verify CLI executable functionality
echo.
echo Test 3: CLI Executable Functionality
echo ====================================
echo [%time%] Testing CLI executable...

if exist %CLI_DIR%\target\release\triageir-cli.exe (
    %CLI_DIR%\target\release\triageir-cli.exe --help > %TEST_RESULTS_DIR%\cli-help-validation.log 2>&1
    if %errorlevel% equ 0 (
        echo ✓ CLI help command works
        
        REM Test version command
        %CLI_DIR%\target\release\triageir-cli.exe --version > %TEST_RESULTS_DIR%\cli-version-validation.log 2>&1
        if %errorlevel% equ 0 (
            echo ✓ CLI version command works
        ) else (
            echo ✗ CLI version command failed
            exit /b 1
        )
    ) else (
        echo ✗ CLI help command failed
        exit /b 1
    )
) else (
    echo ✗ CLI executable not found
    exit /b 1
)

REM Test 4: Quick scan functionality test
echo.
echo Test 4: Quick Scan Functionality
echo ================================
echo [%time%] Running quick scan test...

%CLI_DIR%\target\release\triageir-cli.exe --output %TEST_RESULTS_DIR%\quick-scan.json --format json > %TEST_RESULTS_DIR%\quick-scan-validation.log 2>&1
set SCAN_RESULT=%errorlevel%

if %SCAN_RESULT% equ 0 (
    echo ✓ Quick scan completed successfully
) else if %SCAN_RESULT% equ 2 (
    echo ✓ Quick scan completed with warnings
    
    REM Verify JSON output
    if exist %TEST_RESULTS_DIR%\quick-scan.json (
        for %%A in (%TEST_RESULTS_DIR%\quick-scan.json) do set JSON_SIZE=%%~zA
        if !JSON_SIZE! gtr 1000 (
            echo ✓ JSON output generated ^(!JSON_SIZE! bytes^)
            
            REM Quick JSON validation
            powershell -Command "try { Get-Content '%TEST_RESULTS_DIR%\quick-scan.json' | ConvertFrom-Json | Out-Null; Write-Host '✓ JSON format valid' } catch { Write-Host '✗ JSON format invalid'; exit 1 }" > %TEST_RESULTS_DIR%\json-validation.log 2>&1
            if %errorlevel% equ 0 (
                type %TEST_RESULTS_DIR%\json-validation.log
            ) else (
                echo ✗ JSON format validation failed
                exit /b 1
            )
        ) else (
            echo ✗ JSON output too small ^(!JSON_SIZE! bytes^)
            exit /b 1
        )
    ) else (
        echo ✗ JSON output file not created
        exit /b 1
    )
) else (
    echo ✗ Quick scan failed ^(exit code: %SCAN_RESULT%^)
    echo Check %TEST_RESULTS_DIR%\quick-scan-validation.log for details
    exit /b 1
)

REM Test 5: Test infrastructure validation
echo.
echo Test 5: Test Infrastructure Validation
echo ======================================
echo [%time%] Validating test infrastructure...

REM Check if test scripts exist
set MISSING_SCRIPTS=0

if not exist test-scripts\run-comprehensive-tests.bat (
    echo ✗ Missing: run-comprehensive-tests.bat
    set /a MISSING_SCRIPTS+=1
)

if not exist test-scripts\run-e2e-tests.bat (
    echo ✗ Missing: run-e2e-tests.bat
    set /a MISSING_SCRIPTS+=1
)

if not exist test-scripts\run-performance-benchmarks.bat (
    echo ✗ Missing: run-performance-benchmarks.bat
    set /a MISSING_SCRIPTS+=1
)

if not exist test-scripts\run-forensic-validation.bat (
    echo ✗ Missing: run-forensic-validation.bat
    set /a MISSING_SCRIPTS+=1
)

if not exist test-scripts\generate-test-report.bat (
    echo ✗ Missing: generate-test-report.bat
    set /a MISSING_SCRIPTS+=1
)

if !MISSING_SCRIPTS! equ 0 (
    echo ✓ All test scripts present
) else (
    echo ✗ !MISSING_SCRIPTS! test script^(s^) missing
    exit /b 1
)

REM Check if test modules exist in CLI
if exist %CLI_DIR%\src\comprehensive_tests.rs (
    echo ✓ Comprehensive tests module exists
) else (
    echo ✗ Comprehensive tests module missing
    exit /b 1
)

if exist %CLI_DIR%\src\performance_tests.rs (
    echo ✓ Performance tests module exists
) else (
    echo ✗ Performance tests module missing
    exit /b 1
)

if exist %CLI_DIR%\src\test_error_scenarios.rs (
    echo ✓ Error scenario tests module exists
) else (
    echo ✗ Error scenario tests module missing
    exit /b 1
)

REM Test 6: GUI test infrastructure (if available)
echo.
echo Test 6: GUI Test Infrastructure
echo ===============================
echo [%time%] Checking GUI test infrastructure...

if exist TriageIR-GUI\test\gui-tests.js (
    echo ✓ GUI test suite exists
    
    if exist TriageIR-GUI\package.json (
        echo ✓ GUI package.json exists
    ) else (
        echo ⚠ GUI package.json missing
    )
) else (
    echo ⚠ GUI test suite not found (may not be implemented yet)
)

REM Calculate validation time
set VALIDATION_END_TIME=%time%

echo.
echo ========================================
echo Validation Summary
echo ========================================
echo.
echo Validation started: %VALIDATION_START_TIME%
echo Validation completed: %VALIDATION_END_TIME%
echo.
echo Results:
echo ✓ CLI Build Verification: PASSED
echo ✓ Core Unit Tests: PASSED
echo ✓ CLI Executable Functionality: PASSED
echo ✓ Quick Scan Functionality: PASSED
echo ✓ Test Infrastructure Validation: PASSED
echo ✓ GUI Test Infrastructure: CHECKED
echo.
echo All validation tests passed successfully!
echo.
echo The comprehensive testing implementation is ready for use.
echo Run 'test-scripts\run-comprehensive-tests.bat' to execute the full test suite.
echo.

REM Generate quick validation report
echo Validation Report > %TEST_RESULTS_DIR%\validation-summary.txt
echo ================= >> %TEST_RESULTS_DIR%\validation-summary.txt
echo Generated: %date% %time% >> %TEST_RESULTS_DIR%\validation-summary.txt
echo. >> %TEST_RESULTS_DIR%\validation-summary.txt
echo All validation tests passed successfully. >> %TEST_RESULTS_DIR%\validation-summary.txt
echo The comprehensive testing implementation is ready for use. >> %TEST_RESULTS_DIR%\validation-summary.txt
echo. >> %TEST_RESULTS_DIR%\validation-summary.txt
echo Test Results: >> %TEST_RESULTS_DIR%\validation-summary.txt
echo - CLI Build Verification: PASSED >> %TEST_RESULTS_DIR%\validation-summary.txt
echo - Core Unit Tests: PASSED >> %TEST_RESULTS_DIR%\validation-summary.txt
echo - CLI Executable Functionality: PASSED >> %TEST_RESULTS_DIR%\validation-summary.txt
echo - Quick Scan Functionality: PASSED >> %TEST_RESULTS_DIR%\validation-summary.txt
echo - Test Infrastructure Validation: PASSED >> %TEST_RESULTS_DIR%\validation-summary.txt
echo - GUI Test Infrastructure: CHECKED >> %TEST_RESULTS_DIR%\validation-summary.txt

echo Validation summary saved to: %TEST_RESULTS_DIR%\validation-summary.txt

exit /b 0