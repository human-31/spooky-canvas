//! # test docs in spooky_colors/color_spaces/rgb.rs
//! 
//! ## spooky_colors/color_spaces/rgb.rs h2
//! 
//! qwertyuiop
//! asdfghjkl
//! zxcvbnm
//! 
//! ### spooky_colors/color_spaces/rgb.rs h3
//! 
//! - 10
//! - 31
//! - 06

use super::super::rgba::Rgba;

pub struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

/// constructors
impl Rgb {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn with_rgba(rgba: Rgba) -> Self {
        Self {
            r: rgba[0],
            g: rgba[1],
            b: rgba[2],
        }
    }

    pub fn red(&self) -> u8 {
        self.r
    }

    pub fn green(&self) -> u8 {
        self.g
    }

    pub fn blue(&self) -> u8 {
        self.b
    }

    pub fn gray(&self) -> u8 {
        let r = self.r as u8;
        let g = self.g as u8;
        let b = self.b as u8;

        ((r + g + b) / 3) as u8
    }
} 
