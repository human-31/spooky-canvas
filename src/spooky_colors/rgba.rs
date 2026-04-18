//! # test docs in spooky_colors/rgba.rs
//! 
//! ## spooky_colors/rgba.rs h2
//! 
//! qwertyuiop
//! asdfghjkl
//! zxcvbnm
//! 
//! ### spooky_colors/rgba.rs h3
//! 
//! - 10
//! - 31
//! - 06

pub type Rgba = [u8; 4];

/// Calculates average of the RGB components of the color, ignoring alpha.
#[allow(dead_code)]
pub fn average(rgba: Rgba) -> u8 {
    let r = rgba[0] as u16;
    let g = rgba[1] as u16;
    let b = rgba[2] as u16;

    ((r + g + b) / 3) as u8
}

/// Adds the RGBA values of two colors, clamping each component to [0, 255].
#[allow(dead_code)]
pub fn add(color1: Rgba, color2: Rgba) -> Rgba {
    let r = (color1[0] as u16 + color2[0] as u16).clamp(0, 255) as u8;
    let g = (color1[1] as u16 + color2[1] as u16).clamp(0, 255) as u8;
    let b = (color1[2] as u16 + color2[2] as u16).clamp(0, 255) as u8;
    let a = (color1[3] as u16 + color2[3] as u16).clamp(0, 255) as u8;

    [r, g, b, a]
}

/// Blends two RGBA colors using alpha compositing.
/// The source color is blended over the destination color.
/// This is the fast version, which could have (minimal) precision errors.
#[allow(dead_code)]
pub fn blend(src: Rgba, dst: Rgba) -> Rgba {
    let src_a = src[3] as u16;
    let inv_a = 255 - src_a;

    let r_u16 = (src[0] as u16 * src_a + dst[0] as u16 * inv_a + 128) >> 8;
    let g_u16 = (src[1] as u16 * src_a + dst[1] as u16 * inv_a + 128) >> 8;
    let b_u16 = (src[2] as u16 * src_a + dst[2] as u16 * inv_a + 128) >> 8;

    let dst_a = dst[3] as u16;
    let a_u16 = src_a + ((dst_a * inv_a + 128) >> 8);

    let r = r_u16 as u8;
    let g = g_u16 as u8;
    let b = b_u16 as u8;
    let a = a_u16 as u8;

    [r, g, b, a]
}

/// Blends two RGBA colors using alpha compositing with higher precision.
/// The source color is blended over the destination color.
/// This is the slow version, which should have better precision.
#[allow(dead_code)]
pub fn blend_precise(src: Rgba, dst: Rgba) -> Rgba {
    [0, 0, 0, 0] // delater
}
