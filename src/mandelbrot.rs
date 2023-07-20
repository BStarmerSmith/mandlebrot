// src/mandelbrot.rs

use rayon::prelude::*;
use rand::prelude::*;

/// Check if the given complex number `c` is in the Mandelbrot set.
fn is_mandelbrot(c: (f64, f64)) -> bool {
    // Maximum number of iterations to determine if `c` is in the set.
    const MAX_ITERATIONS: u32 = 1000;
    // Start with z = 0 + 0i
    let (mut zr, mut zi) = (0.0, 0.0);
    // Iterate to determine if `c` is in the set.
    for i in 0..MAX_ITERATIONS {
        // Mandelbrot iteration formula: z = z^2 + c
        let zr_new = zr * zr - zi * zi + c.0;
        let zi_new = 2.0 * zr * zi + c.1;
        // Update the real and imaginary parts with the new values.
        zr = zr_new;
        zi = zi_new;
        // If the magnitude of z exceeds 2, `c` is not in the Mandelbrot set.
        if zr * zr + zi * zi > 4.0 {
            return false;
        }
    }
    // If the loop completes, `c` is considered in the Mandelbrot set.
    true
}

/// Helper function to generate a rainbow color based on the iteration count.
fn rainbow_color(iterations: u32, max_iterations: u32) -> u32 {
    if iterations == max_iterations {
        // If the point is in the Mandelbrot set, return black.
        return 0;
    }

    // Generate a rainbow color based on the iteration count.
    let hue = iterations as f64 / max_iterations as f64;
    let saturation = 1.0;
    let value = 1.0;

    let h = (6.0 * hue).floor();
    let f = hue * 6.0 - h;
    let p = value * (1.0 - saturation);
    let q = value * (1.0 - f * saturation);
    let t = value * (1.0 - (1.0 - f) * saturation);

    let (r, g, b) = match h as i32 {
        0 => (value, t, p),
        1 => (q, value, p),
        2 => (p, value, t),
        3 => (p, q, value),
        4 => (t, p, value),
        _ => (value, p, q),
    };

    ((r * 255.0) as u32) << 16 | ((g * 255.0) as u32) << 8 | ((b * 255.0) as u32)
}

/// Render the Mandelbrot set into a pixel buffer with rainbow colors.
pub fn render_mandelbrot(width: usize, height: usize) -> Vec<u32> {
    // Initialize a pixel buffer to store the colors of each pixel.
    let mut pixels = vec![0; width * height];
    // Set the offset and scaling factors for the Mandelbrot set coordinates.
    let x_offset = -2.5;
    let y_offset = -1.5;
    let x_scale = 3.5 / width as f64;
    let y_scale = 2.0 / height as f64;

    // Maximum number of iterations to determine if a point is in the Mandelbrot set.
    const MAX_ITERATIONS: u32 = 1000;
    // Precalculate colors for all iterations
    let colors: Vec<u32> = (0..=MAX_ITERATIONS)
        .map(|i| rainbow_color(i, MAX_ITERATIONS))
        .collect();

    // Loop through each pixel in the buffer using parallel processing.
    pixels.par_chunks_mut(width).enumerate().for_each(|(y, row)| {
        let cy = y as f64 * y_scale + y_offset;
        for (x, pixel) in row.iter_mut().enumerate() {
            let cx = x as f64 * x_scale + x_offset;
            // Start with z = 0 + 0i
            let (mut zr, mut zi) = (0.0, 0.0);
            // Iterate to determine if `c` is in the set and calculate the iteration count.
            let mut i = 0;
            while i < MAX_ITERATIONS && zr * zr + zi * zi <= 4.0 {
                // Mandelbrot iteration formula: z = z^2 + c
                let zr_new = zr * zr - zi * zi + cx;
                let zi_new = 2.0 * zr * zi + cy;
                // Update the real and imaginary parts with the new values.
                zr = zr_new;
                zi = zi_new;
                i += 1;
            }
            // Get the rainbow color for the current pixel based on the iteration count.
            *pixel = colors[i as usize];
        }
    });
    // Return the completed pixel buffer with rainbow colors.
    pixels
}

pub fn render_mandelbrot_with_params(
    width: usize,
    height: usize,
    center_x: f64,
    center_y: f64,
    zoom: f64,
) -> Vec<u32> {
    // Initialize a pixel buffer to store the colors of each pixel.
    let mut pixels = vec![0; width * height];
    // Calculate the offset and scaling factors based on the center coordinates and zoom level.
    let x_offset = center_x - 1.5 / zoom;
    let y_offset = center_y - 1.0 / zoom;
    let x_scale = 3.0 / (width as f64 * zoom);
    let y_scale = 2.0 / (height as f64 * zoom);

    // Maximum number of iterations to determine if a point is in the Mandelbrot set.
    const MAX_ITERATIONS: u32 = 1000;
    // Precalculate colors for all iterations
    let colors: Vec<u32> = (0..=MAX_ITERATIONS)
        .map(|i| rainbow_color(i, MAX_ITERATIONS))
        .collect();

    // Loop through each pixel in the buffer using parallel processing.
    pixels.par_chunks_mut(width).enumerate().for_each(|(y, row)| {
        let cy = y as f64 * y_scale + y_offset;
        for (x, pixel) in row.iter_mut().enumerate() {
            let cx = x as f64 * x_scale + x_offset;
            // Start with z = 0 + 0i
            let (mut zr, mut zi) = (0.0, 0.0);
            // Iterate to determine if `c` is in the set and calculate the iteration count.
            let mut i = 0;
            while i < MAX_ITERATIONS && zr * zr + zi * zi <= 4.0 {
                // Mandelbrot iteration formula: z = z^2 + c
                let zr_new = zr * zr - zi * zi + cx;
                let zi_new = 2.0 * zr * zi + cy;
                // Update the real and imaginary parts with the new values.
                zr = zr_new;
                zi = zi_new;
                i += 1;
            }
            // Get the rainbow color for the current pixel based on the iteration count.
            *pixel = colors[i as usize];
        }
    });
    // Return the completed pixel buffer with rainbow colors.
    pixels
}
