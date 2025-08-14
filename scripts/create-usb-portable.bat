@echo off
REM TriageIR USB Portable Package Creator
REM Creates a zero-installation USB-ready package

setlocal enabledelayedexpansion

echo ========================================
echo TriageIR USB Portable Package Creator
echo ========================================
echo.

set VERSION=1.0.0
set USB_PACKAGE=TriageIR-USB-Portable
set BUILD_DIR=build
set USB_DIR=%BUILD_DIR%\%USB_PACKAGE%

REM Clean and create directories
if exist "%BUILD_DIR%" rmdir /s /q "%BUILD_DIR%"
mkdir "%BUILD_DIR%"
mkdir "%USB_DIR%"
mkdir "%USB_DIR%\CLI"
mkdir "%USB_DIR%\GUI"
mkdir "%USB_DIR%\Tools"
mkdir "%USB_DIR%\Output"
mkdir "%USB_DIR%\Logs"

echo [1/8] Building CLI with static linking...
cd TriageIR-CLI

REM Ensure static linking for complete portability
set RUSTFLAGS=-C target-feature=+crt-static
cargo build --release --target x86_64-pc-windows-msvc

if %ERRORLEVEL% neq 0 (
    echo ERROR: CLI build failed
    exit /b 1
)
cd ..

echo [2/8] Building portable GUI...
cd TriageIR-GUI

REM Build GUI with portable configuration
npm install
npm run build:portable

if %ERRORLEVEL% neq 0 (
    echo ERROR: GUI build failed
    exit /b 1
)
cd ..

echo [3/8] Copying CLI components...
copy "TriageIR-CLI\target\x86_64-pc-windows-msvc\release\triageir-cli.exe" "%USB_DIR%\CLI\" >nul

REM Copy any required DLLs (should be none with static linking)
REM But include Visual C++ Redistributable just in case
if exist "redist\vcredist_x64.exe" (
    copy "redist\vcredist_x64.exe" "%USB_DIR%\Tools\" >nul
)

echo [4/8] Copying GUI components...
xcopy "TriageIR-GUI\dist-portable\*" "%USB_DIR%\GUI\" /s /e /q >nul

echo [5/8] Creating USB launcher scripts...

REM Main USB launcher
echo @echo off > "%USB_DIR%\TriageIR-USB.bat"
echo REM TriageIR USB Portable Launcher >> "%USB_DIR%\TriageIR-USB.bat"
echo REM Runs directly from USB without installation >> "%USB_DIR%\TriageIR-USB.bat"
echo. >> "%USB_DIR%\TriageIR-USB.bat"
echo set USB_DRIVE=%%~dp0 >> "%USB_DIR%\TriageIR-USB.bat"
echo set USB_DRIVE=%%USB_DRIVE:~0,-1%% >> "%USB_DIR%\TriageIR-USB.bat"
echo. >> "%USB_DIR%\TriageIR-USB.bat"
echo REM Set portable environment >> "%USB_DIR%\TriageIR-USB.bat"
echo set TRIAGEIR_PORTABLE=1 >> "%USB_DIR%\TriageIR-USB.bat"
echo set TRIAGEIR_USB_DRIVE=%%USB_DRIVE%% >> "%USB_DIR%\TriageIR-USB.bat"
echo set TRIAGEIR_OUTPUT_DIR=%%USB_DRIVE%%\Output >> "%USB_DIR%\TriageIR-USB.bat"
echo set TRIAGEIR_LOG_DIR=%%USB_DRIVE%%\Logs >> "%USB_DIR%\TriageIR-USB.bat"
echo. >> "%USB_DIR%\TriageIR-USB.bat"
echo REM Add CLI to PATH temporarily >> "%USB_DIR%\TriageIR-USB.bat"
echo set PATH=%%USB_DRIVE%%\CLI;%%PATH%% >> "%USB_DIR%\TriageIR-USB.bat"
echo. >> "%USB_DIR%\TriageIR-USB.bat"
echo echo TriageIR USB Portable Mode >> "%USB_DIR%\TriageIR-USB.bat"
echo echo ========================= >> "%USB_DIR%\TriageIR-USB.bat"
echo echo USB Drive: %%USB_DRIVE%% >> "%USB_DIR%\TriageIR-USB.bat"
echo echo Output Dir: %%TRIAGEIR_OUTPUT_DIR%% >> "%USB_DIR%\TriageIR-USB.bat"
echo echo. >> "%USB_DIR%\TriageIR-USB.bat"
echo echo Available commands: >> "%USB_DIR%\TriageIR-USB.bat"
echo echo   1. CLI: triageir-cli.exe [options] >> "%USB_DIR%\TriageIR-USB.bat"
echo echo   2. GUI: start TriageIR-GUI.bat >> "%USB_DIR%\TriageIR-USB.bat"
echo echo   3. Help: triageir-cli.exe --help >> "%USB_DIR%\TriageIR-USB.bat"
echo echo. >> "%USB_DIR%\TriageIR-USB.bat"
echo cmd /k >> "%USB_DIR%\TriageIR-USB.bat"

