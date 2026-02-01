use super::super::canvas::Canvas;
use super::par_pixel;
use super::par_row;
use super::single_thread;

pub fn fill_all_table(
    width: u32, height: u32,
    sets: usize, iterations: u32,
) {
    let mut canvas = Canvas::with_rgba(width, height, 20, 90, 220, 255);

    println!();
    println!("On {width}x{height} canvas, filling pixels {iterations} times:");
    println!();
    println!(" set |  single thread  |    par pixel    |     par row");
    println!("-----|-----------------|-----------------|-----------------");

    for i in 0..sets {
        let start = std::time::Instant::now();
        for _ in 0..iterations {
            single_thread::fill_all(&mut canvas, 220, 90, 20, 255);
        }
        let duration_single = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            par_pixel::fill_all(&mut canvas, 20, 220, 90, 255);
        }
        let duration_pixel = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            par_row::fill_all(&mut canvas, 20, 90, 220, 255);
        }
        let duration_row = start.elapsed();

        println!(" {:>3} | {:>15?} | {:>15?} | {:>15?}",
            i, duration_single, duration_pixel, duration_row
        );
    }
}