@echo off
echo Testing TriageIR GUI - New Clean Version
echo ========================================

echo.
echo Installing dependencies...
call npm install

echo.
echo Starting GUI in development mode...
echo Press Ctrl+C to stop the application
echo.
call npm run dev