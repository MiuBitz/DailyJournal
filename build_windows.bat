@echo off
setlocal
cd /d "%~dp0"

echo ==========================================
echo   Daily Journal (Rust) - Windows Build
echo ==========================================
echo.

echo Checking Rust build toolchain...
cargo --version
if errorlevel 1 (
    echo Error: Cargo/Rust is not installed or not in PATH.
    pause
    exit /b 1
)

echo.
echo Compiling optimized release binary...
set "CARGO_TARGET_DIR=%TEMP%\cargo_target_daily_journal"
cargo build --release
if errorlevel 1 (
    echo Error: Compilation failed.
    pause
    exit /b 1
)

if not exist "dist" mkdir dist
copy /y "%CARGO_TARGET_DIR%\release\daily_journal.exe" "dist\DailyJournal.exe" > nul

echo.
echo ==========================================
echo Build complete!
echo ==========================================
echo.
echo High-performance standalone executable created:
echo %CD%\dist\DailyJournal.exe
echo.
pause
