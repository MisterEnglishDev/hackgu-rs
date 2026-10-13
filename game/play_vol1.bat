@echo off
title .hack//G.U. Vol. 1//Rebirth
cd /d "%~dp0\.."

echo ======================================================================
echo             .hack//G.U. Vol. 1//Rebirth (PC Native Recomp)
echo ======================================================================
echo.
echo Launching directly into the Main Menu...
echo Select [NEW GAME] to play the authentic opening cutscene and prologue!
echo.

target\release\hackgu-render.exe
