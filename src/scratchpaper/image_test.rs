#![allow(unused)]

use image::{ImageBuffer, Rgba};

pub struct SimpleCanvas {
    _width: u32,
    _height: u32,
    pub pixels: Vec<u8> // RGBA row-major
}

impl SimpleCanvas {
    pub fn new(width: u32, height: u32, rgba: [u8; 4]) -> Self {
        let mut pixels = Vec::with_capacity((width * height * 4) as usize);

        for _ in 0..(width * height) {
            pixels.extend_from_slice(&rgba);
        }

        Self {
            _width: width,
            _height: height,
            pixels,
        }
    }

    pub fn height(&self) -> u32 {
        self._height
    }

    pub fn width(&self) -> u32 {
        self._width
    }

    pub fn size(&self) -> (u32, u32) {
        (self._width, self._height)
    }


    pub fn index_in_bounds(&self, index: usize) -> bool {
        index < self.pixels.len()
    }

    pub fn index_to_xy(&self, index: usize) -> Option<(u32, u32)> {
        if index >= self.pixels.len() {
            return None;
        }

        Some(self.index_to_xy_unchecked(index))
    }

    pub fn index_to_xy_expect(&self, index: usize) -> (u32, u32) {
        assert!(self.index_in_bounds(index), "index {index} out of bounds");

        self.index_to_xy_unchecked(index)
    }

    pub fn index_to_xy_unchecked(&self, index: usize) -> (u32, u32) {
        let pixel_index = (index as u32) / 4;

        let x = pixel_index % self._width;
        let y = pixel_index / self._width;

        (x, y)
    }

    pub fn save_as_png(&self, path: &str) {
        let buffer = ImageBuffer::<Rgba<u8>, _>::from_raw(
            self._width,
            self._height,
            self.pixels.clone(),
        ).expect("Invalid image buffer");

        buffer.save(path).expect("Failed to save PNG");
    }

    pub fn xy_in_bounds(&self, x: u32, y: u32) -> bool {
        x < self._width && y < self._height
    }

    pub fn xy_to_index(&self, x: u32, y: u32) -> Option<usize> {
        if x >= self._width || y >= self._height {
            return None;
        }

        Some(self.xy_to_index_unchecked(x, y))
    }

    pub fn xy_to_index_expect(&self, x: u32, y: u32) -> usize {
        assert!(
            self.xy_in_bounds(x, y),
            "({x}, {y}) out of bounds:\n  - width: {}\n  - height: {}",
            self._width, self._height,
        );

        self.xy_to_index_unchecked(x, y)
    }

    pub fn xy_to_index_unchecked(&self, x: u32, y: u32) -> usize {
        4 * (self._width * y + x) as usize
    }
}

#[allow(dead_code)]
pub fn run() {
}