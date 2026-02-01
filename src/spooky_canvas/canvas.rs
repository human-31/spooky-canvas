use image::{ImageBuffer, Rgba};

pub struct Canvas {
    width: u32,
    height: u32,
    pub pixels: Vec<u8>, // RGBA row-major
}

// constructors
impl Canvas {
    pub fn new(width: u32, height: u32) -> Self {
        let pixels = vec![0; (width * height * 4) as usize];

        Self {
            width,
            height,
            pixels,
        }
    }

    pub fn with_rgba(width: u32, height: u32, r: u8, g: u8, b: u8, a: u8) -> Self {
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

// getters
impl Canvas {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn pixel_count(&self) -> u32 {
        self.width * self.height
    }

    pub fn pixel_unchecked(&self, x: u32, y: u32) -> (u8, u8, u8, u8) {
        let i = self.xy_to_index_unchecked(x, y);

        (self.pixels[i], self.pixels[i + 1], self.pixels[i + 2], self.pixels[i + 3])
    }
}

// setters
impl Canvas {
    pub fn set_pixel_unchecked(&mut self, x: u32, y: u32) {
        let i = self.xy_to_index_unchecked(x, y);

        self.pixels[i] = 1;
    }
}

// checkers
impl Canvas {
    pub fn index_in_bounds(&self, index: usize) -> bool {
        index < self.pixels.len()
    }

    pub fn xy_in_bounds(&self, x: u32, y: u32) -> bool {
        x < self.width && y < self.height
    }
}

// conversions
impl Canvas {
    pub fn index_to_xy(&self, index: usize) -> Option<(u32, u32)>{
        if !self.index_in_bounds(index) {
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

        let x = pixel_index % self.width;
        let y = pixel_index / self.width;

        (x, y)
    }

    pub fn xy_to_index(&self, x: u32, y: u32) -> Option<usize> {
        if !self.xy_in_bounds(x, y) {
            return None;
        }

        Some(self.xy_to_index_unchecked(x, y))
    }

    pub fn xy_to_index_expect(&self, x: u32, y: u32) -> usize {
        assert!(
            self.xy_in_bounds(x, y),
            "({x}, {y}) out of bounds:\n  - width: {}\n  - height: {}",
            self.width, self.height,
        );

        self.xy_to_index_unchecked(x, y)
    }

    pub fn xy_to_index_unchecked(&self, x: u32, y: u32) -> usize {
        4 * (self.width * y + x) as usize
    }
}

// misc
impl Canvas {
    pub fn save_as_png(&self, path: &str) {
        let buffer = ImageBuffer::<Rgba<u8>, _>::from_raw(
            self.width,
            self.height,
            self.pixels.clone(),
        ).expect("Invalid image buffer");

        buffer.save(path).expect("Failed to save PNG");
    }
}

// private helpers
impl Canvas {

}

#[allow(dead_code)]
pub fn test() {
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