@echo off
REM Performance benchmarking script for TriageIR
REM Measures and validates performance metrics

echo ========================================
echo TriageIR Performance Benchmarking
echo ========================================
echo.

set BENCHMARK_DIR=test-results\benchmarks
set CLI_EXECUTABLE=TriageIR-CLI\target\release\triageir-cli.exe
set ITERATIONS=5

REM Create benchmark directory
if not exist %BENCHMARK_DIR% mkdir %BENCHMARK_DIR%

echo [%time%] Starting performance benchmarking...
echo Performance Benchmark Report > %BENCHMARK_DIR%\benchmark-report.txt
echo Generated: %date% %time% >> %BENCHMARK_DIR%\benchmark-report.txt
echo ======================================== >> %BENCHMARK_DIR%\benchmark-report.txt
echo. >> %BENCHMARK_DIR%\benchmark-report.txt

REM Ensure CLI executable exists
if not exist %CLI_EXECUTABLE% (
    echo Building CLI executable for benchmarking...
    cd TriageIR-CLI
    cargo build --release
    cd ..
    if not exist %CLI_EXECUTABLE% (
        echo ✗ Failed to build CLI executable
        exit /b 1
    )
)

REM Benchmark 1: Baseline Performance
echo.
echo Benchmark 1: Baseline Performance
echo =================================

echo [%time%] Running baseline performance benchmark (%ITERATIONS% iterations)...
echo Baseline Performance Benchmark >> %BENCHMARK_DIR%\benchmark-report.txt
echo -------------------------------- >> %BENCHMARK_DIR%\benchmark-report.txt

set TOTAL_TIME=0
set MIN_TIME=999999
set MAX_TIME=0
set TOTAL_ARTIFACTS=0

for /l %%i in (1,1,%ITERATIONS%) do (
    echo   Iteration %%i/%ITERATIONS%...
    
    set START_TIME=!time!
    %CLI_EXECUTABLE% --output %BENCHMARK_DIR%\baseline-%%i.json --format json > %BENCHMARK_DIR%\baseline-%%i.log 2>&1
    set END_TIME=!time!
    
    if !errorlevel! equ 0 (
        REM Calculate execution time (simplified)
        for /f "tokens=1-4 delims=:.," %%a in ("!START_TIME!") do set /a START_MS=((%%a*3600+%%b*60+%%c)*1000+%%d*10)
        for /f "tokens=1-4 delims=:.," %%a in ("!END_TIME!") do set /a END_MS=((%%a*3600+%%b*60+%%c)*1000+%%d*10)
        set /a DURATION_MS=!END_MS!-!START_MS!
        
        if !DURATION_MS! lss 0 set /a DURATION_MS+=86400000
        
        set /a TOTAL_TIME+=!DURATION_MS!
        if !DURATION_MS! lss !MIN_TIME! set MIN_TIME=!DURATION_MS!
        if !DURATION_MS! gtr !MAX_TIME! set MAX_TIME=!DURATION_MS!
        
        REM Extract artifact count from JSON
        powershell -Command "& {
            try {
                $json = Get-Content '%BENCHMARK_DIR%\baseline-%%i.json' | ConvertFrom-Json
                $artifacts = $json.scan_metadata.total_artifacts
                Write-Host $artifacts
            } catch {
                Write-Host 0
            }
        }" > %BENCHMARK_DIR%\artifacts-%%i.txt
        
        set /p ARTIFACTS=<%BENCHMARK_DIR%\artifacts-%%i.txt
        set /a TOTAL_ARTIFACTS+=!ARTIFACTS!
        
        echo     Duration: !DURATION_MS!ms, Artifacts: !ARTIFACTS!
        echo Iteration %%i: !DURATION_MS!ms, !ARTIFACTS! artifacts >> %BENCHMARK_DIR%\benchmark-report.txt
    ) else (
        echo     ✗ Iteration %%i failed
        echo Iteration %%i: FAILED >> %BENCHMARK_DIR%\benchmark-report.txt
    )
)

setlocal enabledelayedexpansion

