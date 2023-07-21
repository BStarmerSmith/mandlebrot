// src/main.rs

mod mandelbrot;

use minifb::{Key, MouseButton, Window, WindowOptions, MouseMode};
use std::time::{Duration, Instant};
use rayon::prelude::*; // Import the Rayon prelude
use rayon::ThreadPoolBuilder;

/// Function to handle navigation and zooming based on keyboard input.
///
/// This function takes the current window, center coordinates, and zoom level as input and
/// handles navigation and zooming based on the arrow keys for movement and '+'/'-' keys for zooming.
/// The function updates the center coordinates and zoom level accordingly.
///
/// # Arguments
///
/// * `window` - A reference to the minifb window.
/// * `center_x` - A mutable reference to the x-coordinate of the center of the Mandelbrot set.
/// * `center_y` - A mutable reference to the y-coordinate of the center of the Mandelbrot set.
/// * `zoom` - A mutable reference to the zoom level of the Mandelbrot set.
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

/// Function to handle mouse interaction for movement and zooming.
///
/// This function takes the current window, center coordinates, and zoom level as input and
/// handles mouse interaction for movement and zooming. It calculates the mouse movement distance,
/// scales it based on the zoom level, and adjusts the center coordinates accordingly. It also
/// adjusts the zoom level based on mouse scroll wheel movement.
///
/// # Arguments
///
/// * `window` - A mutable reference to the minifb window.
/// * `center_x` - A mutable reference to the x-coordinate of the center of the Mandelbrot set.
/// * `center_y` - A mutable reference to the y-coordinate of the center of the Mandelbrot set.
/// * `zoom` - A mutable reference to the zoom level of the Mandelbrot set.
fn handle_mouse_interaction(
    window: &mut Window,
    center_x: &mut f64,
    center_y: &mut f64,
    zoom: &mut f64,
) {
    const ZOOM_FACTOR: f64 = 1.2; // Adjust the zoom factor for smooth zooming
    const BASE_SENSITIVITY: f64 = 0.00005; // Adjust the base mouse movement sensitivity
    // Get mouse events
    let (mouse_x, mouse_y) = window.get_mouse_pos(MouseMode::Clamp).unwrap();
    let (window_width, window_height) = window.get_size();

    if window.get_mouse_down(MouseButton::Left) {
        // Calculate the base sensitivity based on the current zoom level
        let base_sensitivity = BASE_SENSITIVITY / *zoom;
        // Scale the sensitivity based on the zoom level
        let sensitivity = base_sensitivity * *zoom;

        // Calculate the movement distance based on mouse position and sensitivity
        let dx = (mouse_x as f64 - window_width as f64 / 2.0) * sensitivity;
        let dy = (mouse_y as f64 - window_height as f64 / 2.0) * sensitivity;
        // Calculate the maximum allowed movement distance based on the current zoom level and window size
        let max_dx = window_width as f64 / (2.0 * *zoom);
        let max_dy = window_height as f64 / (2.0 * *zoom);
        // Adjust the center coordinates based on mouse movement, clamping within bounds
        *center_x += dx.clamp(-max_dx, max_dx);
        *center_y += dy.clamp(-max_dy, max_dy);
    }

    if let Some((wheel_delta_x, wheel_delta_y)) = window.get_scroll_wheel() {
        if wheel_delta_y != 0.0 {
            // Adjust the zoom level based on the direction of the wheel
            if wheel_delta_y > 0.0 {
                *zoom *= ZOOM_FACTOR;
            } else {
                *zoom /= ZOOM_FACTOR;
            }
        }
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

    let mut initial_center_x: f64 = -0.5;
    let mut initial_center_y: f64 = 0.0;
    let mut initial_zoom: f64 = 1.0;
    // Save the initial state
    let mut center_x = initial_center_x;
    let mut center_y = initial_center_y;
    let mut zoom = initial_zoom;

    // Initialize Rayon only if it's not already initialized
    if !ThreadPoolBuilder::default().build_global().is_ok() {
        println!("Rayon global thread pool already initialized.");
    }
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
        // Handling mouse interaction for movement and zooming
        handle_mouse_interaction(&mut window, &mut center_x, &mut center_y, &mut zoom);
        // Reset to initial settings if right mouse button is pressed
        if window.get_mouse_down(MouseButton::Right) {
            center_x = initial_center_x;
            center_y = initial_center_y;
            zoom = initial_zoom;
        }
        // Rendering the Mandelbrot set with the updated parameters
        buffer = mandelbrot::render_mandelbrot_with_params(WIDTH, HEIGHT, center_x, center_y, zoom);
        // Updating the window display with the rendered buffer
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap_or_else(|e| panic!("{}", e));
    }
}

