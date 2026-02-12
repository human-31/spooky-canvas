//! # test docs in spooky_canvas/raster/tests.rs
//! 
//! ## spooky_canvas/raster/tests.rs h2
//! 
//! qwertyuiop
//! asdfghjkl
//! zxcvbnm
//! 
//! ### spooky_canvas/raster/tests.rs h3
//! 
//! - 10
//! - 31
//! - 06

use super::super::canvas::Canvas;
use super::multi_thread;
use super::single_thread;

use std::collections::HashMap;

/// Uses each fill_all() variant multiple times, measuring duration.
pub fn fill_all_table(
    width: u32, height: u32,
    sets: u32, iterations: u32,
    print: bool,
) -> (HashMap<String, u32>, HashMap<String, Vec<std::time::Duration>>) {
    let mut canvas = Canvas::with_rgba(width, height, [20, 90, 220, 255]);

    let mut input = HashMap::new();
    let mut results: HashMap<String, Vec<std::time::Duration>> = HashMap::new();

    input.insert("width".to_string(), width);
    input.insert("height".to_string(), height);
    input.insert("sets".to_string(), sets);
    input.insert("iterations".to_string(), iterations);

    results.insert("single_thread".to_string(), Vec::new());
    results.insert("par_pixel".to_string(), Vec::new());
    results.insert("par_row".to_string(), Vec::new());

    if print {
        println!("On {width}x{height} canvas, filling pixels {iterations} times:");
        println!(" set |  single thread  |    par pixel    |     par row");
        println!("-----|-----------------|-----------------|-----------------");
    }
    
    for i in 0..sets {
        let start = std::time::Instant::now();
        for _ in 0..iterations {
            single_thread::fill_all(&mut canvas, [220, 90, 20, 255]);
        }
        let duration_single = start.elapsed();
        
        let start = std::time::Instant::now();
        for _ in 0..iterations {
            multi_thread::fill_all_par_pixel(&mut canvas, [20, 220, 90, 255]);
        }
        let duration_pixel = start.elapsed();
        
        let start = std::time::Instant::now();
        for _ in 0..iterations {
            multi_thread::fill_all_par_row(&mut canvas, [20, 90, 220, 255]);
        }
        let duration_row = start.elapsed();

        results.get_mut("single_thread").unwrap().push(duration_single);
        results.get_mut("par_pixel").unwrap().push(duration_pixel);
        results.get_mut("par_row").unwrap().push(duration_row);
        
        if !print {
            continue;
        }

        println!(
            " {:>3} | {:>15?} | {:>15?} | {:>15?}",
            i, duration_single, duration_pixel, duration_row
        );
    }

    if print {
        println!();
    }

    (input, results)
}

