@echo off
setlocal enabledelayedexpansion

:: Get script directory
set SCRIPT_DIR=%~dp0

:: Detect architecture
if "%PROCESSOR_ARCHITECTURE%"=="AMD64" (
  set ARCH=x64
) else if "%PROCESSOR_ARCHITECTURE%"=="ARM64" (
  set ARCH=arm64
) else if "%PROCESSOR_ARCHITEW6432%"=="AMD64" (
  set ARCH=x64
) else if "%PROCESSOR_ARCHITEW6432%"=="ARM64" (
  set ARCH=arm64
) else (
  echo Unsupported architecture: %PROCESSOR_ARCHITECTURE% 1>&2
  exit /b 1
)

:: Package name for this platform
set PACKAGE_NAME=csskit-win32-!ARCH!

:: Find binary in platform-specific optional dependency
set BIN_PATH=%SCRIPT_DIR%..\..\!PACKAGE_NAME!\bin\csskit.exe
if exist "!BIN_PATH!" (
  "!BIN_PATH!" %*
  exit /b %ERRORLEVEL%
)

:: Binary not found
echo Error: csskit binary not found for win32-!ARCH! 1>&2
echo Please ensure the appropriate platform package is installed: 1>&2
echo   npm install !PACKAGE_NAME! 1>&2
exit /b 1
