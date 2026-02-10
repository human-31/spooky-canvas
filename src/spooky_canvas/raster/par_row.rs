//! # test docs in spooky_canvas/raster/par_row.rs
//! 
//! ## spooky_canvas/raster/par_row.rs h2
//! 
//! qwertyuiop
//! asdfghjkl
//! zxcvbnm
//! 
//! ### spooky_canvas/raster/par_row.rs h3
//! 
//! - 10
//! - 31
//! - 06

use super::super::canvas::Canvas;
use rayon::prelude::*;

/// Fills the entire canvas with the specified RGBA color
/// using parallel row processing.
pub fn fill_all(
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