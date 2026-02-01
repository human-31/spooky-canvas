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
    /*
    kinds of tests to run for test_fill_all():
      - small square
      - large square
      - small wide rectangle
      - large wide rectangle
      - small tall rectangle
      - large tall rectangle
     */
    tests::fill_all_table(1031, 1031, 10, 100);
}