@echo off
title .hack//G.U. Vol. 1//Rebirth
cd /d "%~dp0"

echo ======================================================================
echo             .hack//G.U. Vol. 1//Rebirth (PC Native Recomp)
echo ======================================================================
echo.
echo Launching directly into the Main Menu...
echo Select [NEW GAME] to start the authentic prologue cutscene!
echo.

rem Build first so the game always includes the latest changes (a few seconds when nothing changed).
cargo build --release -q
if errorlevel 1 (
  echo.
  echo The build failed; starting the last built version.
  echo.
)
target\release\hackgu-render.exe
if errorlevel 1 (
  echo.
  echo The game stopped with an error. Details are in crash.log.
  pause
)
