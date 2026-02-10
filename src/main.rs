//! # test docs in main.rs
//! 
//! ## main.rs h2
//! 
//! qwertyuiop
//! asdfghjkl
//! zxcvbnm
//! 
//! ### main.rs h3
//! 
//! - 10
//! - 31
//! - 06

mod scratchpaper;
mod spooky_canvas;

fn main() {
    // spooky_canvas::test_canvas();
    // spooky_canvas::test_raster();

    let mut c = spooky_canvas::Canvas::with_rgba(500, 500, [20, 50, 150, 255]);

    spooky_canvas::raster::par_pixel::fill_all(&mut c, [255, 255, 0, 255]);

    spooky_canvas::raster::single_thread::rect_xy(
        &mut c,
        50, 100, 200, 150,
        [255, 255, 255, 255]
    );

    c.save_as_png("output/main-test-0.png");
}
