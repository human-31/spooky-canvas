//! # test docs in spooky_canvas/raster/multi_thread.rs
//! 
//! ## spooky_canvas/raster/multi_thread.rs h2
//! 
//! qwertyuiop
//! asdfghjkl
//! zxcvbnm
//! 
//! ### spooky_canvas/raster/multi_thread.rs h3
//! 
//! - 10
//! - 31
//! - 06

use super::super::canvas::Canvas;
use rayon::prelude::*;

/// Fills the entire canvas with the specified RGBA color
/// using parallel pixel-wise processing.
pub fn fill_all_par_pixel(
    canvas: &mut Canvas,
    rgba: [u8; 4],
) {
    canvas.pixels
        .par_chunks_mut(4)
        .for_each(
            |pixel| { pixel.copy_from_slice(&rgba); }
        );
}

/// Fills the entire canvas with the specified RGBA color
/// using parallel row processing.
pub fn fill_all_par_row(
    canvas: &mut Canvas,
    rgba: [u8; 4],
) {
    let width = canvas.width() as usize;
    let stride = width * 4;

    canvas.pixels
        .par_chunks_mut(stride)
        .for_each(
            |row| {
                for pixel in row.chunks_mut(4) {
                    pixel.copy_from_slice(&rgba);
                }
            }  
        );
}