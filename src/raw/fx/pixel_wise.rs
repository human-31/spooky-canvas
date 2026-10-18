use crate::raw::canvas::CanvasRaw;
use spooky_colors::{RgbaExt, RgbaRaw};

pub fn pixel_map<F>(canvas: &mut CanvasRaw, f: F)
where
    F: Fn(&RgbaRaw) -> RgbaRaw
{
    for px in canvas.pixels_mut() {
        let a = f(px);
        px.copy_from_slice(&a);
    }
}

pub fn par_pixel_map<F>(canvas: &mut CanvasRaw, f: F)
where
    F: Fn(&RgbaRaw) -> RgbaRaw
{
    todo!("raw::fx::par_pixel_map")
}

pub fn gray(raw: &RgbaRaw) -> RgbaRaw {
    let gray = raw.gray();
    [gray, gray, gray, raw.a()]
}

pub fn luma(raw: &RgbaRaw) -> RgbaRaw {
    let luma = raw.luma_byte();
    [luma, luma, luma, raw.a()]
}

////////////////////////////////////////////////////////////////////////////////
// below are use cases for the previous fns.
// i might delete these later, but this is like notes
// for how to use them.

pub fn pixel_gray(canvas: &mut CanvasRaw) {
    pixel_map(canvas, gray);
}

pub fn par_pixel_gray(canvas: &mut CanvasRaw) {
    par_pixel_map(canvas, gray);
}

pub fn pixel_luma(canvas: &mut CanvasRaw) {
    pixel_map(canvas, luma);
}

pub fn par_pixel_luma(canvas: &mut CanvasRaw) {
    par_pixel_map(canvas, luma);
}