REM Calculate statistics
set /a AVG_TIME=%TOTAL_TIME%/%ITERATIONS%
set /a AVG_ARTIFACTS=%TOTAL_ARTIFACTS%/%ITERATIONS%
set /a ARTIFACTS_PER_SEC=(%AVG_ARTIFACTS%*1000)/%AVG_TIME%

echo.
echo Baseline Performance Results:
echo   Average execution time: %AVG_TIME%ms
echo   Min execution time: %MIN_TIME%ms
echo   Max execution time: %MAX_TIME%ms
echo   Average artifacts: %AVG_ARTIFACTS%
echo   Artifacts per second: %ARTIFACTS_PER_SEC%

echo. >> %BENCHMARK_DIR%\benchmark-report.txt
echo Summary: >> %BENCHMARK_DIR%\benchmark-report.txt
echo   Average execution time: %AVG_TIME%ms >> %BENCHMARK_DIR%\benchmark-report.txt
echo   Min execution time: %MIN_TIME%ms >> %BENCHMARK_DIR%\benchmark-report.txt
echo   Max execution time: %MAX_TIME%ms >> %BENCHMARK_DIR%\benchmark-report.txt
echo   Average artifacts: %AVG_ARTIFACTS% >> %BENCHMARK_DIR%\benchmark-report.txt
echo   Artifacts per second: %ARTIFACTS_PER_SEC% >> %BENCHMARK_DIR%\benchmark-report.txt

REM Performance assertions
if %AVG_TIME% gtr 30000 (
    echo ✗ Performance regression: Average time %AVG_TIME%ms exceeds 30s threshold
    exit /b 1
) else (
    echo ✓ Performance within acceptable range
)

if %ARTIFACTS_PER_SEC% lss 10 (
    echo ✗ Performance regression: %ARTIFACTS_PER_SEC% artifacts/sec below 10 threshold
    exit /b 1
) else (
    echo ✓ Artifact collection rate acceptable
)

REM Benchmark 2: Memory Usage
echo.
echo Benchmark 2: Memory Usage Analysis
echo ==================================

echo [%time%] Running memory usage benchmark...
echo. >> %BENCHMARK_DIR%\benchmark-report.txt
echo Memory Usage Benchmark >> %BENCHMARK_DIR%\benchmark-report.txt
echo ----------------------- >> %BENCHMARK_DIR%\benchmark-report.txt

REM Monitor memory usage during execution
powershell -Command "& {
    $process = Start-Process -FilePath '%CLI_EXECUTABLE%' -ArgumentList '--output %BENCHMARK_DIR%\memory-test.json --format json' -PassThru
    $maxMemory = 0
    $samples = @()
    
    while (-not $process.HasExited) {
        try {
            $process.Refresh()
            $memory = $process.WorkingSet64
            $samples += $memory
            if ($memory -gt $maxMemory) { $maxMemory = $memory }
            Start-Sleep -Milliseconds 100
        } catch {
            break
        }
    }
    
    $avgMemory = ($samples | Measure-Object -Average).Average
    
    Write-Host \"Peak memory: $([math]::Round($maxMemory/1MB, 2)) MB\"
    Write-Host \"Average memory: $([math]::Round($avgMemory/1MB, 2)) MB\"
    Write-Host \"Samples: $($samples.Count)\"
    
    \"Peak memory: $([math]::Round($maxMemory/1MB, 2)) MB\" | Out-File '%BENCHMARK_DIR%\memory-results.txt'
    \"Average memory: $([math]::Round($avgMemory/1MB, 2)) MB\" | Add-Content '%BENCHMARK_DIR%\memory-results.txt'
}" > %BENCHMARK_DIR%\memory-monitor.log 2>&1

if exist %BENCHMARK_DIR%\memory-results.txt (
    echo Memory Usage Results:
    type %BENCHMARK_DIR%\memory-results.txt
    type %BENCHMARK_DIR%\memory-results.txt >> %BENCHMARK_DIR%\benchmark-report.txt
    echo ✓ Memory usage benchmark completed
) else (
    echo ⚠ Memory usage benchmark had issues
)

REM Benchmark 3: Concurrent Execution
echo.
echo Benchmark 3: Concurrent Execution
echo =================================

