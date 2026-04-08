//! # test docs in canvas_mods/fx/core.rs
//! 
//! ## canvas_mods/fx/core.rs h2
//! 
//! qwertyuiop
//! asdfghjkl
//! zxcvbnm
//! 
//! ### canvas_mods/fx/core.rs h3
//! 
//! - 10
//! - 31
//! - 06

use crate::spooky_canvas::Canvas;
use super::pixel_wise;

pub struct FxPipeline<'a> {
    canvas: &'a mut Canvas,
    seed: u64,
}

// TODO: maybe make these required for all pipelines
// maybe required impls for pipelines
impl<'a> FxPipeline<'a> {
    #[allow(dead_code)]
    pub fn new(canvas: &'a mut Canvas) -> Self {
        FxPipeline {
            canvas,
            seed: 1031,
        }
    }

    #[allow(dead_code)]
    pub fn set_seed(mut self, seed: u64) -> Self {
        self.seed = seed;
        self
    }
}


impl<'a> FxPipeline<'a> {
    #[allow(dead_code)]
    pub fn grayscale_avg(self) -> Self {
        pixel_wise::grayscale_avg_single(self.canvas);
        self
    }

    #[allow(dead_code)]
    pub fn grayscale_lum(self) -> Self {
        pixel_wise::grayscale_lum_single(self.canvas);
        self
    }

    #[allow(dead_code)]
    pub fn rgb_noise_int(self, radius: i16) -> Self {
        pixel_wise::rgb_noise_int_single(self.canvas, radius, self.seed);
        self
    }

    #[allow(dead_code)]
    pub fn rgb_noise_float(self, radius: f32) -> Self {
        pixel_wise::rgb_noise_float_single(self.canvas, radius, self.seed);
        self
    }

    #[allow(dead_code)]
    pub fn gray_noise_int(self, radius: i16) -> Self {
        pixel_wise::gray_noise_int_single(self.canvas, radius, self.seed);
        self
    }

    #[allow(dead_code)]
    pub fn gray_noise_float(self, radius: f32) -> Self {
        pixel_wise::gray_noise_float_single(self.canvas, radius, self.seed);
        self
    }

}
