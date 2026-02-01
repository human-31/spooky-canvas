use rayon::result;

use super::super::canvas::Canvas;
use super::par_pixel;
use super::par_row;
use super::single_thread;

use std::collections::HashMap;

pub fn fill_all_table(
    width: u32, height: u32,
    sets: u32, iterations: u32,
    print: bool,
) -> (HashMap<String, u32>, HashMap<String, Vec<std::time::Duration>>) {
    let mut canvas = Canvas::with_rgba(width, height, 20, 90, 220, 255);

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