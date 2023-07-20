#!/bin/bash

# Build the Rust project using cargo
cargo build --release

# Check if the build was successful
if [ $? -ne 0 ]; then
    echo "Build failed. Exiting..."
    exit 1
fi

# Run the compiled program
./target/release/mandelbrot_set

read -n 1 -s -r -p "Press any key to exit..."
