@echo off
REM Comprehensive testing script for TriageIR forensic framework
REM This script runs all test suites and generates a comprehensive test report

echo ========================================
echo TriageIR Comprehensive Testing Suite
echo ========================================
echo.

set TEST_START_TIME=%time%
set TEST_DATE=%date%
set TEST_RESULTS_DIR=test-results
set CLI_DIR=TriageIR-CLI
set GUI_DIR=TriageIR-GUI

REM Create test results directory
if not exist %TEST_RESULTS_DIR% mkdir %TEST_RESULTS_DIR%

echo [%time%] Starting comprehensive testing...
echo Test started: %TEST_DATE% %TEST_START_TIME% > %TEST_RESULTS_DIR%\test-summary.txt
echo. >> %TEST_RESULTS_DIR%\test-summary.txt

REM Initialize test counters
set TOTAL_TESTS=0
set PASSED_TESTS=0
set FAILED_TESTS=0

echo ========================================
echo 1. CLI Unit Tests
echo ========================================
echo.

cd %CLI_DIR%
echo [%time%] Running CLI unit tests...
cargo test --lib > ..\%TEST_RESULTS_DIR%\cli-unit-tests.log 2>&1
if %errorlevel% equ 0 (
    echo ✓ CLI unit tests PASSED
    echo CLI Unit Tests: PASSED >> ..\%TEST_RESULTS_DIR%\test-summary.txt
    set /a PASSED_TESTS+=1
) else (
    echo ✗ CLI unit tests FAILED
    echo CLI Unit Tests: FAILED >> ..\%TEST_RESULTS_DIR%\test-summary.txt
    set /a FAILED_TESTS+=1
)
set /a TOTAL_TESTS+=1
cd ..

echo.
echo ========================================
echo 2. CLI Integration Tests
echo ========================================
echo.

cd %CLI_DIR%
echo [%time%] Running CLI integration tests...
cargo test integration_tests > ..\%TEST_RESULTS_DIR%\cli-integration-tests.log 2>&1
if %errorlevel% equ 0 (
    echo ✓ CLI integration tests PASSED
    echo CLI Integration Tests: PASSED >> ..\%TEST_RESULTS_DIR%\test-summary.txt
    set /a PASSED_TESTS+=1
) else (
    echo ✗ CLI integration tests FAILED
    echo CLI Integration Tests: FAILED >> ..\%TEST_RESULTS_DIR%\test-summary.txt
    set /a FAILED_TESTS+=1
)
set /a TOTAL_TESTS+=1
cd ..

echo.
echo ========================================
echo 3. CLI Comprehensive Tests
echo ========================================
echo.

cd %CLI_DIR%
echo [%time%] Running CLI comprehensive tests...
cargo test comprehensive_tests > ..\%TEST_RESULTS_DIR%\cli-comprehensive-tests.log 2>&1
if %errorlevel% equ 0 (
    echo ✓ CLI comprehensive tests PASSED
    echo CLI Comprehensive Tests: PASSED >> ..\%TEST_RESULTS_DIR%\test-summary.txt
    set /a PASSED_TESTS+=1
) else (
    echo ✗ CLI comprehensive tests FAILED
    echo CLI Comprehensive Tests: FAILED >> ..\%TEST_RESULTS_DIR%\test-summary.txt
    set /a FAILED_TESTS+=1
)
set /a TOTAL_TESTS+=1
cd ..

echo.
echo ========================================
echo 4. CLI Performance Tests
echo ========================================
echo.

cd %CLI_DIR%
echo [%time%] Running CLI performance tests...
cargo test performance_tests > ..\%TEST_RESULTS_DIR%\cli-performance-tests.log 2>&1
if %errorlevel% equ 0 (
    echo ✓ CLI performance tests PASSED
    echo CLI Performance Tests: PASSED >> ..\%TEST_RESULTS_DIR%\test-summary.txt
    set /a PASSED_TESTS+=1
) else (
    echo ✗ CLI performance tests FAILED
    echo CLI Performance Tests: FAILED >> ..\%TEST_RESULTS_DIR%\test-summary.txt
    set /a FAILED_TESTS+=1
)
set /a TOTAL_TESTS+=1
cd ..

