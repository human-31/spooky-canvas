use spooky_colors::RgbaRaw;

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
        todo!("TDOD: CanvasRaw::from_png")
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
}