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

#[allow(dead_code)]
pub fn rgb_avg(rgba: Rgba) -> u8 {
    let r = rgba[0] as u16;
    let g = rgba[1] as u16;
    let b = rgba[2] as u16;

    ((r + g + b) / 3) as u8
}
