use super::super::canvas::Canvas;
use rayon::prelude::*;

pub fn fill_all(
    canvas: &mut Canvas,
    r: u8, g: u8, b: u8, a: u8,
) {
    let width = canvas.width() as usize;
    let stride = width * 4;

    canvas
        .pixels
        .par_chunks_mut(stride)
        .for_each(|row| {
            for px in row.chunks_mut(4) {
                px[0] = r;
                px[1] = g;
                px[2] = b;
                px[3] = a;
            }
        });
}