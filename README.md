# Mandelbrot Set Renderer

This repository contains a simple Mandelbrot set renderer written in Rust. It allows you to visualize the Mandelbrot set and zoom into various regions.

## Requirements

- Rust (1.55.0 or higher)
- Cargo (Rust's package manager)
- Git

## Building and Running

### Windows

1. Install Rust: If you haven't installed Rust, download and install it from the official website: https://www.rust-lang.org/tools/install

2. Clone the repository: If you haven't already cloned the repository, open a terminal or Command Prompt and run the following command:
```
git clone https://github.com/BStarmerSmith/mandlebrot.git
cd mandelbrot-set
```

3. Build and run: To build and run the Mandelbrot set renderer, execute the following command:
```
cargo run --release
```

### Linux

1. Install Rust: If you haven't installed Rust, you can use Rust's official installation script. Open a terminal and run the following command:
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

Note: You might need to restart your terminal session after installing Rust.

2. Clone the repository: If you haven't already cloned the repository, open a terminal and run the following command:
```
git clone https://github.com/BStarmerSmith/mandlebrot.git
cd mandelbrot-set
```

3. Build and run: To build and run the Mandelbrot set renderer, execute the following command:
```
cargo run --release
```

## Controls

- Use arrow keys to navigate the view.
- Use the '+' and '-' keys to zoom in and out, respectively.
- Press 'Esc' to exit the program.
