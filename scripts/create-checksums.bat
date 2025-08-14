@echo off
REM TriageIR Checksum Generation Script
REM Generates SHA-256 checksums for all deployment artifacts

setlocal enabledelayedexpansion

echo ========================================
echo TriageIR Checksum Generation
echo ========================================
echo.

set BUILD_DIR=build
set CHECKSUM_FILE=%BUILD_DIR%\checksums.txt

if not exist "%BUILD_DIR%" (
    echo ERROR: Build directory not found: %BUILD_DIR%
    echo Please run create-deployment-package.bat first
    exit /b 1
)

echo Generating checksums for deployment artifacts...
echo.

REM Create checksums file
echo TriageIR v1.0.0 - SHA-256 Checksums > "%CHECKSUM_FILE%"
echo Generated: %DATE% %TIME% >> "%CHECKSUM_FILE%"
echo ========================================== >> "%CHECKSUM_FILE%"
echo. >> "%CHECKSUM_FILE%"

REM Function to calculate and display checksum
set CHECKSUM_COUNT=0

REM Check for PowerShell availability
powershell -command "Get-Command Get-FileHash" >nul 2>&1
if %ERRORLEVEL% neq 0 (
    echo ERROR: PowerShell with Get-FileHash not available
    echo Please use Windows 10 or later, or install PowerShell Core
    exit /b 1
)

echo Calculating checksums...
echo.

REM Process all files in build directory
for /r "%BUILD_DIR%" %%F in (*) do (
    set "FILEPATH=%%F"
    set "FILENAME=%%~nxF"
    
    REM Skip the checksums file itself and directories
    if not "!FILENAME!"=="checksums.txt" (
        if not "!FILENAME!"=="." (
            if not "!FILENAME!".==".." (
                echo Processing: !FILENAME!
                
                REM Calculate SHA-256 hash using PowerShell
                for /f "tokens=1" %%H in ('powershell -command "Get-FileHash -Algorithm SHA256 '!FILEPATH!' | Select-Object -ExpandProperty Hash"') do (
                    set "HASH=%%H"
                    
                    REM Get relative path
                    set "RELPATH=!FILEPATH:%BUILD_DIR%\=!"
                    
                    REM Write to checksums file
                    echo !HASH!  !RELPATH! >> "%CHECKSUM_FILE%"
                    
                    REM Display progress
                    echo   SHA-256: !HASH!
                    
                    set /a CHECKSUM_COUNT+=1
                )
                echo.
            )
        )
    )
)

REM Add verification instructions to checksums file
echo. >> "%CHECKSUM_FILE%"
echo Verification Instructions: >> "%CHECKSUM_FILE%"
echo ========================= >> "%CHECKSUM_FILE%"
echo. >> "%CHECKSUM_FILE%"
echo Windows PowerShell: >> "%CHECKSUM_FILE%"
echo   Get-FileHash -Algorithm SHA256 filename.ext >> "%CHECKSUM_FILE%"
echo. >> "%CHECKSUM_FILE%"
echo Linux/macOS: >> "%CHECKSUM_FILE%"
echo   sha256sum filename.ext >> "%CHECKSUM_FILE%"
echo. >> "%CHECKSUM_FILE%"
echo Verify all checksums: >> "%CHECKSUM_FILE%"
echo   sha256sum -c checksums.txt >> "%CHECKSUM_FILE%"
echo. >> "%CHECKSUM_FILE%"

REM Create verification script
echo Creating verification script...
echo @echo off > "%BUILD_DIR%\verify-checksums.bat"
echo REM TriageIR Checksum Verification Script >> "%BUILD_DIR%\verify-checksums.bat"
echo. >> "%BUILD_DIR%\verify-checksums.bat"
echo echo Verifying TriageIR deployment checksums... >> "%BUILD_DIR%\verify-checksums.bat"
echo echo. >> "%BUILD_DIR%\verify-checksums.bat"
echo. >> "%BUILD_DIR%\verify-checksums.bat"
echo set VERIFIED=0 >> "%BUILD_DIR%\verify-checksums.bat"
echo set FAILED=0 >> "%BUILD_DIR%\verify-checksums.bat"
echo. >> "%BUILD_DIR%\verify-checksums.bat"

