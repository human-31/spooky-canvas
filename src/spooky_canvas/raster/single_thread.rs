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
    r: u8, g: u8, b: u8, a: u8,
) {
    let rgba = [r, g, b, a];

    for pixel in canvas.pixels.chunks_exact_mut(4) {
        pixel.copy_from_slice(&rgba);
    }
}