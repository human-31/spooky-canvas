use image::{ImageBuffer, Rgba};

pub struct Image {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8> // RGBA row-major
}

impl Image {
    pub fn new(width: u32, height: u32, rgba: [u8; 4]) -> Self {
        let mut pixels = Vec::with_capacity((width * height * 4) as usize);

        for _ in 0..(width * height) {
            pixels.extend_from_slice(&rgba);
        }

        Self {
            width,
            height,
            pixels,
        }
    }
}

#[allow(dead_code)]
pub fn run() {
    let img = Image::new(256, 256, [255, 0, 255, 255]);

    save_png(&img, "output.png");
}

pub fn save_png(img: &Image, path: &str) {
    let buffer = ImageBuffer::<Rgba<u8>, _>::from_raw(
        img.width,
        img.height,
        img.pixels.clone(),
    ).expect("Invalid image buffer");

    buffer.save(path).expect("Failed to save PNG");
}