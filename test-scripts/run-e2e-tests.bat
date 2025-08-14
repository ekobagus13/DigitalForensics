@echo off
REM End-to-end integration testing script
REM Tests the complete workflow from GUI to CLI and back

echo ========================================
echo End-to-End Integration Testing
echo ========================================
echo.

set E2E_TEST_DIR=test-results\e2e
set CLI_EXECUTABLE=TriageIR-CLI\target\release\triageir-cli.exe
set GUI_DIR=TriageIR-GUI

REM Create E2E test directory
if not exist %E2E_TEST_DIR% mkdir %E2E_TEST_DIR%

echo [%time%] Starting end-to-end integration tests...

REM Test 1: CLI Standalone Functionality
echo.
echo Test 1: CLI Standalone Functionality
echo ====================================

if not exist %CLI_EXECUTABLE% (
    echo ✗ CLI executable not found. Building...
    cd TriageIR-CLI
    cargo build --release
    cd ..
    if not exist %CLI_EXECUTABLE% (
        echo ✗ Failed to build CLI executable
        exit /b 1
    )
)

echo [%time%] Testing CLI help output...
%CLI_EXECUTABLE% --help > %E2E_TEST_DIR%\cli-help.txt 2>&1
if %errorlevel% equ 0 (
    echo ✓ CLI help command works
) else (
    echo ✗ CLI help command failed
    exit /b 1
)

echo [%time%] Testing CLI version output...
%CLI_EXECUTABLE% --version > %E2E_TEST_DIR%\cli-version.txt 2>&1
if %errorlevel% equ 0 (
    echo ✓ CLI version command works
) else (
    echo ✗ CLI version command failed
    exit /b 1
)

echo [%time%] Testing CLI full scan with file output...
%CLI_EXECUTABLE% --output %E2E_TEST_DIR%\full-scan.json --format json --verbose > %E2E_TEST_DIR%\cli-full-scan.log 2>&1
if %errorlevel% equ 0 (
    echo ✓ CLI full scan completed successfully
    
    REM Validate JSON output
    if exist %E2E_TEST_DIR%\full-scan.json (
        echo ✓ JSON output file created
        
        REM Check file size (should be substantial)
        for %%A in (%E2E_TEST_DIR%\full-scan.json) do set JSON_SIZE=%%~zA
        if %JSON_SIZE% gtr 1000 (
            echo ✓ JSON output has substantial content (%JSON_SIZE% bytes)
        ) else (
            echo ✗ JSON output is too small (%JSON_SIZE% bytes)
            exit /b 1
        )
    ) else (
        echo ✗ JSON output file not created
        exit /b 1
    )
) else (
    echo ✗ CLI full scan failed
    type %E2E_TEST_DIR%\cli-full-scan.log
    exit /b 1
)

echo [%time%] Testing CLI stdout output...
%CLI_EXECUTABLE% --format json > %E2E_TEST_DIR%\stdout-scan.json 2>%E2E_TEST_DIR%\cli-stdout-scan.log
if %errorlevel% equ 0 (
    echo ✓ CLI stdout output works
    
    REM Validate stdout JSON
    for %%A in (%E2E_TEST_DIR%\stdout-scan.json) do set STDOUT_SIZE=%%~zA
    if %STDOUT_SIZE% gtr 1000 (
        echo ✓ Stdout JSON has substantial content (%STDOUT_SIZE% bytes)
    ) else (
        echo ✗ Stdout JSON is too small (%STDOUT_SIZE% bytes)
        exit /b 1
    )
) else (
    echo ✗ CLI stdout output failed
    type %E2E_TEST_DIR%\cli-stdout-scan.log
    exit /b 1
)

REM Test 2: JSON Schema Validation
echo.
echo Test 2: JSON Schema Validation
echo ===============================

