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
    r: u8, g: u8, b: u8, a: u8,
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
    r: u8, g: u8, b: u8, a: u8,
) {

}


/// Fills the entire canvas with the specified RGBA color.
pub fn fill_all(
    canvas: &mut Canvas,
    r: u8, g: u8, b: u8, a: u8,
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
    r: u8, g: u8, b: u8, a: u8,
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
    r: u8, g: u8, b: u8, a: u8,
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
    r: u8, g: u8, b: u8, a: u8,
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
    r: u8, g: u8, b: u8, a: u8,
) {

}

#[allow(dead_code)]
pub fn main_test() {
    println!("===============================================================");
    println!();

    if false {
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

    if false {
        /*
        kinds of tests to run for test_fill_all():
          - small square
          - large square
          - small wide rectangle
          - large wide rectangle
          - small tall rectangle
          - large tall rectangle
        */

        let sets = 10;
        let small_iter = 1;
        let large_iter = 10;

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
}