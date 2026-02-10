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

/// Fills the entire canvas with the specified RGBA color
/// using a single-thread.
pub fn fill_all(
    canvas: &mut Canvas,
    rgba: [u8; 4]
) {
    for pixel in canvas.pixels.chunks_exact_mut(4) {
        pixel.copy_from_slice(&rgba);
    }
}

pub fn rect_xy(
    canvas: &mut Canvas,
    x: i32, y: i32,
    width: u32, height: u32,
    rgba: [u8; 4],
) {
    let x0 = x.max(0) as u32;
    let y0 = y.max(0) as u32;

    let x1 = ((x + width as i32) as u32).min(canvas.width() - 1);
    let y1 = ((y + height as i32) as u32).min(canvas.height() - 1);

    for py in y0..y1 {
        for px in x0..x1 {
            let i = canvas.xy_to_index_unchecked(px, py);
            canvas.pixels[i..(i + 4)].copy_from_slice(&rgba);
        }
    }
}

pub fn rect_by_row(
    canvas: &mut Canvas,
    x: i32, y: i32,
    width: u32, height: u32,
    rgba: [u8; 4],
) {

}

pub fn triangle_xy(
    canvas: &mut Canvas,
    x1: i32, y1: i32,
    x2: i32, y2: i32,
    x3: i32, y3: i32,
    rgba: [u8; 4],
) {

}