/// Uses each rect() variant multiple times, measuring duration.
pub fn rect_table(
    x: u32, y: u32,
    width: u32, height: u32,
    canvas_width: u32, canvas_height: u32,
    sets: u32, iterations: u32,
    print: bool,
) -> (HashMap<String, u32>, HashMap<String, Vec<std::time::Duration>>) {
    let mut canvas = Canvas::with_rgba(canvas_width, canvas_height, [20, 90, 220, 255]);

    let mut input = HashMap::new();
    let mut results: HashMap<String, Vec<std::time::Duration>> = HashMap::new();

    input.insert("x".to_string(), x);
    input.insert("y".to_string(), y);
    input.insert("width".to_string(), width);
    input.insert("height".to_string(), height);
    input.insert("canvas_width".to_string(), canvas_width);
    input.insert("canvas_height".to_string(), canvas_height);
    input.insert("sets".to_string(), sets);
    input.insert("iterations".to_string(), iterations);

    results.insert("single_thread".to_string(), Vec::new());
    results.insert("par_pixel".to_string(), Vec::new());
    results.insert("par_row".to_string(), Vec::new());

    if print {
        println!("On {canvas_width}x{canvas_height} canvas, {iterations} times,");
        println!("rastering {width}x{height} rect at ({x},{y}):");
        println!(" set |  single thread  |    par pixel    |     par row");
        println!("-----|-----------------|-----------------|-----------------");
    }

    let x = x as i32;
    let y = y as i32;
    
    for i in 0..sets {
        let start = std::time::Instant::now();
        for _ in 0..iterations {
            single_thread::rect_xy(&mut canvas, (x, y), (width, height), [220, 90, 20, 255]);
        }
        let duration_single = start.elapsed();
        
        let start = std::time::Instant::now();
        for _ in 0..iterations {
            // par_pixel::rect(&mut canvas, x, y, width, height, 220, 90, 20, 255);
        }
        let duration_pixel = start.elapsed();
        
        let start = std::time::Instant::now();
        for _ in 0..iterations {
            // par_row::rect(&mut canvas, x, y, width, height, 220, 90, 20, 255);
        }
        let duration_row = start.elapsed();

        results.get_mut("single_thread").unwrap().push(duration_single);
        results.get_mut("par_pixel").unwrap().push(duration_pixel);
        results.get_mut("par_row").unwrap().push(duration_row);
        
        if !print {
            continue;
        }

        println!(
            " {:>3} | {:>15?} | {:>15?} | {:>15?}",
            i, duration_single, duration_pixel, duration_row
        );
    }

    if print {
        println!();
    }

    (input, results)
}

/// Uses each rect() single-threaded variant multiple times, measuring duration.
pub fn rect_single_thread_table(
    x: u32, y: u32,
    width: u32, height: u32,
    canvas_width: u32, canvas_height: u32,
    sets: u32, iterations: u32,
    print: bool,
) -> (HashMap<String, u32>, HashMap<String, Vec<std::time::Duration>>) {
    let mut canvas = Canvas::with_rgba(canvas_width, canvas_height, [20, 90, 220, 255]);

    let mut input = HashMap::new();
    let mut results: HashMap<String, Vec<std::time::Duration>> = HashMap::new();

    input.insert("x".to_string(), x);
    input.insert("y".to_string(), y);
    input.insert("width".to_string(), width);
    input.insert("height".to_string(), height);
    input.insert("canvas_width".to_string(), canvas_width);
    input.insert("canvas_height".to_string(), canvas_height);
    input.insert("sets".to_string(), sets);
    input.insert("iterations".to_string(), iterations);

    results.insert("xy_loop".to_string(), Vec::new());
    results.insert("by_row".to_string(), Vec::new());
    results.insert("row_buffer".to_string(), Vec::new());

    if print {
        println!("On {canvas_width}x{canvas_height} canvas, {iterations} times,");
        println!("rastering {width}x{height} rect at ({x},{y}):");
        println!(" set |     xy loop     |      by row     |    row buffer");
        println!("-----|-----------------|-----------------|-----------------");
    }

    let x = x as i32;
    let y = y as i32;
    
    for i in 0..sets {
        let start = std::time::Instant::now();
        for _ in 0..iterations {
            single_thread::rect_xy(&mut canvas, (x, y), (width, height), [220, 90, 20, 255]);
        }
        let duration_xy = start.elapsed();
        
        let start = std::time::Instant::now();
        for _ in 0..iterations {
            // par_pixel::rect(&mut canvas, x, y, width, height, [220, 90, 20, 255]);
        }
        let duration_row = start.elapsed();
        
        let start = std::time::Instant::now();
        for _ in 0..iterations {
            // par_row::rect(&mut canvas, x, y, width, height, [220, 90, 20, 255]);
        }
        let duration_buff = start.elapsed();

        results.get_mut("xy_loop").unwrap().push(duration_xy);
        results.get_mut("by_row").unwrap().push(duration_row);
        results.get_mut("row_buffer").unwrap().push(duration_buff);
        
        if !print {
            continue;
        }

        println!(
            " {:>3} | {:>15?} | {:>15?} | {:>15?}",
            i, duration_xy, duration_row, duration_buff
        );
    }

    if print {
        println!();
    }

    (input, results)
}