REM Add verification commands for each file
for /f "skip=4 tokens=1,2*" %%A in ("%CHECKSUM_FILE%") do (
    if not "%%A"=="" (
        if not "%%B"=="" (
            echo echo Verifying: %%B >> "%BUILD_DIR%\verify-checksums.bat"
            echo for /f "tokens=1" %%%%H in ('powershell -command "Get-FileHash -Algorithm SHA256 '%%B' | Select-Object -ExpandProperty Hash"'^) do ^( >> "%BUILD_DIR%\verify-checksums.bat"
            echo     if "%%%%H"=="%%A" ^( >> "%BUILD_DIR%\verify-checksums.bat"
            echo         echo   ✓ PASS >> "%BUILD_DIR%\verify-checksums.bat"
            echo         set /a VERIFIED+=1 >> "%BUILD_DIR%\verify-checksums.bat"
            echo     ^) else ^( >> "%BUILD_DIR%\verify-checksums.bat"
            echo         echo   ✗ FAIL >> "%BUILD_DIR%\verify-checksums.bat"
            echo         set /a FAILED+=1 >> "%BUILD_DIR%\verify-checksums.bat"
            echo     ^) >> "%BUILD_DIR%\verify-checksums.bat"
            echo ^) >> "%BUILD_DIR%\verify-checksums.bat"
        )
    )
)

echo. >> "%BUILD_DIR%\verify-checksums.bat"
echo echo. >> "%BUILD_DIR%\verify-checksums.bat"
echo echo Verification Results: >> "%BUILD_DIR%\verify-checksums.bat"
echo echo ==================== >> "%BUILD_DIR%\verify-checksums.bat"
echo echo Verified: %%VERIFIED%% >> "%BUILD_DIR%\verify-checksums.bat"
echo echo Failed: %%FAILED%% >> "%BUILD_DIR%\verify-checksums.bat"
echo. >> "%BUILD_DIR%\verify-checksums.bat"
echo if %%FAILED%% equ 0 ^( >> "%BUILD_DIR%\verify-checksums.bat"
echo     echo ✓ ALL CHECKSUMS VERIFIED >> "%BUILD_DIR%\verify-checksums.bat"
echo     exit /b 0 >> "%BUILD_DIR%\verify-checksums.bat"
echo ^) else ^( >> "%BUILD_DIR%\verify-checksums.bat"
echo     echo ✗ CHECKSUM VERIFICATION FAILED >> "%BUILD_DIR%\verify-checksums.bat"
echo     exit /b 1 >> "%BUILD_DIR%\verify-checksums.bat"
echo ^) >> "%BUILD_DIR%\verify-checksums.bat"

