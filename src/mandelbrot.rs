// src/mandelbrot.rs

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

/// Render the Mandelbrot set into a pixel buffer.
pub fn render_mandelbrot(width: usize, height: usize) -> Vec<u32> {
    // Initialize a pixel buffer to store the colors of each pixel.
    let mut pixels = vec![0; width * height];

    // Set the offset and scaling factors for the Mandelbrot set coordinates.
    let x_offset = -2.5;
    let y_offset = -1.5;
    let x_scale = 3.5 / width as f64;
    let y_scale = 2.0 / height as f64;

    // Loop through each pixel in the buffer.
    for y in 0..height {
        for x in 0..width {
            // Map the pixel coordinates to the corresponding complex number `c`.
            let cx = x as f64 * x_scale + x_offset;
            let cy = y as f64 * y_scale + y_offset;

            // Check if `c` is in the Mandelbrot set and assign the pixel color accordingly.
            let color = if is_mandelbrot((cx, cy)) { 0xFFFFFF } else { 0 };
            pixels[y * width + x] = color;
        }
    }

    // Return the completed pixel buffer with Mandelbrot set colors.
    pixels
}

/// Render the Mandelbrot set into a pixel buffer with specified parameters for navigation and zooming.
pub fn render_mandelbrot_with_params(
    width: usize,
    height: usize,
    center_x: f64,
    center_y: f64,
    zoom: f64,
) -> Vec<u32> {
    let mut pixels = vec![0; width * height];

    // Calculate the offset and scaling factors based on the center coordinates and zoom level.
    let x_offset = center_x - 1.5 / zoom;
    let y_offset = center_y - 1.0 / zoom;
    let x_scale = 3.0 / (width as f64 * zoom);
    let y_scale = 2.0 / (height as f64 * zoom);

    for y in 0..height {
        for x in 0..width {
            let cx = x as f64 * x_scale + x_offset;
            let cy = y as f64 * y_scale + y_offset;
            let color = if is_mandelbrot((cx, cy)) { 0xFFFFFF } else { 0 };
            pixels[y * width + x] = color;
        }
    }

    pixels
}