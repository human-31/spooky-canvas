use spooky_colors::RgbaRaw;

use image::{ImageReader, ImageBuffer, Rgba};

pub struct CanvasRaw {
    // max size: 65,535 x 65,535
    width: u16,
    height: u16,

    // flat vector of RGBA bytes in row-major order
    pub pixels: Vec<u8>,
}

impl CanvasRaw {
    ////////////////////////////////////////////////////////////////////////////
    // CONSTRUCTORS ////////////////////////////////////////////////////////////
    ////////////////////////////////////////////////////////////////////////////
    
    /// init w/ transparent black
    pub fn new(width: u16, height: u16) -> Self {
        let size = (width * height * 4) as usize;
        let pixels = vec![0; size];

        Self {
            width,
            height,
            pixels,
        }
    }

    pub fn with_rgba(width: u16, height: u16, rgba: RgbaRaw) -> Self {
        let mut pixels = Vec::with_capacity((width * height * 4) as usize);

        for px in pixels.chunks_exact_mut(4) {
            px.copy_from_slice(&rgba);
        }

        Self {
            width,
            height,
            pixels
        }
    }

    pub fn from_png(path: &str) -> Self {
        todo!("TODO: CanvasRaw::from_png")
    }

    ////////////////////////////////////////////////////////////////////////////
    // GETTERS /////////////////////////////////////////////////////////////////
    ////////////////////////////////////////////////////////////////////////////
    
    fn width(&self) -> u16 {
        self.width
    }

    fn height(&self) -> u16 {
        self.height
    }

    /// (width, height)
    fn size(&self) -> (u16, u16) {
        (self.width, self.height)
    }

    fn pixel_count(&self) -> u32 {
        (self.width as u32) * (self.height as u32)
    }

    pub fn pixel_at(&self, x: u16, y: u16) -> RgbaRaw {
        todo!("TODO: CanvasRaw::pixel_at_unchecked")
    }

    pub unsafe fn pixel_at_unchecked(&self, x: u16, y: u16) -> RgbaRaw {
        todo!("TODO: CanvasRaw::pixel_at_unchecked")
    }

    ////////////////////////////////////////////////////////////////////////////
    // SETTERS /////////////////////////////////////////////////////////////////
    ////////////////////////////////////////////////////////////////////////////

    pub fn set_pixel(&self, x: u16, y: u16, rgba: RgbaRaw) {
        todo!("TODO: CanvasRaw::set_pixel")
    }

    pub unsafe fn set_pixel_unchecked(&self, x: u16, y: u16, rgba: RgbaRaw) {
        todo!("TODO: CanvasRaw::set_pixel_unchecked")
    }

    ////////////////////////////////////////////////////////////////////////////
    // CHECKERS ////////////////////////////////////////////////////////////////
    ////////////////////////////////////////////////////////////////////////////

    pub fn index_in_bounds(&self, index: usize) -> bool {
        index < self.pixels.len()
    }

    pub fn xy_in_bounds(&self, x: u16, y: u16) -> bool {
        x < self.width && y < self.height
    }

    ////////////////////////////////////////////////////////////////////////////
    // CONVERTERS //////////////////////////////////////////////////////////////
    ////////////////////////////////////////////////////////////////////////////
    
    pub fn index_to_xy(&self, index: usize) -> Option<(u16, u16)> {
        todo!("TODO: CanvasRaw::index_to_xy")
    }

    pub unsafe fn index_to_xy_unchecked(&self, index: usize) -> (u16, u16) {
        todo!("TODO: CanvasRaw::index_to_xy")
    }
    
    pub fn xy_to_index(&self, x: u16, y: u16) -> Option<usize> {
        todo!("TODO: CanvasRaw::xy_to_index")
    }
    
    pub unsafe fn xy_to_index_unchecked(&self, x: u16, y: u16) -> usize {
        todo!("TODO: CanvasRaw::xy_to_index_unchecked")
    }

    ////////////////////////////////////////////////////////////////////////////
    // MISC ////////////////////////////////////////////////////////////////////
    ////////////////////////////////////////////////////////////////////////////
    
    pub fn save_as_png(&self, path: &str) {
        let buffer = ImageBuffer::<Rgba<u8>, _>::from_raw(
            self.width as u32,
            self.height as u32,
            self.pixels.clone(),
        ).expect("Invalid image buffer");

        buffer.save(path).expect("Failed to save PNG");
    }
}