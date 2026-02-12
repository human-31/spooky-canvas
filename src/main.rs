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

    spooky_canvas::raster::multi_thread
        ::fill_all_par_pixel(&mut c, [255, 255, 0, 255]);

    spooky_canvas::raster::single_thread
        ::rect_xy(&mut c, (50, 100), (200, 150), [255, 255, 255, 255]);

    spooky_canvas::raster::single_thread
        ::triangle_xy(&mut c, (150, 150), (50, 400), (200, 300), [0, 90, 255, 255]);

    fn px_around(canvas: &mut spooky_canvas::Canvas, spread: u32, x: u32, y: u32, r: u8, g: u8, b: u8, a: u8) {
        canvas.set_pixel_unchecked(x, y-spread, r, g, b, a);
        canvas.set_pixel_unchecked(x+spread, y, r, g, b, a);
        canvas.set_pixel_unchecked(x, y+spread, r, g, b, a);
        canvas.set_pixel_unchecked(x-spread, y, r, g, b, a);
    }

    px_around(&mut c, 2, 150, 150, 0, 255, 0, 255);
    px_around(&mut c, 2, 50, 400, 0, 255, 0, 255);
    px_around(&mut c, 2, 200, 300, 0, 255, 0, 255);

    c.save_as_png("output/main-test-0.png");
}
