@echo off
REM TriageIR Code Signing Script
REM Signs all executables with digital certificate for security and trust

setlocal enabledelayedexpansion

echo ========================================
echo TriageIR Code Signing
echo ========================================
echo.

REM Configuration
set CERT_FILE=%1
set CERT_PASSWORD=%2
set TIMESTAMP_URL=http://timestamp.digicert.com
set BUILD_DIR=build

if "%CERT_FILE%"=="" (
    echo Usage: sign-executables.bat [certificate-file] [password]
    echo Example: sign-executables.bat mycert.pfx mypassword
    echo.
    echo Note: For production use, store certificate password securely
    echo       and consider using certificate stores instead of files
    exit /b 1
)

if not exist "%CERT_FILE%" (
    echo ERROR: Certificate file not found: %CERT_FILE%
    exit /b 1
)

REM Check for signtool
where signtool >nul 2>&1
if %ERRORLEVEL% neq 0 (
    echo ERROR: signtool not found!
    echo Please install Windows SDK or Visual Studio
    echo Signtool is typically located in:
    echo   C:\Program Files (x86)\Windows Kits\10\bin\x64\signtool.exe
    echo.
    echo Add to PATH or run from Visual Studio Developer Command Prompt
    exit /b 1
)

echo ✓ signtool found
echo Certificate: %CERT_FILE%
echo Timestamp URL: %TIMESTAMP_URL%
echo.

set SIGNED_COUNT=0
set FAILED_COUNT=0

REM Sign CLI executable
echo [1/4] Signing CLI executable...
if exist "%BUILD_DIR%\TriageIR-v1.0.0\CLI\triageir-cli.exe" (
    signtool sign /f "%CERT_FILE%" /p "%CERT_PASSWORD%" /t "%TIMESTAMP_URL%" /d "TriageIR CLI" /du "https://github.com/triageir/triageir" "%BUILD_DIR%\TriageIR-v1.0.0\CLI\triageir-cli.exe"
    if !ERRORLEVEL! equ 0 (
        echo   ✓ CLI executable signed successfully
        set /a SIGNED_COUNT+=1
    ) else (
        echo   ✗ CLI executable signing failed
        set /a FAILED_COUNT+=1
    )
) else (
    echo   ✗ CLI executable not found
    set /a FAILED_COUNT+=1
)

REM Sign GUI executable
echo.
echo [2/4] Signing GUI executable...
if exist "%BUILD_DIR%\TriageIR-v1.0.0\GUI\TriageIR.exe" (
    signtool sign /f "%CERT_FILE%" /p "%CERT_PASSWORD%" /t "%TIMESTAMP_URL%" /d "TriageIR GUI" /du "https://github.com/triageir/triageir" "%BUILD_DIR%\TriageIR-v1.0.0\GUI\TriageIR.exe"
    if !ERRORLEVEL! equ 0 (
        echo   ✓ GUI executable signed successfully
        set /a SIGNED_COUNT+=1
    ) else (
        echo   ✗ GUI executable signing failed
        set /a FAILED_COUNT+=1
    )
) else (
    echo   ✗ GUI executable not found
    set /a FAILED_COUNT+=1
)

REM Sign installer
echo.
echo [3/4] Signing installer...
if exist "%BUILD_DIR%\TriageIR-v1.0.0-Setup.exe" (
    signtool sign /f "%CERT_FILE%" /p "%CERT_PASSWORD%" /t "%TIMESTAMP_URL%" /d "TriageIR Setup" /du "https://github.com/triageir/triageir" "%BUILD_DIR%\TriageIR-v1.0.0-Setup.exe"
    if !ERRORLEVEL! equ 0 (
        echo   ✓ Installer signed successfully
        set /a SIGNED_COUNT+=1
    ) else (
        echo   ✗ Installer signing failed
        set /a FAILED_COUNT+=1
    )
) else (
    echo   ✗ Installer not found
    set /a FAILED_COUNT+=1
)

REM Sign any additional executables
echo.
echo [4/4] Signing additional executables...
set ADDITIONAL_SIGNED=0

for /r "%BUILD_DIR%" %%F in (*.exe) do (
    set "FILEPATH=%%F"
    set "FILENAME=%%~nxF"
    
    REM Skip already signed files
    if not "!FILENAME!"=="triageir-cli.exe" (
        if not "!FILENAME!"=="TriageIR.exe" (
            if not "!FILENAME!"=="TriageIR-v1.0.0-Setup.exe" (
                echo   Signing: !FILENAME!
                signtool sign /f "%CERT_FILE%" /p "%CERT_PASSWORD%" /t "%TIMESTAMP_URL%" /d "TriageIR Component" /du "https://github.com/triageir/triageir" "!FILEPATH!"
                if !ERRORLEVEL! equ 0 (
                    echo     ✓ Signed successfully
                    set /a SIGNED_COUNT+=1
                    set /a ADDITIONAL_SIGNED+=1
                ) else (
                    echo     ✗ Signing failed
                    set /a FAILED_COUNT+=1
                )
            )
        )
    )
)

if %ADDITIONAL_SIGNED% equ 0 (
    echo   No additional executables found
)

