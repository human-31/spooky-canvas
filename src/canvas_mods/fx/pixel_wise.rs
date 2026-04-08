//! # test docs in canvas_mods/fx/pixel_wise.rs
//! 
//! ## canvas_mods/fx/pixel_wise.rs h2
//! 
//! qwertyuiop
//! asdfghjkl
//! zxcvbnm
//! 
//! ### canvas_mods/fx/pixel_wise.rs h3
//! 
//! - 10
//! - 31
//! - 06

use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

use crate::spooky_canvas::Canvas;
use super::constants::*;

#[allow(dead_code)]
pub fn grayscale_avg_single(canvas: &mut Canvas) {
    for pixel in canvas.pixels.chunks_exact_mut(4) {
        let rgb_sum = pixel[0] as f32 + pixel[1] as f32 + pixel[2] as f32;
        let avg = (rgb_sum / 3.0).clamp(0.0, 255.0) as u8;

        pixel.copy_from_slice(&[avg, avg, avg, pixel[3]]);
    }
}

#[allow(dead_code)]
pub fn grayscale_lum_single(canvas: &mut Canvas) {
    for pixel in canvas.pixels.chunks_exact_mut(4) {
        let r_lum = pixel[0] as f32 * LUM_R;
        let g_lum = pixel[1] as f32 * LUM_G;
        let b_lum = pixel[2] as f32 * LUM_B;

        let avg = (r_lum + g_lum + b_lum).clamp(0.0, 255.0) as u8;

        pixel.copy_from_slice(&[avg, avg, avg, pixel[3]]);
    }
}

#[allow(dead_code)]
pub fn rgb_noise_int_single(canvas: &mut Canvas, radius: i16, seed: u64) {
    let mut rng = StdRng::seed_from_u64(seed);

    for pixel in canvas.pixels.chunks_exact_mut(4) {
        let r_noise = rng.random_range(-radius..=radius) as i16;
        let g_noise = rng.random_range(-radius..=radius) as i16;
        let b_noise = rng.random_range(-radius..=radius) as i16;

        let r = (pixel[0] as i16 + r_noise).clamp(0, 255) as u8;
        let g = (pixel[1] as i16 + g_noise).clamp(0, 255) as u8;
        let b = (pixel[2] as i16 + b_noise).clamp(0, 255) as u8;
        let a = pixel[3];

        pixel.copy_from_slice(&[r, g, b, a]);
    }
}

#[allow(dead_code)]
pub fn rgb_noise_float_single(canvas: &mut Canvas, radius: f32, seed: u64) {
    let mut rng = StdRng::seed_from_u64(seed);

    for pixel in canvas.pixels.chunks_exact_mut(4) {
        let r_noise = rng.random_range(-radius..=radius);
        let g_noise = rng.random_range(-radius..=radius);
        let b_noise = rng.random_range(-radius..=radius);

        let r = (pixel[0] as f32 + r_noise).clamp(0.0, 255.0) as u8;
        let g = (pixel[1] as f32 + g_noise).clamp(0.0, 255.0) as u8;
        let b = (pixel[2] as f32 + b_noise).clamp(0.0, 255.0) as u8;
        let a = pixel[3];

        pixel.copy_from_slice(&[r, g, b, a]);
    }
}

#[allow(dead_code)]
pub fn gray_noise_int_single(canvas: &mut Canvas, radius: i16, seed: u64) {
    let mut rng = StdRng::seed_from_u64(seed);

    for pixel in canvas.pixels.chunks_exact_mut(4) {
        let noise = rng.random_range(-radius..=radius) as i16;

        let r = (pixel[0] as i16 + noise).clamp(0, 255) as u8;
        let g = (pixel[1] as i16 + noise).clamp(0, 255) as u8;
        let b = (pixel[2] as i16 + noise).clamp(0, 255) as u8;
        let a = pixel[3];

        pixel.copy_from_slice(&[r, g, b, a]);
    }
}

#[allow(dead_code)]
pub fn gray_noise_float_single(canvas: &mut Canvas, radius: f32, seed: u64) {
    let mut rng = StdRng::seed_from_u64(seed);

    for pixel in canvas.pixels.chunks_exact_mut(4) {
        let noise = rng.random_range(-radius..=radius);

        let r = (pixel[0] as f32 + noise).clamp(0.0, 255.0) as u8;
        let g = (pixel[1] as f32 + noise).clamp(0.0, 255.0) as u8;
        let b = (pixel[2] as f32 + noise).clamp(0.0, 255.0) as u8;
        let a = pixel[3];

        pixel.copy_from_slice(&[r, g, b, a]);
    }
}
