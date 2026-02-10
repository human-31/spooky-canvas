//! # test docs in spooky_canvas/raster/core.rs
//! 
//! ## spooky_canvas/raster/core.rs h2
//! 
//! qwertyuiop
//! asdfghjkl
//! zxcvbnm
//! 
//! ### spooky_canvas/raster/core.rs h3
//! 
//! - 10
//! - 31
//! - 06

use super::single_thread;
use super::super::canvas::Canvas;
use super::tests;

/// Draws a circle on the given canvas with the specified center coordinates,
/// radius, and RGBA color.
/// 
/// The circle replaces any existing pixels within its area.
/// No anti-aliasing is applied.
pub fn circle(
    canvas: &mut Canvas,
    x: u32, y: u32, radius: u32,
    rgba: [u8; 4],
) {

}

/// Draws an ellipse on the given canvas with the specified center coordinates,
/// radii along the x and y axes, and RGBA color.
/// 
/// The ellipse replaces any existing pixels within its area.
/// No anti-aliasing is applied.
pub fn ellipse(
    canvas: &mut Canvas,
    x: u32, y: u32, x_radius: u32, y_radius: u32,
    rgba: [u8; 4]
) {

}


/// Fills the entire canvas with the specified RGBA color.
pub fn fill_all(
    canvas: &mut Canvas,
    rgba: [u8; 4]
) {

}

/// Draws a polygon on the given canvas defined by the list of (x, y) points,
/// using the specified RGBA color.
/// 
/// The polygon replaces any existing pixels within its area.
/// No anti-aliasing is applied.
pub fn polygon(
    canvas: &mut Canvas,
    points: &[(u32, u32)],
    rgba: [u8; 4]
) {

}

/// Draws a rectangle on the given canvas with the specified top-left corner
/// coordinates, width, height, and RGBA color.
/// 
/// The rectangle replaces any existing pixels within its area.
/// No anti-aliasing is applied.
pub fn rect(
    canvas: &mut Canvas,
    x: u32, y: u32, width: u32, height: u32,
    rgba: [u8; 4],
) {

}


/// Draws a square on the given canvas with the specified top-left corner
/// coordinates, size, and RGBA color.
/// 
/// The square replaces any existing pixels within its area.
/// No anti-aliasing is applied.
pub fn square(
    canvas: &mut Canvas,
    x: u32, y: u32, size: u32,
    rgba: [u8; 4],
) {

}

/// Draws a triangle on the given canvas defined by the three vertex
/// coordinates, using the specified RGBA color.
/// 
/// The triangle replaces any existing pixels within its area.
/// No anti-aliasing is applied.
pub fn triangle(
    canvas: &mut Canvas,
    x1: u32, y1: u32,
    x2: u32, y2: u32,
    x3: u32, y3: u32,
    rgba: [u8; 4],
) {

}

