@echo off
REM Test report generator for TriageIR
REM Generates comprehensive HTML test report from test results

echo ========================================
echo TriageIR Test Report Generator
echo ========================================
echo.

set RESULTS_DIR=test-results
set REPORT_FILE=%RESULTS_DIR%\comprehensive-test-report.html

echo [%time%] Generating comprehensive test report...

REM Create HTML report
echo ^<!DOCTYPE html^> > %REPORT_FILE%
echo ^<html lang="en"^> >> %REPORT_FILE%
echo ^<head^> >> %REPORT_FILE%
echo     ^<meta charset="UTF-8"^> >> %REPORT_FILE%
echo     ^<meta name="viewport" content="width=device-width, initial-scale=1.0"^> >> %REPORT_FILE%
echo     ^<title^>TriageIR Comprehensive Test Report^</title^> >> %REPORT_FILE%
echo     ^<style^> >> %REPORT_FILE%
echo         body { font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; margin: 0; padding: 20px; background-color: #f5f5f5; } >> %REPORT_FILE%
echo         .container { max-width: 1200px; margin: 0 auto; background-color: white; padding: 30px; border-radius: 8px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); } >> %REPORT_FILE%
echo         h1 { color: #2c3e50; border-bottom: 3px solid #3498db; padding-bottom: 10px; } >> %REPORT_FILE%
echo         h2 { color: #34495e; margin-top: 30px; } >> %REPORT_FILE%
echo         h3 { color: #7f8c8d; } >> %REPORT_FILE%
echo         .summary { background-color: #ecf0f1; padding: 20px; border-radius: 5px; margin: 20px 0; } >> %REPORT_FILE%
echo         .passed { color: #27ae60; font-weight: bold; } >> %REPORT_FILE%
echo         .failed { color: #e74c3c; font-weight: bold; } >> %REPORT_FILE%
echo         .warning { color: #f39c12; font-weight: bold; } >> %REPORT_FILE%
echo         .test-section { margin: 20px 0; padding: 15px; border-left: 4px solid #3498db; background-color: #f8f9fa; } >> %REPORT_FILE%
echo         .log-content { background-color: #2c3e50; color: #ecf0f1; padding: 15px; border-radius: 5px; font-family: 'Courier New', monospace; font-size: 12px; overflow-x: auto; max-height: 300px; overflow-y: auto; } >> %REPORT_FILE%
echo         .metrics { display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 15px; margin: 20px 0; } >> %REPORT_FILE%
echo         .metric-card { background-color: #ffffff; border: 1px solid #bdc3c7; border-radius: 5px; padding: 15px; text-align: center; } >> %REPORT_FILE%
echo         .metric-value { font-size: 24px; font-weight: bold; color: #2c3e50; } >> %REPORT_FILE%
echo         .metric-label { color: #7f8c8d; font-size: 14px; } >> %REPORT_FILE%
echo         .timestamp { color: #95a5a6; font-size: 12px; } >> %REPORT_FILE%
echo         table { width: 100%%; border-collapse: collapse; margin: 15px 0; } >> %REPORT_FILE%
echo         th, td { border: 1px solid #bdc3c7; padding: 8px; text-align: left; } >> %REPORT_FILE%
echo         th { background-color: #34495e; color: white; } >> %REPORT_FILE%
echo         tr:nth-child(even) { background-color: #f2f2f2; } >> %REPORT_FILE%
echo         .collapsible { background-color: #34495e; color: white; cursor: pointer; padding: 10px; width: 100%%; border: none; text-align: left; outline: none; font-size: 15px; } >> %REPORT_FILE%
echo         .collapsible:hover { background-color: #2c3e50; } >> %REPORT_FILE%
echo         .content { padding: 0 15px; display: none; overflow: hidden; background-color: #f1f1f1; } >> %REPORT_FILE%
echo     ^</style^> >> %REPORT_FILE%
echo ^</head^> >> %REPORT_FILE%
echo ^<body^> >> %REPORT_FILE%
echo     ^<div class="container"^> >> %REPORT_FILE%
echo         ^<h1^>TriageIR Comprehensive Test Report^</h1^> >> %REPORT_FILE%
echo         ^<p class="timestamp"^>Generated: %date% %time%^</p^> >> %REPORT_FILE%

REM Add executive summary
echo         ^<div class="summary"^> >> %REPORT_FILE%
echo             ^<h2^>Executive Summary^</h2^> >> %REPORT_FILE%

REM Parse test summary if available
if exist %RESULTS_DIR%\test-summary.txt (
    echo             ^<p^>This report provides a comprehensive overview of all testing performed on the TriageIR forensic framework.^</p^> >> %REPORT_FILE%
    
    REM Extract summary statistics
    for /f "tokens=3" %%a in ('findstr "Total test suites:" %RESULTS_DIR%\test-summary.txt') do set TOTAL_SUITES=%%a
    for /f "tokens=2" %%a in ('findstr "Passed:" %RESULTS_DIR%\test-summary.txt') do set PASSED_SUITES=%%a
    for /f "tokens=2" %%a in ('findstr "Failed:" %RESULTS_DIR%\test-summary.txt') do set FAILED_SUITES=%%a
    for /f "tokens=3" %%a in ('findstr "Success rate:" %RESULTS_DIR%\test-summary.txt') do set SUCCESS_RATE=%%a
    
    echo             ^<div class="metrics"^> >> %REPORT_FILE%
    echo                 ^<div class="metric-card"^> >> %REPORT_FILE%
    echo                     ^<div class="metric-value"^>%TOTAL_SUITES%^</div^> >> %REPORT_FILE%
    echo                     ^<div class="metric-label"^>Total Test Suites^</div^> >> %REPORT_FILE%
    echo                 ^</div^> >> %REPORT_FILE%
    echo                 ^<div class="metric-card"^> >> %REPORT_FILE%
    echo                     ^<div class="metric-value passed"^>%PASSED_SUITES%^</div^> >> %REPORT_FILE%
    echo                     ^<div class="metric-label"^>Passed^</div^> >> %REPORT_FILE%
    echo                 ^</div^> >> %REPORT_FILE%
    echo                 ^<div class="metric-card"^> >> %REPORT_FILE%
    echo                     ^<div class="metric-value failed"^>%FAILED_SUITES%^</div^> >> %REPORT_FILE%
    echo                     ^<div class="metric-label"^>Failed^</div^> >> %REPORT_FILE%
    echo                 ^</div^> >> %REPORT_FILE%
    echo                 ^<div class="metric-card"^> >> %REPORT_FILE%
    echo                     ^<div class="metric-value"^>%SUCCESS_RATE%^</div^> >> %REPORT_FILE%
    echo                     ^<div class="metric-label"^>Success Rate^</div^> >> %REPORT_FILE%
    echo                 ^</div^> >> %REPORT_FILE%
    echo             ^</div^> >> %REPORT_FILE%
) else (
    echo             ^<p^>Test summary not available.^</p^> >> %REPORT_FILE%
)

echo         ^</div^> >> %REPORT_FILE%

REM Add detailed test results sections
echo         ^<h2^>Detailed Test Results^</h2^> >> %REPORT_FILE%

REM CLI Unit Tests
if exist %RESULTS_DIR%\cli-unit-tests.log (
    echo         ^<div class="test-section"^> >> %REPORT_FILE%
    echo             ^<h3^>CLI Unit Tests^</h3^> >> %REPORT_FILE%
    findstr /c:"test result:" %RESULTS_DIR%\cli-unit-tests.log > temp_result.txt 2>nul
    if %errorlevel% equ 0 (
        for /f "tokens=*" %%a in (temp_result.txt) do (
            echo             ^<p^>%%a^</p^> >> %REPORT_FILE%
        )
        del temp_result.txt
    )
    echo             ^<button class="collapsible"^>View Full Log^</button^> >> %REPORT_FILE%
    echo             ^<div class="content"^> >> %REPORT_FILE%
    echo                 ^<div class="log-content"^> >> %REPORT_FILE%
    powershell -Command "Get-Content '%RESULTS_DIR%\cli-unit-tests.log' | ForEach-Object { $_ -replace '<', '&lt;' -replace '>', '&gt;' } | ForEach-Object { '                    ' + $_ + '<br>' }" >> %REPORT_FILE%
    echo                 ^</div^> >> %REPORT_FILE%
    echo             ^</div^> >> %REPORT_FILE%
    echo         ^</div^> >> %REPORT_FILE%
)

REM CLI Integration Tests
if exist %RESULTS_DIR%\cli-integration-tests.log (
    echo         ^<div class="test-section"^> >> %REPORT_FILE%
    echo             ^<h3^>CLI Integration Tests^</h3^> >> %REPORT_FILE%
    findstr /c:"test result:" %RESULTS_DIR%\cli-integration-tests.log > temp_result.txt 2>nul
    if %errorlevel% equ 0 (
        for /f "tokens=*" %%a in (temp_result.txt) do (
            echo             ^<p^>%%a^</p^> >> %REPORT_FILE%
        )
        del temp_result.txt
    )
    echo             ^<button class="collapsible"^>View Full Log^</button^> >> %REPORT_FILE%
    echo             ^<div class="content"^> >> %REPORT_FILE%
    echo                 ^<div class="log-content"^> >> %REPORT_FILE%
    powershell -Command "Get-Content '%RESULTS_DIR%\cli-integration-tests.log' | ForEach-Object { $_ -replace '<', '&lt;' -replace '>', '&gt;' } | ForEach-Object { '                    ' + $_ + '<br>' }" >> %REPORT_FILE%
    echo                 ^</div^> >> %REPORT_FILE%
    echo             ^</div^> >> %REPORT_FILE%
    echo         ^</div^> >> %REPORT_FILE%
)

REM Performance Benchmarks
if exist %RESULTS_DIR%\benchmarks\benchmark-report.txt (
    echo         ^<div class="test-section"^> >> %REPORT_FILE%
    echo             ^<h3^>Performance Benchmarks^</h3^> >> %REPORT_FILE%
    echo             ^<p^>Performance benchmarking completed. Key metrics:^</p^> >> %REPORT_FILE%
    echo             ^<button class="collapsible"^>View Benchmark Report^</button^> >> %REPORT_FILE%
    echo             ^<div class="content"^> >> %REPORT_FILE%
    echo                 ^<div class="log-content"^> >> %REPORT_FILE%
    powershell -Command "Get-Content '%RESULTS_DIR%\benchmarks\benchmark-report.txt' | ForEach-Object { $_ -replace '<', '&lt;' -replace '>', '&gt;' } | ForEach-Object { '                    ' + $_ + '<br>' }" >> %REPORT_FILE%
    echo                 ^</div^> >> %REPORT_FILE%
    echo             ^</div^> >> %REPORT_FILE%
    echo         ^</div^> >> %REPORT_FILE%
)

REM Forensic Validation
if exist %RESULTS_DIR%\forensic-validation\forensic-validation-report.txt (
    echo         ^<div class="test-section"^> >> %REPORT_FILE%
    echo             ^<h3^>Forensic Validation^</h3^> >> %REPORT_FILE%
    echo             ^<p^>Forensic soundness and data integrity validation completed.^</p^> >> %REPORT_FILE%
    echo             ^<button class="collapsible"^>View Forensic Validation Report^</button^> >> %REPORT_FILE%
    echo             ^<div class="content"^> >> %REPORT_FILE%
    echo                 ^<div class="log-content"^> >> %REPORT_FILE%
    powershell -Command "Get-Content '%RESULTS_DIR%\forensic-validation\forensic-validation-report.txt' | ForEach-Object { $_ -replace '<', '&lt;' -replace '>', '&gt;' } | ForEach-Object { '                    ' + $_ + '<br>' }" >> %REPORT_FILE%
    echo                 ^</div^> >> %REPORT_FILE%
    echo             ^</div^> >> %REPORT_FILE%
    echo         ^</div^> >> %REPORT_FILE%
)

REM End-to-End Tests
if exist %RESULTS_DIR%\e2e (
    echo         ^<div class="test-section"^> >> %REPORT_FILE%
    echo             ^<h3^>End-to-End Integration Tests^</h3^> >> %REPORT_FILE%
    echo             ^<p^>Complete workflow testing from GUI to CLI integration.^</p^> >> %REPORT_FILE%
    
    if exist %RESULTS_DIR%\e2e\full-scan.json (
        for %%A in (%RESULTS_DIR%\e2e\full-scan.json) do set E2E_SIZE=%%~zA
        echo             ^<p^>Sample scan output size: %E2E_SIZE% bytes^</p^> >> %REPORT_FILE%
    )
    
    echo             ^<button class="collapsible"^>View E2E Test Details^</button^> >> %REPORT_FILE%
    echo             ^<div class="content"^> >> %REPORT_FILE%
    echo                 ^<p^>End-to-end test artifacts available in test-results/e2e/ directory.^</p^> >> %REPORT_FILE%
    echo             ^</div^> >> %REPORT_FILE%
    echo         ^</div^> >> %REPORT_FILE%
)

REM Test Environment Information
echo         ^<h2^>Test Environment^</h2^> >> %REPORT_FILE%
echo         ^<div class="test-section"^> >> %REPORT_FILE%
echo             ^<table^> >> %REPORT_FILE%
echo                 ^<tr^>^<th^>Property^</th^>^<th^>Value^</th^>^</tr^> >> %REPORT_FILE%
echo                 ^<tr^>^<td^>Operating System^</td^>^<td^>%OS%^</td^>^</tr^> >> %REPORT_FILE%
echo                 ^<tr^>^<td^>Computer Name^</td^>^<td^>%COMPUTERNAME%^</td^>^</tr^> >> %REPORT_FILE%
echo                 ^<tr^>^<td^>User Name^</td^>^<td^>%USERNAME%^</td^>^</tr^> >> %REPORT_FILE%
echo                 ^<tr^>^<td^>Test Date^</td^>^<td^>%date%^</td^>^</tr^> >> %REPORT_FILE%
echo                 ^<tr^>^<td^>Test Time^</td^>^<td^>%time%^</td^>^</tr^> >> %REPORT_FILE%
echo             ^</table^> >> %REPORT_FILE%
echo         ^</div^> >> %REPORT_FILE%

REM Recommendations and Next Steps
echo         ^<h2^>Recommendations and Next Steps^</h2^> >> %REPORT_FILE%
echo         ^<div class="test-section"^> >> %REPORT_FILE%

if "%FAILED_SUITES%" gtr "0" (
    echo             ^<h3 class="failed"^>Issues Identified^</h3^> >> %REPORT_FILE%
    echo             ^<p^>%FAILED_SUITES% test suite(s) failed. Please review the detailed logs above to identify and resolve issues.^</p^> >> %REPORT_FILE%
    echo             ^<ul^> >> %REPORT_FILE%
    echo                 ^<li^>Review failed test logs for specific error messages^</li^> >> %REPORT_FILE%
    echo                 ^<li^>Check system requirements and dependencies^</li^> >> %REPORT_FILE%
    echo                 ^<li^>Verify proper build configuration^</li^> >> %REPORT_FILE%
    echo                 ^<li^>Re-run tests after addressing issues^</li^> >> %REPORT_FILE%
    echo             ^</ul^> >> %REPORT_FILE%
) else (
    echo             ^<h3 class="passed"^>All Tests Passed^</h3^> >> %REPORT_FILE%
    echo             ^<p^>Congratulations! All test suites have passed successfully.^</p^> >> %REPORT_FILE%
    echo             ^<ul^> >> %REPORT_FILE%
    echo                 ^<li^>The TriageIR framework is ready for deployment^</li^> >> %REPORT_FILE%
    echo                 ^<li^>Consider running tests regularly during development^</li^> >> %REPORT_FILE%
    echo                 ^<li^>Monitor performance metrics over time^</li^> >> %REPORT_FILE%
    echo                 ^<li^>Update test cases as new features are added^</li^> >> %REPORT_FILE%
    echo             ^</ul^> >> %REPORT_FILE%
)

echo         ^</div^> >> %REPORT_FILE%

REM Add JavaScript for collapsible sections
echo         ^<script^> >> %REPORT_FILE%
echo             var coll = document.getElementsByClassName("collapsible"); >> %REPORT_FILE%
echo             var i; >> %REPORT_FILE%
echo             for (i = 0; i ^< coll.length; i++) { >> %REPORT_FILE%
echo                 coll[i].addEventListener("click", function() { >> %REPORT_FILE%
echo                     this.classList.toggle("active"); >> %REPORT_FILE%
echo                     var content = this.nextElementSibling; >> %REPORT_FILE%
echo                     if (content.style.display === "block") { >> %REPORT_FILE%
echo                         content.style.display = "none"; >> %REPORT_FILE%
echo                     } else { >> %REPORT_FILE%
echo                         content.style.display = "block"; >> %REPORT_FILE%
echo                     } >> %REPORT_FILE%
echo                 }); >> %REPORT_FILE%
echo             } >> %REPORT_FILE%
echo         ^</script^> >> %REPORT_FILE%

echo     ^</div^> >> %REPORT_FILE%
echo ^</body^> >> %REPORT_FILE%
echo ^</html^> >> %REPORT_FILE%

echo ✓ Comprehensive test report generated: %REPORT_FILE%
echo.
echo The report includes:
echo   - Executive summary with key metrics
echo   - Detailed test results for all suites
echo   - Performance benchmark data
echo   - Forensic validation results
echo   - Test environment information
echo   - Recommendations and next steps
echo.
echo Open %REPORT_FILE% in a web browser to view the full report.

exit /b 0