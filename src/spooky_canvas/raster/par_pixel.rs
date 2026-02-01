use super::super::canvas::Canvas;
use rayon::prelude::*;

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