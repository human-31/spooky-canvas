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
    rgba: [u8; 4],
) {
    canvas.pixels
        .par_chunks_mut(4)
        .for_each(
            |pixel| { pixel.copy_from_slice(&rgba); }
        );
}