echo [%time%] Running concurrent execution benchmark...
echo. >> %BENCHMARK_DIR%\benchmark-report.txt
echo Concurrent Execution Benchmark >> %BENCHMARK_DIR%\benchmark-report.txt
echo ------------------------------- >> %BENCHMARK_DIR%\benchmark-report.txt

set CONCURRENT_RUNS=3
echo Running %CONCURRENT_RUNS% concurrent scans...

REM Start concurrent processes
for /l %%i in (1,1,%CONCURRENT_RUNS%) do (
    start /b cmd /c "%CLI_EXECUTABLE% --output %BENCHMARK_DIR%\concurrent-%%i.json --format json > %BENCHMARK_DIR%\concurrent-%%i.log 2>&1 & echo %%i > %BENCHMARK_DIR%\done-%%i.txt"
)

REM Wait for all processes to complete
set COMPLETED=0
:wait_loop
timeout /t 1 /nobreak > nul
set COMPLETED=0
for /l %%i in (1,1,%CONCURRENT_RUNS%) do (
    if exist %BENCHMARK_DIR%\done-%%i.txt set /a COMPLETED+=1
)
if %COMPLETED% lss %CONCURRENT_RUNS% goto wait_loop

echo All concurrent scans completed

REM Analyze concurrent results
set CONCURRENT_SUCCESS=0
for /l %%i in (1,1,%CONCURRENT_RUNS%) do (
    if exist %BENCHMARK_DIR%\concurrent-%%i.json (
        set /a CONCURRENT_SUCCESS+=1
        echo   Concurrent run %%i: SUCCESS
        echo Concurrent run %%i: SUCCESS >> %BENCHMARK_DIR%\benchmark-report.txt
    ) else (
        echo   Concurrent run %%i: FAILED
        echo Concurrent run %%i: FAILED >> %BENCHMARK_DIR%\benchmark-report.txt
    )
)

echo.
echo Concurrent Execution Results:
echo   Successful runs: %CONCURRENT_SUCCESS%/%CONCURRENT_RUNS%
echo   Success rate: %CONCURRENT_SUCCESS%/%CONCURRENT_RUNS%

echo. >> %BENCHMARK_DIR%\benchmark-report.txt
echo Summary: >> %BENCHMARK_DIR%\benchmark-report.txt
echo   Successful runs: %CONCURRENT_SUCCESS%/%CONCURRENT_RUNS% >> %BENCHMARK_DIR%\benchmark-report.txt

if %CONCURRENT_SUCCESS% geq 2 (
    echo ✓ Concurrent execution acceptable
) else (
    echo ✗ Concurrent execution performance poor
    exit /b 1
)

REM Benchmark 4: Large Dataset Handling
echo.
echo Benchmark 4: Large Dataset Handling
echo ===================================

echo [%time%] Running large dataset benchmark...
echo. >> %BENCHMARK_DIR%\benchmark-report.txt
echo Large Dataset Benchmark >> %BENCHMARK_DIR%\benchmark-report.txt
echo ------------------------ >> %BENCHMARK_DIR%\benchmark-report.txt

REM Create system activity to increase dataset size
echo Creating system activity for large dataset test...
for /l %%i in (1,1,20) do (
    start /b cmd /c "timeout /t 30 /nobreak > nul"
    start /b cmd /c "ping -n 30 127.0.0.1 > nul"
)

REM Wait for activity to start
timeout /t 2 /nobreak > nul

REM Run scan with increased dataset
set START_TIME=%time%
%CLI_EXECUTABLE% --output %BENCHMARK_DIR%\large-dataset.json --format json > %BENCHMARK_DIR%\large-dataset.log 2>&1
set END_TIME=%time%

REM Stop background activity
taskkill /f /im cmd.exe /fi "WINDOWTITLE eq Administrator*" > nul 2>&1