echo.
echo ========================================
echo 5. CLI Error Scenario Tests
echo ========================================
echo.

cd %CLI_DIR%
echo [%time%] Running CLI error scenario tests...
cargo test test_error_scenarios > ..\%TEST_RESULTS_DIR%\cli-error-tests.log 2>&1
if %errorlevel% equ 0 (
    echo ✓ CLI error scenario tests PASSED
    echo CLI Error Scenario Tests: PASSED >> ..\%TEST_RESULTS_DIR%\test-summary.txt
    set /a PASSED_TESTS+=1
) else (
    echo ✗ CLI error scenario tests FAILED
    echo CLI Error Scenario Tests: FAILED >> ..\%TEST_RESULTS_DIR%\test-summary.txt
    set /a FAILED_TESTS+=1
)
set /a TOTAL_TESTS+=1
cd ..

echo.
echo ========================================
echo 6. CLI Build and Executable Tests
echo ========================================
echo.

cd %CLI_DIR%
echo [%time%] Building CLI release executable...
cargo build --release > ..\%TEST_RESULTS_DIR%\cli-build.log 2>&1
if %errorlevel% equ 0 (
    echo ✓ CLI build PASSED
    
    REM Test the built executable
    echo [%time%] Testing CLI executable...
    target\release\triageir-cli.exe --help > ..\%TEST_RESULTS_DIR%\cli-executable-test.log 2>&1
    if %errorlevel% equ 0 (
        echo ✓ CLI executable test PASSED
        echo CLI Build and Executable Tests: PASSED >> ..\%TEST_RESULTS_DIR%\test-summary.txt
        set /a PASSED_TESTS+=1
    ) else (
        echo ✗ CLI executable test FAILED
        echo CLI Build and Executable Tests: FAILED >> ..\%TEST_RESULTS_DIR%\test-summary.txt
        set /a FAILED_TESTS+=1
    )
) else (
    echo ✗ CLI build FAILED
    echo CLI Build and Executable Tests: FAILED >> ..\%TEST_RESULTS_DIR%\test-summary.txt
    set /a FAILED_TESTS+=1
)
set /a TOTAL_TESTS+=1
cd ..

echo.
echo ========================================
echo 7. GUI Dependencies and Setup
echo ========================================
echo.

cd %GUI_DIR%
echo [%time%] Installing GUI dependencies...
npm install > ..\%TEST_RESULTS_DIR%\gui-install.log 2>&1
if %errorlevel% equ 0 (
    echo ✓ GUI dependencies installation PASSED
    echo GUI Dependencies: PASSED >> ..\%TEST_RESULTS_DIR%\test-summary.txt
    set /a PASSED_TESTS+=1
) else (
    echo ✗ GUI dependencies installation FAILED
    echo GUI Dependencies: FAILED >> ..\%TEST_RESULTS_DIR%\test-summary.txt
    set /a FAILED_TESTS+=1
)
set /a TOTAL_TESTS+=1
cd ..

echo.
echo ========================================
echo 8. GUI Unit Tests
echo ========================================
echo.

cd %GUI_DIR%
echo [%time%] Running GUI unit tests...
npm test > ..\%TEST_RESULTS_DIR%\gui-unit-tests.log 2>&1
if %errorlevel% equ 0 (
    echo ✓ GUI unit tests PASSED
    echo GUI Unit Tests: PASSED >> ..\%TEST_RESULTS_DIR%\test-summary.txt
    set /a PASSED_TESTS+=1
) else (
    echo ✗ GUI unit tests FAILED
    echo GUI Unit Tests: FAILED >> ..\%TEST_RESULTS_DIR%\test-summary.txt
    set /a FAILED_TESTS+=1
)
set /a TOTAL_TESTS+=1
cd ..

echo.
echo ========================================
echo 9. End-to-End Integration Tests
echo ========================================
echo.

