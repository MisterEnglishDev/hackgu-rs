@echo off
title .hack//G.U. Vol. 1//Rebirth
cd /d "%~dp0"

echo ======================================================================
echo             .hack//G.U. Vol. 1//Rebirth (PC Native Recomp)
echo ======================================================================
echo.

rem The game reads its files from target\ (written by Extract_Game_Assets.bat).
if not exist "target\cmn\data\message\eng\messageInfo.bin" (
  echo The game files have not been set up yet.
  echo Run Extract_Game_Assets.bat first and point it at your .hack//G.U. Last Recode folder.
  echo.
  pause
  exit /b 1
)

where cargo >nul 2>&1
if errorlevel 1 (
  if not exist "target\release\hackgu-render.exe" (
    echo Rust is not installed ^(cargo was not found^). Install it from https://rustup.rs and run this again.
    echo.
    pause
    exit /b 1
  )
  goto launch
)

rem Build first so the game always includes the latest changes (a few seconds when nothing changed).
if not exist "target\release\hackgu-render.exe" (
  echo ######################################################################
  echo #                                                                    #
  echo #   FIRST LAUNCH: COMPILING THE GAME - PLEASE WAIT                   #
  echo #                                                                    #
  echo #   This takes about 5-15 minutes and your CPU will be at 100%%.      #
  echo #   Nothing is frozen: the progress bar below shows how far along    #
  echo #   it is. The game window opens by itself when it is done.          #
  echo #   Later launches skip this and start in seconds.                   #
  echo #                                                                    #
  echo ######################################################################
  echo.
)
cargo build --release
if errorlevel 1 (
  echo.
  if not exist "target\release\hackgu-render.exe" (
    echo The build failed. The messages above say why.
    pause
    exit /b 1
  )
  echo The build failed; starting the last built version.
  echo.
)

:launch
echo Starting the game...
target\release\hackgu-render.exe
if errorlevel 1 (
  echo.
  echo The game stopped with an error. Details are in crash.log.
  pause
)
