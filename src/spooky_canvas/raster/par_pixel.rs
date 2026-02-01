//! # test docs in spooky_canvas/raster/par_pixel.rs
//! 
//! ## spooky_canvas/raster/par_pixel.rs h2
//! 
//! qwertyuiop
//! asdfghjkl
//! zxcvbnm
//! 
//! ### spooky_canvas/raster/par_pixel.rs h3
//! 
//! - 10
//! - 31
//! - 06

use super::super::canvas::Canvas;
use rayon::prelude::*;

/// Fills the entire canvas with the specified RGBA color
/// using parallel pixel-wise processing.
pub fn fill_all(
    canvas: &mut Canvas,
    r: u8, g: u8, b: u8, a: u8,
) {
    canvas
        .pixels
        .par_chunks_mut(4)
        .for_each(|pixel| {
            pixel[0] = r;
            pixel[1] = g;
            pixel[2] = b;
            pixel[3] = a;
        });
}