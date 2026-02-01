//! # test docs in spooky_canvas/canvas.rs
//! 
//! ## spooky_canvas/canvas.rs h2
//! 
//! qwertyuiop
//! asdfghjkl
//! zxcvbnm
//! 
//! ### spooky_canvas/canvas.rs h3
//! 
//! - 10
//! - 31
//! - 06

use image::{ImageBuffer, Rgba};

/// Simple RGBA canvas.
pub struct Canvas {
    /// Width of the canvas in pixels.
    width: u32,

    /// Height of the canvas in pixels.
    height: u32,

    /// Pixel data stored as a flat vector of bytes (RGBA) in row-major order.
    pub pixels: Vec<u8>,
}

/// Constructors
impl Canvas {
    /// Creates a new canvas with the specified width and height,
    /// initialized to transparent black.
    pub fn new(width: u32, height: u32) -> Self {
        let pixels = vec![0; (width * height * 4) as usize];

        Self {
            width,
            height,
            pixels,
        }
    }

    /// Creates a new canvas with the specified width, height, and RGBA color.
    pub fn with_rgba(
        width: u32, height: u32,
        r: u8, g: u8, b: u8, a: u8,
    ) -> Self {
        let mut pixels = Vec::with_capacity((width * height * 4) as usize);

        for _ in 0..(width * height) {
            pixels.push(r);
            pixels.push(g);
            pixels.push(b);
            pixels.push(a);
        }

        Self {
            width,
            height,
            pixels
        }
    }
}

/// Getters
impl Canvas {
    /// Returns the width of the canvas in pixels.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Returns the height of the canvas in pixels.
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Returns the size of the canvas as (width, height) in pixels.
    pub fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    /// Returns the total number of pixels in the canvas.
    pub fn pixel_count(&self) -> u32 {
        self.width * self.height
    }

    /// Returns the RGBA value of the pixel at (x, y) without bounds checking.
    pub fn pixel_unchecked(&self, x: u32, y: u32) -> (u8, u8, u8, u8) {
        let i = self.xy_to_index_unchecked(x, y);

        (self.pixels[i], self.pixels[i + 1], self.pixels[i + 2], self.pixels[i + 3])
    }
}

/// Setters
impl Canvas {
    /// Sets the RGBA value of the pixel at (x, y) without bounds checking.
    pub fn set_pixel_unchecked(
        &mut self, x: u32, y: u32,
        r: u8, g: u8, b: u8, a: u8,
    ) {
        let i = self.xy_to_index_unchecked(x, y);

        self.pixels[i] = r;
        self.pixels[i + 1] = g;
        self.pixels[i + 2] = b;
        self.pixels[i + 3] = a;
    }
}

/// Checkers
impl Canvas {
    /// Returns true if the given index is within the bounds of the pixel data.
    pub fn index_in_bounds(&self, index: usize) -> bool {
        index < self.pixels.len()
    }

    /// Returns true if the given (x, y) coordinates are within
    /// the canvas bounds.
    pub fn xy_in_bounds(&self, x: u32, y: u32) -> bool {
        x < self.width && y < self.height
    }
}

/// Conversions
impl Canvas {
    /// Returns the (x, y) coordinates corresponding to the given index,
    /// or None if the index is out of bounds.
    pub fn index_to_xy(&self, index: usize) -> Option<(u32, u32)>{
        if !self.index_in_bounds(index) {
            return None;
        }

        Some(self.index_to_xy_unchecked(index))
    }
    
    /// Returns the (x, y) coordinates corresponding to the given index,
    /// or panics if the index is out of bounds.
    pub fn index_to_xy_expect(&self, index: usize) -> (u32, u32) {
        assert!(self.index_in_bounds(index), "index {index} out of bounds");

        self.index_to_xy_unchecked(index)
    }

    /// Returns the (x, y) coordinates corresponding to the given index
    /// without bounds checking.
    pub fn index_to_xy_unchecked(&self, index: usize) -> (u32, u32) {
        let pixel_index = (index as u32) / 4;

        let x = pixel_index % self.width;
        let y = pixel_index / self.width;

        (x, y)
    }

