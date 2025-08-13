@echo off
echo TriageIR CLI Sample Scan
echo ========================

REM Check if executable exists
if not exist "target\release\triageir-cli.exe" (
    echo.
    echo ERROR: Release executable not found!
    echo Please run 'build.bat' first to build the CLI.
    echo.
    pause
    exit /b 1
)

echo.
echo This will run a sample forensic scan of your system.
echo The scan will collect system information, processes, network connections,
echo persistence mechanisms, and event logs.
echo.
echo WARNING: This scan requires Administrator privileges for complete data collection.
echo.
set /p continue="Continue? (y/N): "
if /i not "%continue%"=="y" (
    echo Scan cancelled.
    exit /b 0
)

echo.
echo Starting comprehensive scan...
echo ==============================

REM Create output filename with timestamp
for /f "tokens=2 delims==" %%a in ('wmic OS Get localdatetime /value') do set "dt=%%a"
set "YY=%dt:~2,2%" & set "YYYY=%dt:~0,4%" & set "MM=%dt:~4,2%" & set "DD=%dt:~6,2%"
set "HH=%dt:~8,2%" & set "Min=%dt:~10,2%" & set "Sec=%dt:~12,2%"
set "timestamp=%YYYY%-%MM%-%DD%_%HH%-%Min%-%Sec%"
set "output_file=triageir-scan-%timestamp%.json"

echo Output file: %output_file%
echo.

REM Run the scan with verbose output
echo Command: target\release\triageir-cli.exe --verbose --output "%output_file%"
echo.
target\release\triageir-cli.exe --verbose --output "%output_file%"

REM Check results
if %ERRORLEVEL% EQU 0 (
    echo.
    echo ===========================
    echo SCAN COMPLETED SUCCESSFULLY
    echo ===========================
    echo.
    if exist "%output_file%" (
        echo ✓ Output file created: %output_file%
        
        REM Show file size
        for %%A in ("%output_file%") do set "filesize=%%~zA"
        echo ✓ File size: %filesize% bytes
        
        REM Show basic stats
        echo.
        echo Basic scan statistics:
        echo ----------------------
        findstr /c:"running_processes" "%output_file%" >nul && echo ✓ Process data collected
        findstr /c:"network_connections" "%output_file%" >nul && echo ✓ Network data collected
        findstr /c:"persistence_mechanisms" "%output_file%" >nul && echo ✓ Persistence data collected
        findstr /c:"event_logs" "%output_file%" >nul && echo ✓ Event log data collected
        findstr /c:"system_info" "%output_file%" >nul && echo ✓ System info collected
        
        echo.
        echo You can now:
        echo 1. Open the JSON file in a text editor to view raw data
        echo 2. Load the file in the TriageIR GUI for analysis
        echo 3. Validate the output with: python validate_output.py "%output_file%"
        echo.
        
        REM Ask if user wants to validate
        set /p validate="Validate the output now? (y/N): "
        if /i "%validate%"=="y" (
            where python >nul 2>nul
            if %ERRORLEVEL% EQU 0 (
                echo.
                echo Running validation...
                python validate_output.py "%output_file%"
            ) else (
                echo Python not found. Please install Python to use the validator.
            )
        )
        
    ) else (
        echo ✗ Output file was not created
        echo Check the error messages above for details.
    )
) else (
    echo.
    echo ===========================
    echo SCAN FAILED
    echo ===========================
    echo.
    echo Exit code: %ERRORLEVEL%
    echo.
    echo Common issues:
    echo - Insufficient privileges (try running as Administrator)
    echo - Antivirus blocking the executable
    echo - Missing dependencies
    echo - System compatibility issues
    echo.
    echo Check the error messages above for more details.
)

echo.
pause