// src/main.rs

mod mandelbrot;

use minifb::{Key, Window, WindowOptions};
use std::time::{Duration, Instant};
use rayon::prelude::*; // Import the Rayon prelude
use rayon::ThreadPoolBuilder;

// Function to handle navigation and zooming
fn handle_navigation_and_zoom(window: &Window, center_x: &mut f64, center_y: &mut f64, zoom: &mut f64) {
    const ZOOM_FACTOR: f64 = 1.1;
    const NAVIGATION_STEP: f64 = 0.2;
    // Handling arrow keys for navigation
    if window.is_key_down(Key::Left) {
        *center_x -= NAVIGATION_STEP / *zoom;
    }
    if window.is_key_down(Key::Right) {
        *center_x += NAVIGATION_STEP / *zoom;
    }
    if window.is_key_down(Key::Up) {
        *center_y -= NAVIGATION_STEP / *zoom;
    }
    if window.is_key_down(Key::Down) {
        *center_y += NAVIGATION_STEP / *zoom;
    }
    // Handling '+' and '-' keys for zooming
    if window.is_key_down(Key::Equal) || window.is_key_down(Key::Period) {
        *zoom /= ZOOM_FACTOR;
    }
    if window.is_key_down(Key::Minus) || window.is_key_down(Key::Comma) {
        *zoom *= ZOOM_FACTOR;
    }
}

fn main() {
    const WIDTH: usize = 800;
    const HEIGHT: usize = 600;
    const MAX_FPS: u64 = 60; // Cap the frame rate at 60 FPS
    let mut buffer: Vec<u32> = mandelbrot::render_mandelbrot(WIDTH, HEIGHT);

    let mut window = Window::new(
        "Mandelbrot Set",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| panic!("{}", e));

    let mut center_x: f64 = -0.5;
    let mut center_y: f64 = 0.0;
    let mut zoom: f64 = 1.0;

    let mut last_frame_time = Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Calculate the time taken to render the previous frame.
        let elapsed_time = last_frame_time.elapsed();
        let frame_duration = Duration::from_secs_f64(1.0 / MAX_FPS as f64);
        // If the frame rendering took less time than the target frame duration, sleep for the remaining time.
        if elapsed_time < frame_duration {
            std::thread::sleep(frame_duration - elapsed_time);
        }
        // Update the last frame time for the next iteration.
        last_frame_time = Instant::now();
        // Handling navigation and zooming
        handle_navigation_and_zoom(&window, &mut center_x, &mut center_y, &mut zoom);
        // Rendering the Mandelbrot set with the updated parameters
        buffer = mandelbrot::render_mandelbrot_with_params(WIDTH, HEIGHT, center_x, center_y, zoom);
        // Updating the window display with the rendered buffer
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap_or_else(|e| panic!("{}", e));
    }
}