echo.
echo ========================================
echo Code Signing Results
echo ========================================
echo.
echo Successfully signed: %SIGNED_COUNT% files
echo Failed to sign: %FAILED_COUNT% files
echo.

REM Verify signatures
echo Verifying signatures...
echo.

if exist "%BUILD_DIR%\TriageIR-v1.0.0\CLI\triageir-cli.exe" (
    echo CLI Executable:
    signtool verify /pa "%BUILD_DIR%\TriageIR-v1.0.0\CLI\triageir-cli.exe"
    echo.
)

if exist "%BUILD_DIR%\TriageIR-v1.0.0\GUI\TriageIR.exe" (
    echo GUI Executable:
    signtool verify /pa "%BUILD_DIR%\TriageIR-v1.0.0\GUI\TriageIR.exe"
    echo.
)

if exist "%BUILD_DIR%\TriageIR-v1.0.0-Setup.exe" (
    echo Installer:
    signtool verify /pa "%BUILD_DIR%\TriageIR-v1.0.0-Setup.exe"
    echo.
)

REM Create signature verification script
echo Creating signature verification script...
echo @echo off > "%BUILD_DIR%\verify-signatures.bat"
echo REM TriageIR Signature Verification Script >> "%BUILD_DIR%\verify-signatures.bat"
echo. >> "%BUILD_DIR%\verify-signatures.bat"
echo echo Verifying TriageIR digital signatures... >> "%BUILD_DIR%\verify-signatures.bat"
echo echo. >> "%BUILD_DIR%\verify-signatures.bat"
echo. >> "%BUILD_DIR%\verify-signatures.bat"
echo set VERIFIED=0 >> "%BUILD_DIR%\verify-signatures.bat"
echo set FAILED=0 >> "%BUILD_DIR%\verify-signatures.bat"
echo. >> "%BUILD_DIR%\verify-signatures.bat"

REM Add verification for each signed file
for /r "%BUILD_DIR%" %%F in (*.exe) do (
    set "FILEPATH=%%F"
    set "RELPATH=!FILEPATH:%BUILD_DIR%\=!"
    
    echo echo Verifying: !RELPATH! >> "%BUILD_DIR%\verify-signatures.bat"
    echo signtool verify /pa "!RELPATH!" ^>nul 2^>^&1 >> "%BUILD_DIR%\verify-signatures.bat"
    echo if %%ERRORLEVEL%% equ 0 ^( >> "%BUILD_DIR%\verify-signatures.bat"
    echo     echo   ✓ Valid signature >> "%BUILD_DIR%\verify-signatures.bat"
    echo     set /a VERIFIED+=1 >> "%BUILD_DIR%\verify-signatures.bat"
    echo ^) else ^( >> "%BUILD_DIR%\verify-signatures.bat"
    echo     echo   ✗ Invalid or missing signature >> "%BUILD_DIR%\verify-signatures.bat"
    echo     set /a FAILED+=1 >> "%BUILD_DIR%\verify-signatures.bat"
    echo ^) >> "%BUILD_DIR%\verify-signatures.bat"
)

echo. >> "%BUILD_DIR%\verify-signatures.bat"
echo echo. >> "%BUILD_DIR%\verify-signatures.bat"
echo echo Signature Verification Results: >> "%BUILD_DIR%\verify-signatures.bat"
echo echo =============================== >> "%BUILD_DIR%\verify-signatures.bat"
echo echo Verified: %%VERIFIED%% >> "%BUILD_DIR%\verify-signatures.bat"
echo echo Failed: %%FAILED%% >> "%BUILD_DIR%\verify-signatures.bat"
echo. >> "%BUILD_DIR%\verify-signatures.bat"
echo if %%FAILED%% equ 0 ^( >> "%BUILD_DIR%\verify-signatures.bat"
echo     echo ✓ ALL SIGNATURES VERIFIED >> "%BUILD_DIR%\verify-signatures.bat"
echo     exit /b 0 >> "%BUILD_DIR%\verify-signatures.bat"
echo ^) else ^( >> "%BUILD_DIR%\verify-signatures.bat"
echo     echo ✗ SIGNATURE VERIFICATION FAILED >> "%BUILD_DIR%\verify-signatures.bat"
echo     exit /b 1 >> "%BUILD_DIR%\verify-signatures.bat"
echo ^) >> "%BUILD_DIR%\verify-signatures.bat"

if %FAILED_COUNT% equ 0 (
    echo ✓ ALL FILES SIGNED SUCCESSFULLY
    echo.
    echo Code signing completed successfully!
    echo Verification script created: %BUILD_DIR%\verify-signatures.bat
    echo.
    echo Security Notes:
    echo - All executables are now digitally signed
    echo - Users will see verified publisher information
    echo - Windows SmartScreen warnings should be reduced
    echo - Signatures include timestamp for long-term validity
    echo.
    exit /b 0
) else (
    echo ✗ CODE SIGNING FAILED
    echo %FAILED_COUNT% file(s) failed to sign
    echo.
    echo Common Issues:
    echo - Certificate file not found or invalid
    echo - Incorrect password
    echo - Certificate expired or not trusted
    echo - Network issues with timestamp server
    echo - Insufficient permissions
    echo.
    exit /b 1
)

REM Security cleanup - clear password from memory
set CERT_PASSWORD=