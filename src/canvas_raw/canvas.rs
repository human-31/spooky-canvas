use spooky_colors::RgbaRaw;

use image::{ImageReader, ImageBuffer, Rgba};

pub struct CanvasRaw {
    // max size: 65,535 x 65,535
    width: u16,
    height: u16,

    // flat vector of RGBA bytes in row-major order
    pixels: Vec<u8>,
}

impl CanvasRaw {
    ////////////////////////////////////////////////////////////////////////////
    // CONSTRUCTORS ////////////////////////////////////////////////////////////
    ////////////////////////////////////////////////////////////////////////////
    
    /// init w/ transparent black
    pub fn new(width: u16, height: u16) -> Self {
        let size = (width as usize) * (height as usize) * 4;
        let pixels = vec![0; size];

        Self {
            width,
            height,
            pixels,
        }
    }

    pub fn with_rgba(width: u16, height: u16, rgba: RgbaRaw) -> Self {
        let count = (width as usize) * (height as usize);
        let pixels = vec![rgba; count].into_flattened();

        Self {
            width,
            height,
            pixels
        }
    }

    /// fix for all edge cases and stuff later
    pub fn from_png(path: &str) -> Self {
        let img = ImageReader::open(path).unwrap()
            .decode().unwrap()
            .to_rgba8();
        
        let (w32, h32) = img.dimensions();
        let pixels = img.into_raw();

        Self {
            width: w32 as u16,
            height: h32 as u16,
            pixels,
        }
    }

    ////////////////////////////////////////////////////////////////////////////
    // GETTERS /////////////////////////////////////////////////////////////////
    ////////////////////////////////////////////////////////////////////////////
    
    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    /// (width, height)
    pub fn size(&self) -> (u16, u16) {
        (self.width, self.height)
    }

    pub fn pixels(&self) -> &[u8] {
        &self.pixels
    }

    pub fn pixels_mut(&mut self) -> &mut [u8] {
        &mut self.pixels
    }

    pub fn pixel_count(&self) -> u32 {
        (self.width as u32) * (self.height as u32)
    }

    pub fn pixel_at(&self, x: u16, y: u16) -> Option<RgbaRaw> {
        if !self.xy_in_bounds(x, y) {
            return None;
        }

        Some(unsafe { self.pixel_at_unchecked(x, y) })
    }

    pub unsafe fn pixel_at_unchecked(&self, x: u16, y: u16) -> RgbaRaw {
        let i = self.xy_to_index_unchecked(x, y);

        unsafe {
            self.pixels
                .as_ptr()
                .add(i)
                .cast::<[u8; 4]>()
                .read()
        }
    }

    ////////////////////////////////////////////////////////////////////////////
    // SETTERS /////////////////////////////////////////////////////////////////
    ////////////////////////////////////////////////////////////////////////////

    pub fn set_pixel(&mut self, x: u16, y: u16, rgba: RgbaRaw) {
        if !self.xy_in_bounds(x, y) {
            return;
        }

        unsafe {
            self.set_pixel_unchecked(x, y, rgba);
        }
    }

    pub unsafe fn set_pixel_unchecked(&mut self, x: u16, y: u16, rgba: RgbaRaw) {
        let i = self.xy_to_index_unchecked(x, y);

        unsafe {
            self.pixels
                .as_mut_ptr()
                .add(i)
                .cast::<[u8; 4]>()
                .write(rgba);
        }
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
        if !self.index_in_bounds(index) {
            return None;
        }

        Some(self.index_to_xy_unchecked(index))
    }

    pub fn index_to_xy_unchecked(&self, index: usize) -> (u16, u16) {
        debug_assert!(self.index_in_bounds(index));

        let pixel_index = (index / 4) as u32;

        let x32 = pixel_index % self.width as u32;
        let y32 = pixel_index / self.width as u32;

        (x32 as u16, y32 as u16)
    }
    
    pub fn xy_to_index(&self, x: u16, y: u16) -> Option<usize> {
        if !self.xy_in_bounds(x, y) {
            return None;
        }

        Some(self.xy_to_index_unchecked(x, y))
    }
    
    pub fn xy_to_index_unchecked(&self, x: u16, y: u16) -> usize {
        debug_assert!(self.xy_in_bounds(x, y));
    
        4 * ((self.width as usize) * (y as usize) + (x as usize))
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