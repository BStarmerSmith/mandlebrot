// src/mandelbrot.rs

use rayon::prelude::*;
use rand::prelude::*;

// Maximum number of iterations to determine if a point is in the Mandelbrot set.
const MAX_ITERATIONS: u32 = 1500;

/// Check if the given complex number `c` is in the Mandelbrot set.
///
/// The Mandelbrot set is a set of complex numbers c for which the function `f(z) = z^2 + c`
/// does not diverge when iterated from `z = 0`. If the number of iterations reaches the
/// maximum limit (`MAX_ITERATIONS`) without the magnitude of `z` exceeding 2, the function
/// considers the number `c` to be in the Mandelbrot set and returns `true`.
///
/// # Arguments
///
/// * `c` - A tuple representing the complex number (real part, imaginary part).
///
/// # Returns
///
/// * `true` if the given complex number `c` is in the Mandelbrot set, otherwise `false`.
fn is_mandelbrot(c: (f64, f64)) -> bool {
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
///
/// This function takes an iteration count as input and calculates a rainbow color
/// based on the given count. The hue of the color is determined by dividing the
/// iteration count by the maximum limit (`MAX_ITERATIONS`). If the iteration count
/// equals the maximum limit, the color is black to represent points in the Mandelbrot set.
///
/// # Arguments
///
/// * `iterations` - The iteration count for a specific point in the Mandelbrot set.
///
/// # Returns
///
/// * A 32-bit unsigned integer representing the RGB color value.
fn rainbow_color(iterations: u32) -> u32 {
    if iterations == MAX_ITERATIONS {
        // If the point is in the Mandelbrot set, return black.
        return 0;
    }
    // Generate a rainbow color based on the iteration count.
    let hue = iterations as f64 / MAX_ITERATIONS as f64;
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
///
/// This function generates the Mandelbrot set by calculating the iteration count for each
/// pixel in the given `width` and `height` dimensions. The pixel buffer is then filled
/// with rainbow colors based on the iteration count for each pixel. If a pixel is part of
/// the Mandelbrot set (determined by `is_mandelbrot`), it is colored black.
///
/// # Arguments
///
/// * `width` - The width of the image in pixels.
/// * `height` - The height of the image in pixels.
///
/// # Returns
///
/// * A vector of 32-bit unsigned integers representing the RGB color values of each pixel.
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
        .map(|i| rainbow_color(i))
        .collect();

    // Loop through each pixel in the buffer using parallel processing.
    pixels.par_chunks_mut(width).enumerate().for_each(|(y, row)| {
        let cy = y as f64 * y_scale + y_offset;
        for (x, pixel) in row.iter_mut().enumerate() {
            let cx = x as f64 * x_scale + x_offset;
            if is_mandelbrot((cx, cy)) {
                *pixel = 0;
            } else {
                // Start with z = 0 + 0i
                let (mut zr, mut zi) = (0.0, 0.0);
                // Iterate to determine the iteration count.
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
        }
    });
    pixels
}

/// Render the Mandelbrot set into a pixel buffer with rainbow colors, with customizable parameters.
///
/// This function generates the Mandelbrot set using custom parameters: `width`, `height`, `center_x`,
/// `center_y`, and `zoom`. The pixel buffer is filled with rainbow colors based on the iteration count
/// for each pixel, and points within the Mandelbrot set are colored black.
///
/// # Arguments
///
/// * `width` - The width of the image in pixels.
/// * `height` - The height of the image in pixels.
/// * `center_x` - The x-coordinate of the center of the image.
/// * `center_y` - The y-coordinate of the center of the image.
/// * `zoom` - The zoom level of the Mandelbrot set.
///
/// # Returns
///
/// * A vector of 32-bit unsigned integers representing the RGB color values of each pixel.
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
        .map(|i| rainbow_color(i))
        .collect();

    // Loop through each pixel in the buffer using parallel processing.
    pixels.par_chunks_mut(width).enumerate().for_each(|(y, row)| {
        let cy = y as f64 * y_scale + y_offset;
        for (x, pixel) in row.iter_mut().enumerate() {
            let cx = x as f64 * x_scale + x_offset;
            if is_mandelbrot((cx, cy)) {
                *pixel = 0;
            } else {
                // Start with z = 0 + 0i
                let (mut zr, mut zi) = (0.0, 0.0);
                // Iterate to determine the iteration count.
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
        }
    });
    pixels
}