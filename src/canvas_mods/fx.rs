//! # test docs in canvas_mods/fx.rs
//! 
//! ## canvas_mods/fx.rs h2
//! 
//! qwertyuiop
//! asdfghjkl
//! zxcvbnm
//! 
//! ### canvas_mods/fx.rs h3
//! 
//! - 10
//! - 31
//! - 06

mod core;
mod constants;
mod pixel_wise;

use crate::spooky_canvas::Canvas;
use self::core::FxPipeline;

#[allow(dead_code)]
pub trait FxOps {
    fn fx(&mut self) -> FxPipeline<'_>;
}

impl FxOps for Canvas {
    fn fx(&mut self) -> FxPipeline<'_> {
        FxPipeline::new(self)
    }
}