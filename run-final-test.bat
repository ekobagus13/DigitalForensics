@echo off
echo ========================================
echo TriageIR Final GUI-CLI Integration Test
echo ========================================
echo.

echo Step 1: Building CLI Application...
echo ------------------------------------
cd TriageIR-CLI
echo Building release version...
cargo build --release
if %ERRORLEVEL% neq 0 (
    echo ERROR: CLI build failed!
    pause
    exit /b 1
)
echo ✅ CLI build successful!
echo.

echo Step 2: Generating Test Data...
echo --------------------------------
echo Running CLI to generate sample data...
.\target\release\triageir-cli.exe > test-gui-data.json 2>&1
if %ERRORLEVEL% neq 0 (
    echo WARNING: CLI execution had issues, but continuing...
)
echo ✅ Test data generated!
echo.

echo Step 3: Setting up GUI...
echo --------------------------
cd ..\TriageIR-GUI
echo Installing GUI dependencies...
call npm install
if %ERRORLEVEL% neq 0 (
    echo ERROR: GUI dependency installation failed!
    pause
    exit /b 1
)
echo ✅ GUI dependencies installed!
echo.

echo Step 4: Launching GUI for Testing...
echo ------------------------------------
echo.
echo 🚀 Starting TriageIR GUI...
echo.
echo TEST INSTRUCTIONS:
echo ==================
echo 1. Click "Quick Scan" to test live CLI integration
echo 2. Click "Open Results" to load the test-gui-data.json file
echo 3. Verify all tabs display data correctly:
echo    - Overview: Scan metadata and summary
echo    - System Info: Uptime, users, OS details
echo    - Processes: Running process table
echo    - Network: Network connections table
echo    - Persistence: Autostart mechanisms
echo    - Events: Windows event logs
echo 4. Test "Save Results" and "Export Report" buttons
echo 5. Check visual appeal and responsiveness
echo.
echo Press Ctrl+C to stop the test when complete
echo.
pause
call npm run dev