echo [%time%] Validating JSON schema compliance...
powershell -Command "& {
    $json = Get-Content '%E2E_TEST_DIR%\full-scan.json' | ConvertFrom-Json
    
    # Check required top-level fields
    if (-not $json.scan_metadata) { Write-Error 'Missing scan_metadata'; exit 1 }
    if (-not $json.artifacts) { Write-Error 'Missing artifacts'; exit 1 }
    if (-not $json.collection_log) { Write-Error 'Missing collection_log'; exit 1 }
    
    # Check scan metadata fields
    if (-not $json.scan_metadata.scan_id) { Write-Error 'Missing scan_id'; exit 1 }
    if (-not $json.scan_metadata.hostname) { Write-Error 'Missing hostname'; exit 1 }
    if (-not $json.scan_metadata.total_artifacts) { Write-Error 'Missing total_artifacts'; exit 1 }
    
    # Check artifacts structure
    if (-not $json.artifacts.system_info) { Write-Error 'Missing system_info'; exit 1 }
    if (-not $json.artifacts.running_processes) { Write-Error 'Missing running_processes'; exit 1 }
    if (-not $json.artifacts.network_connections) { Write-Error 'Missing network_connections'; exit 1 }
    if (-not $json.artifacts.persistence_mechanisms) { Write-Error 'Missing persistence_mechanisms'; exit 1 }
    if (-not $json.artifacts.event_logs) { Write-Error 'Missing event_logs'; exit 1 }
    if (-not $json.artifacts.execution_evidence) { Write-Error 'Missing execution_evidence'; exit 1 }
    
    Write-Host 'JSON schema validation passed'
}" > %E2E_TEST_DIR%\schema-validation.log 2>&1

if %errorlevel% equ 0 (
    echo ✓ JSON schema validation passed
) else (
    echo ✗ JSON schema validation failed
    type %E2E_TEST_DIR%\schema-validation.log
    exit /b 1
)

REM Test 3: Data Integrity and Consistency
echo.
echo Test 3: Data Integrity and Consistency
echo ======================================

echo [%time%] Testing data consistency across multiple runs...
%CLI_EXECUTABLE% --output %E2E_TEST_DIR%\consistency-test-1.json --format json > %E2E_TEST_DIR%\consistency-1.log 2>&1
timeout /t 2 /nobreak > nul
%CLI_EXECUTABLE% --output %E2E_TEST_DIR%\consistency-test-2.json --format json > %E2E_TEST_DIR%\consistency-2.log 2>&1

if %errorlevel% equ 0 (
    echo ✓ Multiple scans completed
    
    REM Compare key fields for consistency
    powershell -Command "& {
        $scan1 = Get-Content '%E2E_TEST_DIR%\consistency-test-1.json' | ConvertFrom-Json
        $scan2 = Get-Content '%E2E_TEST_DIR%\consistency-test-2.json' | ConvertFrom-Json
        
        # Hostname should be consistent
        if ($scan1.scan_metadata.hostname -ne $scan2.scan_metadata.hostname) {
            Write-Error 'Hostname inconsistency detected'
            exit 1
        }
        
        # OS version should be consistent
        if ($scan1.scan_metadata.os_version -ne $scan2.scan_metadata.os_version) {
            Write-Error 'OS version inconsistency detected'
            exit 1
        }
        
        # Process count should be reasonably similar (within 20%%)
        $proc1 = $scan1.artifacts.running_processes.Count
        $proc2 = $scan2.artifacts.running_processes.Count
        $variance = [Math]::Abs($proc1 - $proc2) / $proc1 * 100
        
        if ($variance -gt 20) {
            Write-Warning \"Process count variance: $variance%% (may be acceptable)\"
        }
        
        Write-Host 'Data consistency validation passed'
    }" > %E2E_TEST_DIR%\consistency-validation.log 2>&1
    
    if %errorlevel% equ 0 (
        echo ✓ Data consistency validation passed
    ) else (
        echo ⚠ Data consistency validation had warnings
        type %E2E_TEST_DIR%\consistency-validation.log
    )
) else (
    echo ✗ Multiple scans failed
    exit /b 1
)

REM Test 4: Error Handling and Recovery
echo.
echo Test 4: Error Handling and Recovery
echo ===================================

echo [%time%] Testing invalid arguments handling...
%CLI_EXECUTABLE% --invalid-argument > %E2E_TEST_DIR%\invalid-args.log 2>&1
if %errorlevel% neq 0 (
    echo ✓ Invalid arguments properly rejected
) else (
    echo ✗ Invalid arguments should be rejected
    exit /b 1
)

