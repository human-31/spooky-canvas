use rayon::iter;

use super::super::canvas::Canvas;
use super::tests;

pub fn circle(
    canvas: &mut Canvas,
    x: u32, y: u32, radius: u32,
    r: u8, g: u8, b: u8, a: u8,
) {

}

pub fn ellipse(
    canvas: &mut Canvas,
    x: u32, y: u32, x_radius: u32, y_radius: u32,
    r: u8, g: u8, b: u8, a: u8,
) {

}


pub fn fill_all(
    canvas: &mut Canvas,
    r: u8, g: u8, b: u8, a: u8,
) {

}

pub fn polygon(
    canvas: &mut Canvas,
    points: &[(u32, u32)],
    r: u8, g: u8, b: u8, a: u8,
) {

}

pub fn rect(
    canvas: &mut Canvas,
    x: u32, y: u32, width: u32, height: u32,
    r: u8, g: u8, b: u8, a: u8,
) {

}

pub fn square(
    canvas: &mut Canvas,
    x: u32, y: u32, size: u32,
    r: u8, g: u8, b: u8, a: u8,
) {

}

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