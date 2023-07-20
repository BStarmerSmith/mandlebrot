@echo off

REM Build the Rust project using cargo
cargo build --release

REM Check if the build was successful
if %errorlevel% neq 0 (
    echo Build failed. Exiting...
    pause
    exit /b
)

REM Run the compiled program
target\release\mandelbrot_set.exe

pause
