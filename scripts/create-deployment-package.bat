@echo off
REM TriageIR Deployment Package Creation Script
REM Creates a complete deployment package with all components

setlocal enabledelayedexpansion

echo ========================================
echo TriageIR Deployment Package Creator
echo ========================================
echo.

REM Set version and package name
set VERSION=1.0.0
set PACKAGE_NAME=TriageIR-v%VERSION%
set BUILD_DIR=build
set PACKAGE_DIR=%BUILD_DIR%\%PACKAGE_NAME%

REM Clean previous builds
echo [1/8] Cleaning previous builds...
if exist "%BUILD_DIR%" (
    rmdir /s /q "%BUILD_DIR%"
)
mkdir "%BUILD_DIR%"
mkdir "%PACKAGE_DIR%"

REM Build CLI component
echo [2/8] Building CLI component...
cd TriageIR-CLI
call build-professional.bat
if %ERRORLEVEL% neq 0 (
    echo ERROR: CLI build failed
    exit /b 1
)
cd ..

REM Build GUI component
echo [3/8] Building GUI component...
cd TriageIR-GUI
call npm install
if %ERRORLEVEL% neq 0 (
    echo ERROR: GUI npm install failed
    exit /b 1
)
call npm run build
if %ERRORLEVEL% neq 0 (
    echo ERROR: GUI build failed
    exit /b 1
)
cd ..

REM Copy CLI executable and dependencies
echo [4/8] Copying CLI components...
mkdir "%PACKAGE_DIR%\CLI"
copy "TriageIR-CLI\target\release\triageir-cli.exe" "%PACKAGE_DIR%\CLI\" >nul
if not exist "%PACKAGE_DIR%\CLI\triageir-cli.exe" (
    echo ERROR: CLI executable not found
    exit /b 1
)

REM Copy GUI application
echo [5/8] Copying GUI components...
mkdir "%PACKAGE_DIR%\GUI"
xcopy "TriageIR-GUI\dist\*" "%PACKAGE_DIR%\GUI\" /s /e /q >nul
if %ERRORLEVEL% neq 0 (
    echo ERROR: GUI copy failed
    exit /b 1
)

REM Copy documentation
echo [6/8] Copying documentation...
mkdir "%PACKAGE_DIR%\docs"
copy "docs\*.md" "%PACKAGE_DIR%\docs\" >nul
copy "README.md" "%PACKAGE_DIR%\" >nul
copy "TriageIR-CLI\README.md" "%PACKAGE_DIR%\CLI-README.md" >nul
copy "TriageIR-CLI\USAGE.md" "%PACKAGE_DIR%\CLI-USAGE.md" >nul
copy "TriageIR-CLI\PERFORMANCE.md" "%PACKAGE_DIR%\CLI-PERFORMANCE.md" >nul
copy "TriageIR-GUI\README.md" "%PACKAGE_DIR%\GUI-README.md" >nul

REM Copy test scripts and examples
echo [7/8] Copying test scripts and examples...
mkdir "%PACKAGE_DIR%\test-scripts"
copy "test-scripts\*.bat" "%PACKAGE_DIR%\test-scripts\" >nul

mkdir "%PACKAGE_DIR%\examples"
if exist "TriageIR-CLI\examples" (
    xcopy "TriageIR-CLI\examples\*" "%PACKAGE_DIR%\examples\" /s /e /q >nul
)

REM Create launcher scripts
echo [8/8] Creating launcher scripts...

REM CLI launcher
echo @echo off > "%PACKAGE_DIR%\TriageIR-CLI.bat"
echo REM TriageIR CLI Launcher >> "%PACKAGE_DIR%\TriageIR-CLI.bat"
echo cd /d "%%~dp0CLI" >> "%PACKAGE_DIR%\TriageIR-CLI.bat"
echo triageir-cli.exe %%* >> "%PACKAGE_DIR%\TriageIR-CLI.bat"

REM GUI launcher
echo @echo off > "%PACKAGE_DIR%\TriageIR-GUI.bat"
echo REM TriageIR GUI Launcher >> "%PACKAGE_DIR%\TriageIR-GUI.bat"
echo cd /d "%%~dp0GUI" >> "%PACKAGE_DIR%\TriageIR-GUI.bat"
echo start "" "TriageIR.exe" >> "%PACKAGE_DIR%\TriageIR-GUI.bat"

REM Quick start script
echo @echo off > "%PACKAGE_DIR%\Quick-Start.bat"
echo echo TriageIR Quick Start >> "%PACKAGE_DIR%\Quick-Start.bat"
echo echo ================== >> "%PACKAGE_DIR%\Quick-Start.bat"
echo echo. >> "%PACKAGE_DIR%\Quick-Start.bat"
echo echo 1. CLI: Run TriageIR-CLI.bat >> "%PACKAGE_DIR%\Quick-Start.bat"
echo echo 2. GUI: Run TriageIR-GUI.bat >> "%PACKAGE_DIR%\Quick-Start.bat"
echo echo. >> "%PACKAGE_DIR%\Quick-Start.bat"
echo echo For help: >> "%PACKAGE_DIR%\Quick-Start.bat"
echo echo   CLI: TriageIR-CLI.bat --help >> "%PACKAGE_DIR%\Quick-Start.bat"
echo echo   GUI: See GUI-README.md >> "%PACKAGE_DIR%\Quick-Start.bat"
echo echo. >> "%PACKAGE_DIR%\Quick-Start.bat"
echo pause >> "%PACKAGE_DIR%\Quick-Start.bat"