REM CLI launcher
echo @echo off > "%USB_DIR%\TriageIR-CLI.bat"
echo set USB_DRIVE=%%~dp0 >> "%USB_DIR%\TriageIR-CLI.bat"
echo set USB_DRIVE=%%USB_DRIVE:~0,-1%% >> "%USB_DIR%\TriageIR-CLI.bat"
echo set TRIAGEIR_PORTABLE=1 >> "%USB_DIR%\TriageIR-CLI.bat"
echo set TRIAGEIR_OUTPUT_DIR=%%USB_DRIVE%%\Output >> "%USB_DIR%\TriageIR-CLI.bat"
echo cd /d "%%USB_DRIVE%%\CLI" >> "%USB_DIR%\TriageIR-CLI.bat"
echo triageir-cli.exe %%* >> "%USB_DIR%\TriageIR-CLI.bat"

REM GUI launcher
echo @echo off > "%USB_DIR%\TriageIR-GUI.bat"
echo set USB_DRIVE=%%~dp0 >> "%USB_DIR%\TriageIR-GUI.bat"
echo set USB_DRIVE=%%USB_DRIVE:~0,-1%% >> "%USB_DIR%\TriageIR-GUI.bat"
echo set TRIAGEIR_PORTABLE=1 >> "%USB_DIR%\TriageIR-GUI.bat"
echo set TRIAGEIR_CLI_PATH=%%USB_DRIVE%%\CLI\triageir-cli.exe >> "%USB_DIR%\TriageIR-GUI.bat"
echo set TRIAGEIR_OUTPUT_DIR=%%USB_DRIVE%%\Output >> "%USB_DIR%\TriageIR-GUI.bat"
echo cd /d "%%USB_DRIVE%%\GUI" >> "%USB_DIR%\TriageIR-GUI.bat"
echo start "" "TriageIR.exe" >> "%USB_DIR%\TriageIR-GUI.bat"

REM Quick scan launcher
echo @echo off > "%USB_DIR%\Quick-Scan.bat"
echo set USB_DRIVE=%%~dp0 >> "%USB_DIR%\Quick-Scan.bat"
echo set USB_DRIVE=%%USB_DRIVE:~0,-1%% >> "%USB_DIR%\Quick-Scan.bat"
echo set TIMESTAMP=%%DATE:~-4,4%%%%DATE:~-10,2%%%%DATE:~-7,2%%_%%TIME:~0,2%%%%TIME:~3,2%%%%TIME:~6,2%% >> "%USB_DIR%\Quick-Scan.bat"
echo set TIMESTAMP=%%TIMESTAMP: =0%% >> "%USB_DIR%\Quick-Scan.bat"
echo echo Running quick forensic scan... >> "%USB_DIR%\Quick-Scan.bat"
echo "%%USB_DRIVE%%\CLI\triageir-cli.exe" --output "%%USB_DRIVE%%\Output\quick_scan_%%TIMESTAMP%%.json" --verbose >> "%USB_DIR%\Quick-Scan.bat"
echo echo Scan completed. Results saved to Output folder. >> "%USB_DIR%\Quick-Scan.bat"
echo pause >> "%USB_DIR%\Quick-Scan.bat"

echo [6/8] Creating USB documentation...

