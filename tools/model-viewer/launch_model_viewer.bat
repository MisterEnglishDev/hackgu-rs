@echo off
title .hack//G.U. 3D Model Viewer & Studio
cd /d "%~dp0\..\.."

echo ======================================================================
echo           .hack//G.U. Model Viewer & Inspection Studio
echo ======================================================================
echo.
echo Drag and drop any .ccs / .css model onto this window to inspect,
echo view animations, equip weapons, or export to GLB.
echo.

if "%~1"=="" (
    target\release\hackgu-render.exe VRChat\work\haseo\raw\2hsw01.ccs
) else (
    target\release\hackgu-render.exe "%~1"
)
