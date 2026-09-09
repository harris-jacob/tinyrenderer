/// A single pixel.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    /// Packed as 0RGB, the layout minifb expects.
    const fn to_packed(self) -> u32 {
        (self.r as u32) << 16 | (self.g as u32) << 8 | self.b as u32
    }

    const fn from_packed(p: u32) -> Self {
        Color::rgb((p >> 16) as u8, (p >> 8) as u8, p as u8)
    }
}

/// An image in memory. The origin is the bottom-left corner and y grows upwards.
///
/// Coordinates are signed: rasterization works in signed space, and anything
/// outside the image is simply not drawn.
pub struct Framebuffer {
    width: usize,
    height: usize,
    pixels: Vec<u32>,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Framebuffer { width, height, pixels: vec![0; width * height] }
    }

    pub fn width(&self) -> i32 {
        self.width as i32
    }

    pub fn height(&self) -> i32 {
        self.height as i32
    }

    /// Out-of-bounds reads return black, out-of-bounds writes are ignored.
    pub fn get(&self, x: i32, y: i32) -> Color {
        match self.index(x, y) {
            Some(i) => Color::from_packed(self.pixels[i]),
            None => Color::default(),
        }
    }

    pub fn set(&mut self, x: i32, y: i32, c: Color) {
        if let Some(i) = self.index(x, y) {
            self.pixels[i] = c.to_packed();
        }
    }

    pub fn clear(&mut self, c: Color) {
        self.pixels.fill(c.to_packed());
    }

    /// The raw pixel buffer, top row first — ready to hand to minifb.
    pub fn pixels(&self) -> &[u32] {
        &self.pixels
    }

    /// Rows are stored top-down (what the window wants) while the public API is
    /// y-up, so the flip happens here and nowhere else. This is also the only
    /// place a signed coordinate becomes an index, so the range check below is
    /// what makes the casts sound.
    fn index(&self, x: i32, y: i32) -> Option<usize> {
        let in_bounds = (0..self.width()).contains(&x) && (0..self.height()).contains(&y);
        in_bounds.then(|| x as usize + (self.height - 1 - y as usize) * self.width)
    }
}