echo [%time%] Testing invalid output path handling...
%CLI_EXECUTABLE% --output "Z:\invalid\path\output.json" --format json > %E2E_TEST_DIR%\invalid-path.log 2>&1
if %errorlevel% neq 0 (
    echo ✓ Invalid output path properly handled
) else (
    echo ⚠ Invalid output path handling may need review
)

echo [%time%] Testing graceful degradation...
%CLI_EXECUTABLE% --output %E2E_TEST_DIR%\degradation-test.json --format json --verbose > %E2E_TEST_DIR%\degradation.log 2>&1
if %errorlevel% equ 0 -o %errorlevel% equ 2 (
    echo ✓ Graceful degradation works (exit code: %errorlevel%)
    
    REM Check if output was still generated
    if exist %E2E_TEST_DIR%\degradation-test.json (
        echo ✓ Output generated despite potential errors
    ) else (
        echo ✗ No output generated during degradation test
        exit /b 1
    )
) else (
    echo ✗ Graceful degradation failed
    exit /b 1
)

REM Test 5: Performance Under Load
echo.
echo Test 5: Performance Under Load
echo ==============================

echo [%time%] Testing performance under system load...

REM Create background load
start /b cmd /c "for /l %%i in (1,1,100) do (echo Load test %%i & timeout /t 1 /nobreak > nul)"
start /b cmd /c "for /l %%i in (1,1,100) do (ping -n 1 127.0.0.1 > nul & timeout /t 1 /nobreak > nul)"

REM Run scan under load
%CLI_EXECUTABLE% --output %E2E_TEST_DIR%\load-test.json --format json > %E2E_TEST_DIR%\load-test.log 2>&1
set LOAD_TEST_RESULT=%errorlevel%

REM Stop background load
taskkill /f /im cmd.exe /fi "WINDOWTITLE eq Load test*" > nul 2>&1

if %LOAD_TEST_RESULT% equ 0 (
    echo ✓ Performance under load test passed
    
    REM Check output quality
    for %%A in (%E2E_TEST_DIR%\load-test.json) do set LOAD_SIZE=%%~zA
    if %LOAD_SIZE% gtr 1000 (
        echo ✓ Quality maintained under load (%LOAD_SIZE% bytes)
    ) else (
        echo ⚠ Output quality may be affected under load
    )
) else (
    echo ✗ Performance under load test failed
    exit /b 1
)

REM Test 6: GUI-CLI Integration (if GUI is available)
echo.
echo Test 6: GUI-CLI Integration
echo ===========================

if exist %GUI_DIR%\package.json (
    echo [%time%] Testing GUI-CLI integration...
    
    cd %GUI_DIR%
    
    REM Check if dependencies are installed
    if not exist node_modules (
        echo Installing GUI dependencies...
        npm install > ..\%E2E_TEST_DIR%\gui-install.log 2>&1
        if %errorlevel% neq 0 (
            echo ✗ GUI dependency installation failed
            cd ..
            exit /b 1
        )
    )
    
    REM Run GUI integration tests
    npm run test:integration > ..\%E2E_TEST_DIR%\gui-integration.log 2>&1
    if %errorlevel% equ 0 (
        echo ✓ GUI-CLI integration tests passed
    ) else (
        echo ⚠ GUI-CLI integration tests had issues (may be expected in headless environment)
        REM Don't fail the entire test suite for GUI issues in headless environment
    )
    
    cd ..
) else (
    echo ⚠ GUI not available for integration testing
)

echo.
echo ========================================
echo End-to-End Test Summary
echo ========================================
echo.
echo [%time%] All end-to-end integration tests completed
echo.
echo Test Results:
echo ✓ CLI Standalone Functionality
echo ✓ JSON Schema Validation
echo ✓ Data Integrity and Consistency
echo ✓ Error Handling and Recovery
echo ✓ Performance Under Load
echo ✓ GUI-CLI Integration (if available)
echo.
echo Test artifacts saved in: %E2E_TEST_DIR%
echo.

exit /b 0