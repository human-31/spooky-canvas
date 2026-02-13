//! # test docs in spooky_canvas/raster/single_thread.rs
//! 
//! ## spooky_canvas/raster/single_thread.rs h2
//! 
//! qwertyuiop
//! asdfghjkl
//! zxcvbnm
//! 
//! ### spooky_canvas/raster/single_thread.rs h3
//! 
//! - 10
//! - 31
//! - 06

use super::super::canvas::Canvas;
use super::helpers::point_in_triangle;

/// Draws a circle on the given canvas with the specified center coordinates,
/// radius, and RGBA color.
/// 
/// Uses a simple nested for loop approach.
/// 
/// center = (x, y) \
/// rgba = [r, g, b, a]
pub fn circle_xy(
    canvas: &mut Canvas,
    center: (i32, i32),
    radius: u32,
    rgba: [u8; 4],
) {
    let (center_x, center_y) = center;

    let x_left = (center_x - 10 - radius as i32).max(0) as u32;
    let x_right = ((center_x + 10 + radius as i32) as u32).min(canvas.width());

    let y_top = (center_y - 10 - radius as i32).max(0) as u32;
    let y_bottom = ((center_y + 10 + radius as i32) as u32).min(canvas.height());

    
    for pixel_y in y_top..y_bottom {
        for pixel_x in x_left..x_right {
            let dx = pixel_x as i32 - center_x;
            let dy = pixel_y as i32 - center_y;

            if dx * dx + dy * dy > (radius as f32 + 0.5).powi(2) as i32 {
                continue;
            }
            
            let i = canvas.xy_to_index_unchecked(pixel_x, pixel_y);
            canvas.pixels[i..(i + 4)].copy_from_slice(&rgba);
        }
    }
}

/// Fills the entire canvas with the specified RGBA color
/// using a single-thread.
/// 
/// rgba = [r, g, b, a]
pub fn fill_all(
    canvas: &mut Canvas,
    rgba: [u8; 4]
) {
    for pixel in canvas.pixels.chunks_exact_mut(4) {
        pixel.copy_from_slice(&rgba);
    }
}

/// Draws a rectangle on the given canvas with the specified top-left corner
/// coordinates, width, height, and RGBA color.
/// 
/// Uses a simple nested for loop approach.
/// 
/// top_left = (x, y) \
/// size = (width, height) \
/// rgba = [r, g, b, a]
pub fn rect_xy(
    canvas: &mut Canvas,
    top_left: (i32, i32),
    size: (u32, u32),
    rgba: [u8; 4],
) {
    let (x, y) = top_left;
    let (width, height) = size;

    let x0 = x.max(0) as u32;
    let y0 = y.max(0) as u32;

    let x1 = ((x + width as i32) as u32).min(canvas.width());
    let y1 = ((y + height as i32) as u32).min(canvas.height());

    for py in y0..y1 {
        for px in x0..x1 {
            let i = canvas.xy_to_index_unchecked(px, py);
            canvas.pixels[i..(i + 4)].copy_from_slice(&rgba);
        }
    }
}

// pub fn rect_by_row(
//     canvas: &mut Canvas,
//     x: i32, y: i32,
//     width: u32, height: u32,
//     rgba: [u8; 4],
// ) {

// }

/// Draws a triangle on the given canvas defined by the three vertex
/// coordinates, using the specified RGBA color.
/// 
/// Uses a simple nested for loop approach.
/// 
/// v1 = (x1, y1)
/// v2 = (x2, y2)
/// v3 = (x3, y3)
/// rgba = [r, g, b, a]
pub fn triangle_xy(
    canvas: &mut Canvas,
    v1: (i32, i32),
    v2: (i32, i32),
    v3: (i32, i32),
    rgba: [u8; 4],
) {
    let (x1, y1) = v1;
    let (x2, y2) = v2;
    let (x3, y3) = v3;

    let x_left = x1.min(x2).min(x3).max(0) as u32;
    let x_right = (x1.max(x2).max(x3) as u32).min(canvas.width());

    let y_top = y1.min(y2).min(y3).max(0) as u32;
    let y_bottom = (y1.max(y2).max(y3) as u32).min(canvas.height());

    for py in y_top..y_bottom {
        for px in x_left..x_right {
            if  !point_in_triangle((px as i32, py as i32), v1, v2, v3) {
                continue;
            }

            let i = canvas.xy_to_index_unchecked(px, py);
            canvas.pixels[i..(i + 4)].copy_from_slice(&rgba);
        }
    }
}