    /// Returns the index corresponding to the given (x, y) coordinates,
    /// or None if the coordinates are out of bounds.
    pub fn xy_to_index(&self, x: u32, y: u32) -> Option<usize> {
        if !self.xy_in_bounds(x, y) {
            return None;
        }

        Some(self.xy_to_index_unchecked(x, y))
    }

    /// Returns the index corresponding to the given (x, y) coordinates,
    /// or panics if the coordinates are out of bounds.
    pub fn xy_to_index_expect(&self, x: u32, y: u32) -> usize {
        assert!(
            self.xy_in_bounds(x, y),
            "({x}, {y}) out of bounds:\n  - width: {}\n  - height: {}",
            self.width, self.height,
        );

        self.xy_to_index_unchecked(x, y)
    }

    /// Returns the index corresponding to the given (x, y) coordinates
    /// without bounds checking.
    pub fn xy_to_index_unchecked(&self, x: u32, y: u32) -> usize {
        4 * (self.width * y + x) as usize
    }
}

/// Misc
impl Canvas {
    /// Saves the canvas as a PNG file at the specified path.
    pub fn save_as_png(&self, path: &str) {
        let buffer = ImageBuffer::<Rgba<u8>, _>::from_raw(
            self.width,
            self.height,
            self.pixels.clone(),
        ).expect("Invalid image buffer");

        buffer.save(path).expect("Failed to save PNG");
    }
}

/// Private Helpers
impl Canvas {

}

#[allow(dead_code)]
pub fn main_test() {
    println!("Running basic_canvas/canvas.rs\n");

    let canvas = Canvas::new(256, 256);

    let width = canvas.width();
    let height = canvas.height();
    let (w, h) = canvas.size();
    let px_count = canvas.pixel_count();

    println!("canvas");
    println!("  - width: {width}");
    println!("  - height: {height}");
    println!("  - size: ({w}, {h})");
    println!("  - pixel count: {px_count}");
    println!();

    let (x, y) = (10, 31);
    let index = canvas.xy_to_index_unchecked(x, y);
    let (r, g, b, a) = canvas.pixel_unchecked(x, y);

    println!("pixel at ({x}, {y})");
    println!("  - index {index}");
    println!("  - rgba value: ({r}, {g}, {b}, {a})");
    println!();

    let index = 1031;
    let (x, y) = canvas.index_to_xy_unchecked(index);
    let (r, g, b, a) = canvas.pixel_unchecked(x, y);

    println!("pixel at index {index}");
    println!("  - (x, y): ({x}, {y})");
    println!("  - rgba value: ({r}, {g}, {b}, {a})");
    println!();

    canvas.save_as_png("output/canvas-test-0.png");

    let canvas = Canvas::with_rgba(256, 256, 255, 0, 255, 255);

    let width = canvas.width();
    let height = canvas.height();
    let (w, h) = canvas.size();
    let px_count = canvas.pixel_count();

    println!("canvas");
    println!("  - width: {width}");
    println!("  - height: {height}");
    println!("  - size: ({w}, {h})");
    println!("  - pixel count: {px_count}");
    println!();

    let (x, y) = (10, 31);
    let index = canvas.xy_to_index_unchecked(x, y);
    let (r, g, b, a) = canvas.pixel_unchecked(x, y);

    println!("pixel at ({x}, {y})");
    println!("  - index {index}");
    println!("  - rgba value: ({r}, {g}, {b}, {a})");
    println!();

    let index = 1031;
    let (x, y) = canvas.index_to_xy_unchecked(index);
    let (r, g, b, a) = canvas.pixel_unchecked(x, y);

    println!("pixel at index {index}");
    println!("  - (x, y): ({x}, {y})");
    println!("  - rgba value: ({r}, {g}, {b}, {a})");
    println!();

    canvas.save_as_png("output/canvas-test-1.png");
}