REM Create PowerShell verification script
echo Creating PowerShell verification script...
echo # TriageIR Checksum Verification Script > "%BUILD_DIR%\verify-checksums.ps1"
echo # PowerShell version for cross-platform compatibility >> "%BUILD_DIR%\verify-checksums.ps1"
echo. >> "%BUILD_DIR%\verify-checksums.ps1"
echo Write-Host "Verifying TriageIR deployment checksums..." >> "%BUILD_DIR%\verify-checksums.ps1"
echo Write-Host "" >> "%BUILD_DIR%\verify-checksums.ps1"
echo. >> "%BUILD_DIR%\verify-checksums.ps1"
echo $verified = 0 >> "%BUILD_DIR%\verify-checksums.ps1"
echo $failed = 0 >> "%BUILD_DIR%\verify-checksums.ps1"
echo. >> "%BUILD_DIR%\verify-checksums.ps1"
echo # Read checksums file >> "%BUILD_DIR%\verify-checksums.ps1"
echo $checksums = Get-Content "checksums.txt" ^| Where-Object { $_ -match "^[A-F0-9]{64}" } >> "%BUILD_DIR%\verify-checksums.ps1"
echo. >> "%BUILD_DIR%\verify-checksums.ps1"
echo foreach ($line in $checksums) { >> "%BUILD_DIR%\verify-checksums.ps1"
echo     $parts = $line -split "  " >> "%BUILD_DIR%\verify-checksums.ps1"
echo     $expectedHash = $parts[0] >> "%BUILD_DIR%\verify-checksums.ps1"
echo     $filename = $parts[1] >> "%BUILD_DIR%\verify-checksums.ps1"
echo. >> "%BUILD_DIR%\verify-checksums.ps1"
echo     if (Test-Path $filename) { >> "%BUILD_DIR%\verify-checksums.ps1"
echo         Write-Host "Verifying: $filename" >> "%BUILD_DIR%\verify-checksums.ps1"
echo         $actualHash = (Get-FileHash -Algorithm SHA256 $filename).Hash >> "%BUILD_DIR%\verify-checksums.ps1"
echo. >> "%BUILD_DIR%\verify-checksums.ps1"
echo         if ($actualHash -eq $expectedHash) { >> "%BUILD_DIR%\verify-checksums.ps1"
echo             Write-Host "  ✓ PASS" -ForegroundColor Green >> "%BUILD_DIR%\verify-checksums.ps1"
echo             $verified++ >> "%BUILD_DIR%\verify-checksums.ps1"
echo         } else { >> "%BUILD_DIR%\verify-checksums.ps1"
echo             Write-Host "  ✗ FAIL" -ForegroundColor Red >> "%BUILD_DIR%\verify-checksums.ps1"
echo             $failed++ >> "%BUILD_DIR%\verify-checksums.ps1"
echo         } >> "%BUILD_DIR%\verify-checksums.ps1"
echo     } else { >> "%BUILD_DIR%\verify-checksums.ps1"
echo         Write-Host "  ✗ FILE NOT FOUND" -ForegroundColor Red >> "%BUILD_DIR%\verify-checksums.ps1"
echo         $failed++ >> "%BUILD_DIR%\verify-checksums.ps1"
echo     } >> "%BUILD_DIR%\verify-checksums.ps1"
echo } >> "%BUILD_DIR%\verify-checksums.ps1"
echo. >> "%BUILD_DIR%\verify-checksums.ps1"
echo Write-Host "" >> "%BUILD_DIR%\verify-checksums.ps1"
echo Write-Host "Verification Results:" >> "%BUILD_DIR%\verify-checksums.ps1"
echo Write-Host "====================" >> "%BUILD_DIR%\verify-checksums.ps1"
echo Write-Host "Verified: $verified" >> "%BUILD_DIR%\verify-checksums.ps1"
echo Write-Host "Failed: $failed" >> "%BUILD_DIR%\verify-checksums.ps1"
echo. >> "%BUILD_DIR%\verify-checksums.ps1"
echo if ($failed -eq 0) { >> "%BUILD_DIR%\verify-checksums.ps1"
echo     Write-Host "✓ ALL CHECKSUMS VERIFIED" -ForegroundColor Green >> "%BUILD_DIR%\verify-checksums.ps1"
echo     exit 0 >> "%BUILD_DIR%\verify-checksums.ps1"
echo } else { >> "%BUILD_DIR%\verify-checksums.ps1"
echo     Write-Host "✗ CHECKSUM VERIFICATION FAILED" -ForegroundColor Red >> "%BUILD_DIR%\verify-checksums.ps1"
echo     exit 1 >> "%BUILD_DIR%\verify-checksums.ps1"
echo } >> "%BUILD_DIR%\verify-checksums.ps1"

echo ========================================
echo Checksum Generation Complete
echo ========================================
echo.
echo Generated checksums for %CHECKSUM_COUNT% files
echo.
echo Files created:
echo - %CHECKSUM_FILE%
echo - %BUILD_DIR%\verify-checksums.bat
echo - %BUILD_DIR%\verify-checksums.ps1
echo.
echo To verify checksums:
echo   Windows: %BUILD_DIR%\verify-checksums.bat
echo   PowerShell: powershell -ExecutionPolicy Bypass -File %BUILD_DIR%\verify-checksums.ps1
echo   Linux/macOS: sha256sum -c %CHECKSUM_FILE%
echo.

REM Display checksums file content
echo Checksums file content:
echo =======================
type "%CHECKSUM_FILE%"

echo.
echo Checksum generation completed successfully!
pause