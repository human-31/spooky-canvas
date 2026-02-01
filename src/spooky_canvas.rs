//! # test docs in spook_canvas.rs
//! 
//! ## spook_canvas.rs h2
//! 
//! qwertyuiop
//! asdfghjkl
//! zxcvbnm
//! 
//! ### spook_canvas.rs h3
//! 
//! - 10
//! - 31
//! - 06

mod canvas;
pub mod raster;

// inside modules to use
pub use canvas::Canvas;

// running each module's tests
pub use canvas::main_test as test_canvas;
pub use raster::main_test as test_raster;