//! # test docs in spooky_colors/luminance.rs
//! 
//! ## spooky_colors/luminance.rs h2
//! 
//! qwertyuiop
//! asdfghjkl
//! zxcvbnm
//! 
//! ### spooky_colors/luminance.rs h3
//! 
//! - 10
//! - 31
//! - 06

use super::rgba::Rgba;

pub const LUM_R: f32 = 0.2126;
pub const LUM_G: f32 = 0.7152;
pub const LUM_B: f32 = 0.0722;

#[allow(dead_code)]
fn luminance(rgba: Rgba) -> f32 {
    let r_lum = rgba[0] as f32 * LUM_R;
    let g_lum = rgba[1] as f32 * LUM_G;
    let b_lum = rgba[2] as f32 * LUM_B;

    (r_lum + g_lum + b_lum).clamp(0.0, 255.0)
}

#[allow(dead_code)]
fn luminance_from_rgb(r: u8, g: u8, b: u8) -> f32 {
    let r_lum = r as f32 * LUM_R;
    let g_lum = g as f32 * LUM_G;
    let b_lum = b as f32 * LUM_B;

    (r_lum + g_lum + b_lum).clamp(0.0, 255.0)
}

#[allow(dead_code)]
fn luminance_norm(rgba: Rgba) -> f32 {
    luminance(rgba) / 255.0
}

#[allow(dead_code)]
fn luminance_norm_from_rgb(r: u8, g: u8, b: u8) -> f32 {
    luminance_from_rgb(r, g, b) / 255.0
}