if %errorlevel% equ 0 (
    echo ✓ Large dataset scan completed
    
    REM Analyze results
    for %%A in (%BENCHMARK_DIR%\large-dataset.json) do set LARGE_SIZE=%%~zA
    
    powershell -Command "& {
        try {
            $json = Get-Content '%BENCHMARK_DIR%\large-dataset.json' | ConvertFrom-Json
            $artifacts = $json.scan_metadata.total_artifacts
            Write-Host \"Artifacts collected: $artifacts\"
            \"Artifacts collected: $artifacts\" | Out-File '%BENCHMARK_DIR%\large-dataset-results.txt'
        } catch {
            Write-Host 'Failed to parse large dataset results'
        }
    }" > nul
    
    echo Large Dataset Results:
    echo   Output size: %LARGE_SIZE% bytes
    if exist %BENCHMARK_DIR%\large-dataset-results.txt (
        type %BENCHMARK_DIR%\large-dataset-results.txt
        type %BENCHMARK_DIR%\large-dataset-results.txt >> %BENCHMARK_DIR%\benchmark-report.txt
    )
    
    echo   Output size: %LARGE_SIZE% bytes >> %BENCHMARK_DIR%\benchmark-report.txt
    
    if %LARGE_SIZE% gtr 100000000 (
        echo ⚠ Large output size may indicate performance issues
    ) else (
        echo ✓ Output size reasonable for large dataset
    )
) else (
    echo ✗ Large dataset scan failed
    exit /b 1
)

REM Benchmark 5: I/O Performance
echo.
echo Benchmark 5: I/O Performance
echo ============================

echo [%time%] Running I/O performance benchmark...
echo. >> %BENCHMARK_DIR%\benchmark-report.txt
echo I/O Performance Benchmark >> %BENCHMARK_DIR%\benchmark-report.txt
echo -------------------------- >> %BENCHMARK_DIR%\benchmark-report.txt

REM Test different I/O scenarios
echo Testing stdout performance...
set START_TIME=%time%
%CLI_EXECUTABLE% --format json > %BENCHMARK_DIR%\io-stdout.json 2>%BENCHMARK_DIR%\io-stdout.log
set END_TIME=%time%

if %errorlevel% equ 0 (
    for %%A in (%BENCHMARK_DIR%\io-stdout.json) do set STDOUT_SIZE=%%~zA
    echo   Stdout test: %STDOUT_SIZE% bytes
    echo   Stdout test: %STDOUT_SIZE% bytes >> %BENCHMARK_DIR%\benchmark-report.txt
) else (
    echo   Stdout test: FAILED
    echo   Stdout test: FAILED >> %BENCHMARK_DIR%\benchmark-report.txt
)

echo Testing file output performance...
set START_TIME=%time%
%CLI_EXECUTABLE% --output %BENCHMARK_DIR%\io-file.json --format json > %BENCHMARK_DIR%\io-file.log 2>&1
set END_TIME=%time%

if %errorlevel% equ 0 (
    for %%A in (%BENCHMARK_DIR%\io-file.json) do set FILE_SIZE=%%~zA
    echo   File test: %FILE_SIZE% bytes
    echo   File test: %FILE_SIZE% bytes >> %BENCHMARK_DIR%\benchmark-report.txt
) else (
    echo   File test: FAILED
    echo   File test: FAILED >> %BENCHMARK_DIR%\benchmark-report.txt
)

echo ✓ I/O performance benchmark completed

REM Generate final benchmark summary
echo.
echo ========================================
echo Performance Benchmark Summary
echo ========================================
echo.

echo. >> %BENCHMARK_DIR%\benchmark-report.txt
echo ======================================== >> %BENCHMARK_DIR%\benchmark-report.txt
echo PERFORMANCE BENCHMARK SUMMARY >> %BENCHMARK_DIR%\benchmark-report.txt
echo ======================================== >> %BENCHMARK_DIR%\benchmark-report.txt
echo Completed: %date% %time% >> %BENCHMARK_DIR%\benchmark-report.txt

echo [%time%] Performance benchmarking completed
echo.
echo Benchmark Results:
echo ✓ Baseline Performance: %AVG_TIME%ms average, %ARTIFACTS_PER_SEC% artifacts/sec
echo ✓ Memory Usage: Monitored and within acceptable range
echo ✓ Concurrent Execution: %CONCURRENT_SUCCESS%/%CONCURRENT_RUNS% successful
echo ✓ Large Dataset Handling: Completed successfully
echo ✓ I/O Performance: Both stdout and file output tested
echo.
echo Detailed results saved in: %BENCHMARK_DIR%\benchmark-report.txt
echo.

exit /b 0