REM Create installation guide
echo # TriageIR Installation Guide > "%PACKAGE_DIR%\INSTALL.md"
echo. >> "%PACKAGE_DIR%\INSTALL.md"
echo ## Quick Installation >> "%PACKAGE_DIR%\INSTALL.md"
echo. >> "%PACKAGE_DIR%\INSTALL.md"
echo 1. Extract this package to your preferred location >> "%PACKAGE_DIR%\INSTALL.md"
echo 2. Run `Quick-Start.bat` to see available options >> "%PACKAGE_DIR%\INSTALL.md"
echo 3. For CLI usage: Run `TriageIR-CLI.bat --help` >> "%PACKAGE_DIR%\INSTALL.md"
echo 4. For GUI usage: Run `TriageIR-GUI.bat` >> "%PACKAGE_DIR%\INSTALL.md"
echo. >> "%PACKAGE_DIR%\INSTALL.md"
echo ## System Requirements >> "%PACKAGE_DIR%\INSTALL.md"
echo. >> "%PACKAGE_DIR%\INSTALL.md"
echo - Windows 10 or later >> "%PACKAGE_DIR%\INSTALL.md"
echo - Administrator privileges recommended >> "%PACKAGE_DIR%\INSTALL.md"
echo - 4 GB RAM minimum, 8 GB recommended >> "%PACKAGE_DIR%\INSTALL.md"
echo - 100 MB disk space for installation >> "%PACKAGE_DIR%\INSTALL.md"
echo. >> "%PACKAGE_DIR%\INSTALL.md"
echo ## Documentation >> "%PACKAGE_DIR%\INSTALL.md"
echo. >> "%PACKAGE_DIR%\INSTALL.md"
echo - `docs/USER_MANUAL.md` - Complete user manual >> "%PACKAGE_DIR%\INSTALL.md"
echo - `docs/DEVELOPER_GUIDE.md` - Developer documentation >> "%PACKAGE_DIR%\INSTALL.md"
echo - `docs/API_REFERENCE.md` - API reference >> "%PACKAGE_DIR%\INSTALL.md"
echo - `CLI-README.md` - CLI component documentation >> "%PACKAGE_DIR%\INSTALL.md"
echo - `GUI-README.md` - GUI component documentation >> "%PACKAGE_DIR%\INSTALL.md"

REM Create version info file
echo TriageIR Version Information > "%PACKAGE_DIR%\VERSION.txt"
echo ============================= >> "%PACKAGE_DIR%\VERSION.txt"
echo. >> "%PACKAGE_DIR%\VERSION.txt"
echo Version: %VERSION% >> "%PACKAGE_DIR%\VERSION.txt"
echo Build Date: %DATE% %TIME% >> "%PACKAGE_DIR%\VERSION.txt"
echo. >> "%PACKAGE_DIR%\VERSION.txt"
echo Components: >> "%PACKAGE_DIR%\VERSION.txt"
echo - TriageIR-CLI: Rust-based forensic collection engine >> "%PACKAGE_DIR%\VERSION.txt"
echo - TriageIR-GUI: Electron-based graphical interface >> "%PACKAGE_DIR%\VERSION.txt"
echo. >> "%PACKAGE_DIR%\VERSION.txt"
echo Package Contents: >> "%PACKAGE_DIR%\VERSION.txt"
echo - CLI/: Command-line interface executable >> "%PACKAGE_DIR%\VERSION.txt"
echo - GUI/: Graphical user interface application >> "%PACKAGE_DIR%\VERSION.txt"
echo - docs/: Complete documentation >> "%PACKAGE_DIR%\VERSION.txt"
echo - test-scripts/: Testing and validation scripts >> "%PACKAGE_DIR%\VERSION.txt"
echo - examples/: Usage examples and sample data >> "%PACKAGE_DIR%\VERSION.txt"

REM Create ZIP package
echo.
echo Creating ZIP package...
powershell -command "Compress-Archive -Path '%PACKAGE_DIR%\*' -DestinationPath '%BUILD_DIR%\%PACKAGE_NAME%.zip' -Force"
if %ERRORLEVEL% neq 0 (
    echo ERROR: ZIP creation failed
    exit /b 1
)

REM Create installer (if NSIS is available)
where makensis >nul 2>&1
if %ERRORLEVEL% equ 0 (
    echo Creating installer...
    call scripts\create-installer.nsi "%PACKAGE_DIR%" "%BUILD_DIR%\%PACKAGE_NAME%-Setup.exe"
) else (
    echo NSIS not found, skipping installer creation
)

REM Display results
echo.
echo ========================================
echo Deployment Package Created Successfully
echo ========================================
echo.
echo Package Location: %BUILD_DIR%\%PACKAGE_NAME%
echo ZIP Archive: %BUILD_DIR%\%PACKAGE_NAME%.zip
echo.
echo Package Contents:
echo - CLI executable and documentation
echo - GUI application and resources
echo - Complete documentation suite
echo - Test scripts and examples
echo - Installation and quick start guides
echo.
echo Package Size:
for %%A in ("%BUILD_DIR%\%PACKAGE_NAME%.zip") do echo - ZIP: %%~zA bytes
echo.
echo Ready for distribution!
echo.

REM Verify package integrity
echo Verifying package integrity...
if not exist "%PACKAGE_DIR%\CLI\triageir-cli.exe" (
    echo WARNING: CLI executable missing
)
if not exist "%PACKAGE_DIR%\GUI" (
    echo WARNING: GUI directory missing
)
if not exist "%PACKAGE_DIR%\docs\USER_MANUAL.md" (
    echo WARNING: User manual missing
)

echo.
echo Verification complete. Package is ready for deployment.
pause