#[allow(dead_code)]
pub fn main_test() {
    println!("===============================================================");
    println!();

    let test_id = 4;

    match test_id {
        0 => {
            let sets = 5;
            let small_iter = 5;
            let medium_iter = 50;
            let large_iter = 500;
            let xlarge_iter = 5000;
        
            tests::fill_all_table(1, 1, sets, small_iter, true);
            tests::fill_all_table(1, 1, sets, medium_iter, true);
            tests::fill_all_table(1, 1, sets, large_iter, true);
            tests::fill_all_table(1, 1, sets, xlarge_iter, true);
        
            println!("=============================================================");
            println!();
            
            tests::fill_all_table(10, 10, sets, small_iter, true);
            tests::fill_all_table(10, 10, sets, medium_iter, true);
            tests::fill_all_table(10, 10, sets, large_iter, true);
            tests::fill_all_table(10, 10, sets, xlarge_iter, true);
            
            println!("=============================================================");
            println!();
            
            tests::fill_all_table(100, 100, sets, small_iter, true);
            tests::fill_all_table(100, 100, sets, medium_iter, true);
            tests::fill_all_table(100, 100, sets, large_iter, true);
            tests::fill_all_table(100, 100, sets, xlarge_iter, true);
        }

        1 => {
            /*
            kinds of tests to run for tests::fill_all_table():
              - small square
              - large square
              - small wide rectangle
              - large wide rectangle
              - small tall rectangle
              - large tall rectangle
            */
        
            let sets = 10;
            let small_iter = 10;
            let large_iter = 100;
        
            // 1:1 small
            tests::fill_all_table(144, 144, sets, small_iter, true);
            tests::fill_all_table(144, 144, sets, large_iter, true);
            
            // 1:1 large
            tests::fill_all_table(1440, 1440, sets, small_iter, true);
            tests::fill_all_table(1440, 1440, sets, large_iter, true);
            
            // 16:9 small
            tests::fill_all_table(144, 81, sets, small_iter, true);
            tests::fill_all_table(144, 81, sets, large_iter, true);
            
            // 16:9 large
            tests::fill_all_table(1440, 810, sets, small_iter, true);
            tests::fill_all_table(1440, 810, sets, large_iter, true);
            
            // 9:16 small
            tests::fill_all_table(81, 144, sets, small_iter, true);
            tests::fill_all_table(81, 144, sets, large_iter, true);
            
            // 9:16 large
            tests::fill_all_table(810, 1440, sets, small_iter, true);
            tests::fill_all_table(810, 1440, sets, large_iter, true);
        
            // 4:3 small
            tests::fill_all_table(144, 108, sets, small_iter, true);
            tests::fill_all_table(144, 108, sets, large_iter, true);
            
            // 4:3 large
            tests::fill_all_table(1440, 1080, sets, small_iter, true);
            tests::fill_all_table(1440, 1080, sets, large_iter, true);
            
            // 3:4 small
            tests::fill_all_table(108, 144, sets, small_iter, true);
            tests::fill_all_table(108, 144, sets, large_iter, true);
            
            // 3:4 large
            tests::fill_all_table(1080, 1440, sets, small_iter, true);
            tests::fill_all_table(1080, 1440, sets, large_iter, true);
        }

        2 => {
            /*
            kinds of tests to run for tests::rect_table():
              - small square
              - large square
              - small wide rectangle
              - large wide rectangle
              - small tall rectangle
              - large tall rectangle
            */
        
            let sets = 5;
            let small_iter = 50;
            let large_iter = 500;
            let canvas_width = 2000;
            let canvas_height = 2000;
            let x = 50;
            let y = 50;
        
            // 1:1 small
            tests::rect_table(x, y, 144, 144, canvas_width, canvas_height, sets, small_iter, true);
            tests::rect_table(x, y, 144, 144, canvas_width, canvas_height, sets, large_iter, true);
            
            // 1:1 large
            tests::rect_table(x, y, 1440, 1440, canvas_width, canvas_height, sets, small_iter, true);
            tests::rect_table(x, y, 1440, 1440, canvas_width, canvas_height, sets, large_iter, true);
            
            // 16:9 small
            tests::rect_table(x, y, 144, 81, canvas_width, canvas_height, sets, small_iter, true);
            tests::rect_table(x, y, 144, 81, canvas_width, canvas_height, sets, large_iter, true);
            
            // 16:9 large
            tests::rect_table(x, y, 1440, 810, canvas_width, canvas_height, sets, small_iter, true);
            tests::rect_table(x, y, 1440, 810, canvas_width, canvas_height, sets, large_iter, true);
            
            // 9:16 small
            tests::rect_table(x, y, 81, 144, canvas_width, canvas_height, sets, small_iter, true);
            tests::rect_table(x, y, 81, 144, canvas_width, canvas_height, sets, large_iter, true);
            
            // 9:16 large
            tests::rect_table(x, y, 810, 1440, canvas_width, canvas_height, sets, small_iter, true);
            tests::rect_table(x, y, 810, 1440, canvas_width, canvas_height, sets, large_iter, true);
        
            // 4:3 small
            tests::rect_table(x, y, 144, 108, canvas_width, canvas_height, sets, small_iter, true);
            tests::rect_table(x, y, 144, 108, canvas_width, canvas_height, sets, large_iter, true);
            
            // 4:3 large
            tests::rect_table(x, y, 1440, 1080, canvas_width, canvas_height, sets, small_iter, true);
            tests::rect_table(x, y, 1440, 1080, canvas_width, canvas_height, sets, large_iter, true);
            
            // 3:4 small
            tests::rect_table(x, y, 108, 144, canvas_width, canvas_height, sets, small_iter, true);
            tests::rect_table(x, y, 108, 144, canvas_width, canvas_height, sets, large_iter, true);
            
            // 3:4 large
            tests::rect_table(x, y, 1080, 1440, canvas_width, canvas_height, sets, small_iter, true);
            tests::rect_table(x, y, 1080, 1440, canvas_width, canvas_height, sets, large_iter, true);
        }

        3 => {
            tests::rect_table(50, 50, 500, 500, 1000, 1000, 10, 50, true);
        }

        4 => {
            let mut canvas = Canvas::with_rgba(500, 500, [64, 20, 30, 40]);
            let rect_fn = single_thread::rect_xy;

            // 0
            rect_fn(&mut canvas, 50, 50, 200, 300, [220, 90, 20, 255]);

            // 1
            rect_fn(&mut canvas, 100, -50, 200, 300, [174, 85, 48, 255]);

            // 2
            rect_fn(&mut canvas, 400, -50, 200, 300, [90, 174, 48, 255]);

            // 3

            // 4
            
            // 5

            // 6
            
            // 7

            // 8

            canvas.save_as_png("output/rect-bounds-test.png");
        }
        _ => {}
    }
}