echo [%time%] Running end-to-end integration tests...
call test-scripts\run-e2e-tests.bat > %TEST_RESULTS_DIR%\e2e-tests.log 2>&1
if %errorlevel% equ 0 (
    echo ✓ End-to-end integration tests PASSED
    echo End-to-End Integration Tests: PASSED >> %TEST_RESULTS_DIR%\test-summary.txt
    set /a PASSED_TESTS+=1
) else (
    echo ✗ End-to-end integration tests FAILED
    echo End-to-End Integration Tests: FAILED >> %TEST_RESULTS_DIR%\test-summary.txt
    set /a FAILED_TESTS+=1
)
set /a TOTAL_TESTS+=1

echo.
echo ========================================
echo 10. Performance Benchmarking
echo ========================================
echo.

echo [%time%] Running performance benchmarks...
call test-scripts\run-performance-benchmarks.bat > %TEST_RESULTS_DIR%\performance-benchmarks.log 2>&1
if %errorlevel% equ 0 (
    echo ✓ Performance benchmarks PASSED
    echo Performance Benchmarks: PASSED >> %TEST_RESULTS_DIR%\test-summary.txt
    set /a PASSED_TESTS+=1
) else (
    echo ✗ Performance benchmarks FAILED
    echo Performance Benchmarks: FAILED >> %TEST_RESULTS_DIR%\test-summary.txt
    set /a FAILED_TESTS+=1
)
set /a TOTAL_TESTS+=1

echo.
echo ========================================
echo 11. Security and Forensic Validation
echo ========================================
echo.

echo [%time%] Running security and forensic validation...
call test-scripts\run-forensic-validation.bat > %TEST_RESULTS_DIR%\forensic-validation.log 2>&1
if %errorlevel% equ 0 (
    echo ✓ Security and forensic validation PASSED
    echo Security and Forensic Validation: PASSED >> %TEST_RESULTS_DIR%\test-summary.txt
    set /a PASSED_TESTS+=1
) else (
    echo ✗ Security and forensic validation FAILED
    echo Security and Forensic Validation: FAILED >> %TEST_RESULTS_DIR%\test-summary.txt
    set /a FAILED_TESTS+=1
)
set /a TOTAL_TESTS+=1

REM Calculate test results
set TEST_END_TIME=%time%
set /a SUCCESS_RATE=(%PASSED_TESTS% * 100) / %TOTAL_TESTS%

echo.
echo ========================================
echo TEST SUMMARY
echo ========================================
echo.
echo Test completed: %date% %TEST_END_TIME%
echo Total test suites: %TOTAL_TESTS%
echo Passed: %PASSED_TESTS%
echo Failed: %FAILED_TESTS%
echo Success rate: %SUCCESS_RATE%%%
echo.

REM Write final summary
echo. >> %TEST_RESULTS_DIR%\test-summary.txt
echo ======================================== >> %TEST_RESULTS_DIR%\test-summary.txt
echo FINAL SUMMARY >> %TEST_RESULTS_DIR%\test-summary.txt
echo ======================================== >> %TEST_RESULTS_DIR%\test-summary.txt
echo Test completed: %date% %TEST_END_TIME% >> %TEST_RESULTS_DIR%\test-summary.txt
echo Total test suites: %TOTAL_TESTS% >> %TEST_RESULTS_DIR%\test-summary.txt
echo Passed: %PASSED_TESTS% >> %TEST_RESULTS_DIR%\test-summary.txt
echo Failed: %FAILED_TESTS% >> %TEST_RESULTS_DIR%\test-summary.txt
echo Success rate: %SUCCESS_RATE%%% >> %TEST_RESULTS_DIR%\test-summary.txt

REM Generate detailed test report
echo [%time%] Generating detailed test report...
call test-scripts\generate-test-report.bat

if %FAILED_TESTS% gtr 0 (
    echo.
    echo ⚠ WARNING: %FAILED_TESTS% test suite(s) failed!
    echo Check the log files in %TEST_RESULTS_DIR% for details.
    echo.
    exit /b 1
) else (
    echo.
    echo ✓ All test suites passed successfully!
    echo Test results available in %TEST_RESULTS_DIR%
    echo.
    exit /b 0
)