REM USB README
echo # TriageIR USB Portable Edition > "%USB_DIR%\README-USB.md"
echo. >> "%USB_DIR%\README-USB.md"
echo ## Zero-Installation Forensic Toolkit >> "%USB_DIR%\README-USB.md"
echo. >> "%USB_DIR%\README-USB.md"
echo This is the USB portable edition of TriageIR that runs directly from >> "%USB_DIR%\README-USB.md"
echo your USB drive without requiring any installation on the target system. >> "%USB_DIR%\README-USB.md"
echo. >> "%USB_DIR%\README-USB.md"
echo ### Quick Start >> "%USB_DIR%\README-USB.md"
echo. >> "%USB_DIR%\README-USB.md"
echo 1. **Plug in USB drive** >> "%USB_DIR%\README-USB.md"
echo 2. **Run TriageIR-USB.bat** for interactive mode >> "%USB_DIR%\README-USB.md"
echo 3. **Or run Quick-Scan.bat** for immediate scan >> "%USB_DIR%\README-USB.md"
echo. >> "%USB_DIR%\README-USB.md"
echo ### Directory Structure >> "%USB_DIR%\README-USB.md"
echo. >> "%USB_DIR%\README-USB.md"
echo - `CLI/` - Command-line forensic engine >> "%USB_DIR%\README-USB.md"
echo - `GUI/` - Graphical user interface >> "%USB_DIR%\README-USB.md"
echo - `Output/` - Scan results and reports >> "%USB_DIR%\README-USB.md"
echo - `Logs/` - Application logs >> "%USB_DIR%\README-USB.md"
echo - `Tools/` - Additional utilities >> "%USB_DIR%\README-USB.md"
echo. >> "%USB_DIR%\README-USB.md"
echo ### Usage Examples >> "%USB_DIR%\README-USB.md"
echo. >> "%USB_DIR%\README-USB.md"
echo ```cmd >> "%USB_DIR%\README-USB.md"
echo # Quick system scan >> "%USB_DIR%\README-USB.md"
echo TriageIR-CLI.bat --output Output\my_scan.json >> "%USB_DIR%\README-USB.md"
echo. >> "%USB_DIR%\README-USB.md"
echo # Incident response scan >> "%USB_DIR%\README-USB.md"
echo TriageIR-CLI.bat --only processes,network,persistence --output Output\incident.json >> "%USB_DIR%\README-USB.md"
echo. >> "%USB_DIR%\README-USB.md"
echo # Launch GUI >> "%USB_DIR%\README-USB.md"
echo TriageIR-GUI.bat >> "%USB_DIR%\README-USB.md"
echo ``` >> "%USB_DIR%\README-USB.md"
echo. >> "%USB_DIR%\README-USB.md"
echo ### Features >> "%USB_DIR%\README-USB.md"
echo. >> "%USB_DIR%\README-USB.md"
echo - ✅ **Zero Installation** - No system modifications >> "%USB_DIR%\README-USB.md"
echo - ✅ **Portable** - Runs from any USB drive >> "%USB_DIR%\README-USB.md"
echo - ✅ **Self-Contained** - All dependencies included >> "%USB_DIR%\README-USB.md"
echo - ✅ **Forensically Sound** - Read-only system access >> "%USB_DIR%\README-USB.md"
echo - ✅ **Professional** - Complete forensic toolkit >> "%USB_DIR%\README-USB.md"

echo [7/8] Creating USB utilities...

REM USB drive detector
echo @echo off > "%USB_DIR%\Tools\detect-usb.bat"
echo REM Detect USB drive letter automatically >> "%USB_DIR%\Tools\detect-usb.bat"
echo for /f "tokens=1" %%%%i in ('wmic logicaldisk where "drivetype=2" get deviceid /format:value ^| find "="') do ( >> "%USB_DIR%\Tools\detect-usb.bat"
echo     set USB_LETTER=%%%%i >> "%USB_DIR%\Tools\detect-usb.bat"
echo     set USB_LETTER=!USB_LETTER:~9,2! >> "%USB_DIR%\Tools\detect-usb.bat"
echo     echo USB Drive detected: !USB_LETTER! >> "%USB_DIR%\Tools\detect-usb.bat"
echo ^) >> "%USB_DIR%\Tools\detect-usb.bat"

REM System info collector
echo @echo off > "%USB_DIR%\Tools\collect-system-info.bat"
echo set USB_DRIVE=%%~dp0.. >> "%USB_DIR%\Tools\collect-system-info.bat"
echo set TIMESTAMP=%%DATE:~-4,4%%%%DATE:~-10,2%%%%DATE:~-7,2%%_%%TIME:~0,2%%%%TIME:~3,2%%%%TIME:~6,2%% >> "%USB_DIR%\Tools\collect-system-info.bat"
echo set TIMESTAMP=%%TIMESTAMP: =0%% >> "%USB_DIR%\Tools\collect-system-info.bat"
echo echo Collecting system information... >> "%USB_DIR%\Tools\collect-system-info.bat"
echo systeminfo ^> "%%USB_DRIVE%%\Output\systeminfo_%%TIMESTAMP%%.txt" >> "%USB_DIR%\Tools\collect-system-info.bat"
echo ipconfig /all ^> "%%USB_DRIVE%%\Output\ipconfig_%%TIMESTAMP%%.txt" >> "%USB_DIR%\Tools\collect-system-info.bat"
echo netstat -an ^> "%%USB_DRIVE%%\Output\netstat_%%TIMESTAMP%%.txt" >> "%USB_DIR%\Tools\collect-system-info.bat"
echo tasklist /v ^> "%%USB_DRIVE%%\Output\tasklist_%%TIMESTAMP%%.txt" >> "%USB_DIR%\Tools\collect-system-info.bat"
echo echo System information collected to Output folder. >> "%USB_DIR%\Tools\collect-system-info.bat"

