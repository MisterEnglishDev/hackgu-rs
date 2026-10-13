@echo off
title .hack//G.U. - Asset Setup ^& Extractor
cd /d "%~dp0"

echo ======================================================================
echo         .hack//G.U. Clean-Room Asset Setup (Bring Your Own Game)
echo ======================================================================
echo.
echo TIP: You can drag and drop your Steam game folder, a .cpk archive,
echo      or a PS2 .iso directly onto this .bat file!
echo.

python --version >nul 2>&1
if errorlevel 1 (
    echo [ERROR] Python is not installed or not in your PATH.
    echo Please install Python 3.8+ to run the extraction script.
    pause
    exit /b 1
)

python tools\extract_assets.py %*
if errorlevel 1 (
    echo.
    echo Asset extraction encountered an error.
    pause
    exit /b 1
)

echo.
pause
