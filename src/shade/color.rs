pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    /// 0-255 where 255 is max alpha
    pub a: u8,
}
impl Rgba {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
    pub const fn splat(c: u8) -> Self {
        Self { r: c, g: c, b: c }
    }
    pub const fn with_alpha(self, a: u8) -> Rgba {
        Rgba::new(self.r, self.g, self.b, a)
    }
    pub const fn into_rgba(self) -> Rgba {
        Rgba::new(self.r, self.g, self.b, 255)
    }
}