echo [8/8] Creating autorun configuration...

REM Create autorun.inf for USB (optional - may be blocked by security)
echo [autorun] > "%USB_DIR%\autorun.inf"
echo icon=CLI\triageir-cli.exe >> "%USB_DIR%\autorun.inf"
echo label=TriageIR Forensic Toolkit >> "%USB_DIR%\autorun.inf"
echo action=Launch TriageIR Forensic Toolkit >> "%USB_DIR%\autorun.inf"
echo open=TriageIR-USB.bat >> "%USB_DIR%\autorun.inf"

REM Copy documentation
mkdir "%USB_DIR%\docs"
if exist "docs" xcopy "docs\*.md" "%USB_DIR%\docs\" /q >nul

REM Copy examples
mkdir "%USB_DIR%\examples"
if exist "examples" xcopy "examples\*" "%USB_DIR%\examples\" /s /e /q >nul

echo.
echo ========================================
echo USB Portable Package Created!
echo ========================================
echo.
echo Package Location: %USB_DIR%
echo.
echo To deploy to USB:
echo 1. Insert USB drive
echo 2. Copy contents of %USB_DIR% to USB root
echo 3. Safely eject USB
echo 4. Plug into target system and run TriageIR-USB.bat
echo.
echo Features:
echo ✅ Zero installation required
echo ✅ Runs directly from USB
echo ✅ Self-contained with all dependencies
echo ✅ Automatic output organization
echo ✅ Portable environment detection
echo.

REM Create deployment instructions
echo # USB Deployment Instructions > "%USB_DIR%\DEPLOY-TO-USB.md"
echo. >> "%USB_DIR%\DEPLOY-TO-USB.md"
echo ## Steps to Deploy TriageIR to USB Drive >> "%USB_DIR%\DEPLOY-TO-USB.md"
echo. >> "%USB_DIR%\DEPLOY-TO-USB.md"
echo 1. **Format USB Drive** (optional but recommended): >> "%USB_DIR%\DEPLOY-TO-USB.md"
echo    - Use FAT32 for maximum compatibility >> "%USB_DIR%\DEPLOY-TO-USB.md"
echo    - Or NTFS for large file support >> "%USB_DIR%\DEPLOY-TO-USB.md"
echo. >> "%USB_DIR%\DEPLOY-TO-USB.md"
echo 2. **Copy Files**: >> "%USB_DIR%\DEPLOY-TO-USB.md"
echo    - Copy ALL contents of this folder to USB root >> "%USB_DIR%\DEPLOY-TO-USB.md"
echo    - Maintain directory structure >> "%USB_DIR%\DEPLOY-TO-USB.md"
echo. >> "%USB_DIR%\DEPLOY-TO-USB.md"
echo 3. **Verify Deployment**: >> "%USB_DIR%\DEPLOY-TO-USB.md"
echo    - Check that TriageIR-USB.bat exists in USB root >> "%USB_DIR%\DEPLOY-TO-USB.md"
echo    - Verify CLI and GUI folders are present >> "%USB_DIR%\DEPLOY-TO-USB.md"
echo. >> "%USB_DIR%\DEPLOY-TO-USB.md"
echo 4. **Usage on Target System**: >> "%USB_DIR%\DEPLOY-TO-USB.md"
echo    - Insert USB drive >> "%USB_DIR%\DEPLOY-TO-USB.md"
echo    - Navigate to USB drive >> "%USB_DIR%\DEPLOY-TO-USB.md"
echo    - Double-click TriageIR-USB.bat >> "%USB_DIR%\DEPLOY-TO-USB.md"
echo    - Or run Quick-Scan.bat for immediate scan >> "%USB_DIR%\DEPLOY-TO-USB.md"

echo USB portable package ready for deployment!
pause