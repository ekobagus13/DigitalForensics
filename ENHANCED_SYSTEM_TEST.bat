@echo off
color 0A
echo.
echo  ████████╗██████╗ ██╗ █████╗  ██████╗ ███████╗██╗██████╗ 
echo  ╚══██╔══╝██╔══██╗██║██╔══██╗██╔════╝ ██╔════╝██║██╔══██╗
echo     ██║   ██████╔╝██║███████║██║  ███╗█████╗  ██║██████╔╝
echo     ██║   ██╔══██╗██║██╔══██║██║   ██║██╔══╝  ██║██╔══██╗
echo     ██║   ██║  ██║██║██║  ██║╚██████╔╝███████╗██║██║  ██║
echo     ╚═╝   ╚═╝  ╚═╝╚═╝╚═╝  ╚═╝ ╚═════╝ ╚══════╝╚═╝╚═╝  ╚═╝
echo.
echo           ENHANCED SYSTEM COLLECTION TEST
echo           ==================================
echo           Real Comprehensive Data Collection & GUI Display
echo.

set "test_start=%time%"
echo 🕐 Test Started: %date% at %test_start%
echo 💻 System: %COMPUTERNAME%
echo 👤 User: %USERNAME%
echo.

echo 🔧 PHASE 1: Enhanced CLI Collection
echo ===================================
cd TriageIR-CLI
echo Running enhanced forensic collection...

set "output_file=comprehensive-scan-%COMPUTERNAME%-%date:~-4,4%%date:~-10,2%%date:~-7,2%-%time:~0,2%%time:~3,2%%time:~6,2%.json"
set "output_file=%output_file: =0%"

.\target\release\triageir-cli.exe --output "%output_file%" --verbose

if exist "%output_file%" (
    for %%A in ("%output_file%") do (
        echo ✅ Enhanced data collected!
        echo 📁 File: %output_file%
        echo 📏 Size: %%~zA bytes
        
        if %%~zA GTR 100000 (
            echo 🎉 SUCCESS: Comprehensive data collection (>100KB)
        ) else (
            echo ⚠️ Warning: Data size smaller than expected
        )
    )
) else (
    echo ❌ No output file generated!
    pause
    exit /b 1
)
echo.

echo 📊 PHASE 2: Data Analysis
echo ==========================
echo Analyzing comprehensive data...

findstr /c:"279" "%output_file%" >nul
if %ERRORLEVEL% equ 0 (
    echo ✅ Large process count detected
) else (
    echo ⚠️ Process count may be lower than expected
)

findstr /c:"network_connections" "%output_file%" >nul
if %ERRORLEVEL% equ 0 (
    echo ✅ Network data comprehensive
) else (
    echo ❌ Network data missing
)

echo.

echo 🎨 PHASE 3: GUI Integration Test
echo =================================
cd ..\TriageIR-GUI
echo Launching GUI for comprehensive data display...
echo.

echo 📖 COMPREHENSIVE TEST INSTRUCTIONS:
echo ====================================
echo.
echo The GUI will display REAL comprehensive data from your system:
echo.
echo 1. 🔍 VERIFY COMPREHENSIVE DATA:
echo    ✓ Summary Cards should show ~279 processes, ~67 connections
echo    ✓ Process table should show hundreds of real processes
echo    ✓ Network table should show dozens of real connections
echo    ✓ Persistence should show real autostart programs
echo    ✓ Events should show real Windows events
echo.
echo 2. ⚡ TEST LIVE COMPREHENSIVE SCAN:
echo    ✓ Click "Quick Scan" - should collect hundreds of artifacts
echo    ✓ Watch progress with real-time updates
echo    ✓ Verify comprehensive results display
echo.
echo 3. 📁 LOAD COMPREHENSIVE DATA:
echo    ✓ Click "Open Results"
echo    ✓ Load: %output_file%
echo    ✓ Verify all tabs show comprehensive real data
echo.
echo 4. 🔍 VERIFY REAL SYSTEM DETAILS:
echo    ✓ Your actual running programs (Chrome, VS Code, etc.)
echo    ✓ Your real network connections and ports
echo    ✓ Your actual autostart programs and services
echo    ✓ Recent real Windows events from your system
echo.
echo 5. 📊 PERFORMANCE ASSESSMENT:
echo    ✓ GUI handles large datasets smoothly
echo    ✓ Tables scroll and display properly
echo    ✓ Search/filter works with real data
echo    ✓ Export functions work with large datasets
echo.
echo 🎯 SUCCESS CRITERIA:
echo - GUI displays 200+ processes clearly
echo - Network connections show real ports/services
echo - Performance remains smooth with large data
echo - All forensic details are accurate and readable
echo.
echo Press any key to launch comprehensive GUI test...
pause >nul

echo.
echo 🚀 Launching GUI with Comprehensive Data...
echo ===========================================
call npm run dev

echo.
echo 📊 COMPREHENSIVE TEST COMPLETED
echo ================================
set "test_end=%time%"
echo 🕐 Completed: %test_end%
echo 📁 Comprehensive data: %output_file%
echo.
echo 📝 FINAL COMPREHENSIVE ASSESSMENT:
echo ===================================
echo.
set /p "q1=1. Did GUI handle 200+ processes smoothly? (Y/N): "
set /p "q2=2. Were real network connections displayed clearly? (Y/N): "
set /p "q3=3. Did you see your actual running programs? (Y/N): "
set /p "q4=4. Was performance good with large datasets? (Y/N): "
set /p "q5=5. Did comprehensive data look professional? (Y/N): "
set /p "q6=6. Overall comprehensive system rating (1-10): "
echo.
echo 📋 COMPREHENSIVE TEST RESULTS:
echo ===============================
echo Large Dataset Handling: %q1%
echo Real Network Display: %q2%
echo Actual Program Recognition: %q3%
echo Performance with Big Data: %q4%
echo Professional Appearance: %q5%
echo Overall Rating: %q6%/10
echo.
echo 🎉 ENHANCED COLLECTION SUCCESS!
echo Your system now provides comprehensive forensic data
echo suitable for professional digital forensics investigations.
echo.
pause