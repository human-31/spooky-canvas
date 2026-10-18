use spooky_colors::{RgbaRaw, RgbaExt};

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

    /// fun test, ill prolly put this into its own module with more funcitonality
    pub fn to_img_string(&self) -> String {
        let ramp = " .:-=+*#%@";
        let mut ramp_vec: Vec<&str> = ramp.split("").skip(1).collect();
        ramp_vec.pop().expect("empty ramp");

        let to_char = |luma: f32| -> &str {
            let index = ((luma * (ramp_vec.len() as f32)) as usize).clamp(0, ramp_vec.len() - 1);

            ramp_vec[index]
        };

        let mut out = String::new();

        for y in 0..self.height {
            if y != 0 {
                out += "\n";
            }

            for x in 0..self.width {
                let luma = unsafe{self.pixel_at_unchecked(x, y)}.luma();
                out += to_char(luma);
            }
        }

        out
    }

    /// might delater?
    pub fn scaled_nearest(&self, new_w: u16, new_h: u16) -> Self {
        let (w, h) = (self.width as usize, self.height as usize);
        let (nw, nh) = (new_w as usize, new_h as usize);
        assert!(w > 0 && h > 0 && nw > 0 && nh > 0);

        // Column mapping is the same for every row, so compute it once.
        // Using pixel centers avoids a half-pixel shift.
        let x_map: Vec<usize> = (0..nw).map(|x| ((2 * x + 1) * w) / (2 * nw)).collect();

        let mut out: Vec<[u8; 4]> = Vec::with_capacity(nw * nh);
        for y in 0..nh {
            let sy = ((2 * y + 1) * h) / (2 * nh);
            let row = &self.pixels[sy * w * 4..(sy + 1) * w * 4];
            out.extend(x_map.iter().map(|&sx| {
                let p = &row[sx * 4..sx * 4 + 4];
                [p[0], p[1], p[2], p[3]]
            }));
        }

        Self { width: new_w, height: new_h, pixels: out.into_flattened() }
    }

    /// might delater?
    pub fn scaled_bilinear(&self, new_w: u16, new_h: u16) -> Self {
        let (w, h) = (self.width as usize, self.height as usize);
        let (nw, nh) = (new_w as usize, new_h as usize);
        assert!(w > 0 && h > 0 && nw > 0 && nh > 0);

        let x_scale = w as f32 / nw as f32;
        let y_scale = h as f32 / nh as f32;
        let mut out: Vec<u8> = Vec::with_capacity(nw * nh * 4);

        for y in 0..nh {
            let fy = ((y as f32 + 0.5) * y_scale - 0.5).max(0.0);
            let y0 = (fy as usize).min(h - 1);
            let y1 = (y0 + 1).min(h - 1);
            let ty = fy - y0 as f32;

            for x in 0..nw {
                let fx = ((x as f32 + 0.5) * x_scale - 0.5).max(0.0);
                let x0 = (fx as usize).min(w - 1);
                let x1 = (x0 + 1).min(w - 1);
                let tx = fx - x0 as f32;

                let at = |xx: usize, yy: usize, c: usize| self.pixels[(yy * w + xx) * 4 + c] as f32;

                for c in 0..4 {
                    let top = at(x0, y0, c) * (1.0 - tx) + at(x1, y0, c) * tx;
                    let bot = at(x0, y1, c) * (1.0 - tx) + at(x1, y1, c) * tx;
                    out.push((top * (1.0 - ty) + bot * ty + 0.5) as u8);
                }
            }
        }

        Self { width: new_w, height: new_h, pixels: out }
    }
}