# Mandelbrot Set Renderer

This repository contains a simple Mandelbrot set renderer written in Rust. It allows you to visualize the Mandelbrot set and zoom into various regions.

## What is the Mandelbrot Set?

The Mandelbrot set is a famous fractal discovered by mathematician Benoît B. Mandelbrot in 1980. It is a set of complex numbers that exhibit a unique and beautiful self-replicating pattern. The set is defined by iterating a simple formula on complex numbers and determining whether the iterated sequence remains bounded or escapes to infinity. Points that remain bounded are considered part of the Mandelbrot set, while those that escape are outside the set.

The Mandelbrot set is visualized by assigning colors to each point based on the number of iterations required for the point to escape or reach a predefined limit. This results in stunning visual representations of the fractal, exhibiting intricate patterns and shapes.

## The Mandelbrot Set Algorithm

The algorithm for generating the Mandelbrot set involves iterating a formula on complex numbers to determine if they belong to the set. Here's a high-level breakdown of the steps involved:

1. **Complex Number Representation**: The Mandelbrot set deals with complex numbers, which have both real and imaginary components. A complex number is represented as `(re, im)` where `re` is the real part and `im` is the imaginary part.

2. **Mandelbrot Iteration Formula**: The core of the algorithm is the Mandelbrot iteration formula. Given a complex number `c = (re, im)`, the formula iterates `z = z^2 + c`, where `z` starts at `(0, 0)`. The iteration continues until either the magnitude of `z` exceeds a predefined limit (commonly 2) or a maximum number of iterations is reached. If the magnitude exceeds the limit, the point is considered outside the set and escapes; otherwise, it is considered part of the set.

3. **Coloring the Points**: To visualize the Mandelbrot set, each point in the complex plane is assigned a color based on the number of iterations required for the point to escape or reach the maximum iterations. The color assignment can be customized to create stunning visual effects.

4. **Rendering the Set**: The algorithm generates a 2D representation of the Mandelbrot set by applying the iteration formula to a grid of complex numbers in the complex plane. Each point in the grid corresponds to a pixel in the output image.

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

- Use arrow keys or hold left click and move the mouse to navigate the view.
- Use the '+' and '-' keys or scrollwheel to zoom in and out, respectively.
- Right-click the mouse to reset back to the start position.
- Press 'Esc' to exit the program.

## File Descriptions

### [src/mandelbrot.rs](src/mandelbrot.rs)

This module contains the implementation of the Mandelbrot set renderer. It includes functions to render the Mandelbrot set into a pixel buffer with rainbow colors, handle navigation and zooming based on keyboard input, and handle mouse interaction for movement and zooming.

#### Functions

- `is_mandelbrot(c: (f64, f64)) -> bool`: Checks if the given complex number `c` is in the Mandelbrot set.
- `rainbow_color(iterations: u32) -> u32`: Helper function to generate a rainbow color based on the iteration count.
- `render_mandelbrot(width: usize, height: usize) -> Vec<u32>`: Renders the Mandelbrot set into a pixel buffer with rainbow colors.
- `render_mandelbrot_with_params(width: usize, height: usize, center_x: f64, center_y: f64, zoom: f64) -> Vec<u32>`: Renders the Mandelbrot set into a pixel buffer with rainbow colors, allowing for custom center coordinates and zoom level.

### [src/main.rs](src/main.rs)

This is the main entry point of the Mandelbrot set renderer application. It utilizes the functions from the `mandelbrot` module to render the Mandelbrot set and handle navigation and zooming based on user input. The application uses the minifb crate to create a window and display the rendered Mandelbrot set.

#### Functions

- `handle_navigation_and_zoom(window: &Window, center_x: &mut f64, center_y: &mut f64, zoom: &mut f64)`: Handles navigation and zooming based on keyboard input.
- `handle_mouse_interaction(window: &mut Window, center_x: &mut f64, center_y: &mut f64, zoom: &mut f64)`: Handles mouse interaction for movement and zooming.
- `main()`: The main function that initializes the window and runs the Mandelbrot set renderer application.
