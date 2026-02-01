use super::super::canvas::Canvas;

pub fn fill_all(
    canvas: &mut Canvas,
    r: u8, g: u8, b: u8, a: u8,
) {
    let rgba = [r, g, b, a];

    for pixel in canvas.pixels.chunks_exact_mut(4) {
        pixel.copy_from_slice(&rgba);
    }
}