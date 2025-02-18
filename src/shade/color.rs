use std::ops::Mul;

#[derive(Clone)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    /// 0-255 where 255 is max alpha
    pub a: u8,
}

impl Rgba {
    pub const fn transparent() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        }
    }
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
    pub fn into_rgb_byte(self) -> u32 {
        // Scale each channel by alpha (integer math)
        let r_scaled = (self.r as u16 * self.a as u16) / 255;
        let g_scaled = (self.g as u16 * self.a as u16) / 255;
        let b_scaled = (self.b as u16 * self.a as u16) / 255;

        // Put them into 0x00RRGGBB
        ((r_scaled as u32) << 16) | ((g_scaled as u32) << 8) | (b_scaled as u32)
    }
}

pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    pub const fn red() -> Self {
        Self::new(255, 0, 0)
    }
    pub const fn green() -> Self {
        Self::new(0, 255, 0)
    }
    pub const fn blue() -> Self {
        Self::new(0, 0, 255)
    }
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
    pub const fn const_mul(&self, rhs: f64) -> Rgb {
        let r = ((self.r as f64) * rhs) as u8;
        let g = ((self.g as f64) * rhs) as u8;
        let b = ((self.g as f64) * rhs) as u8;
        Rgb::new(r, g, b)
    }
}

impl Mul<f64> for Rgb {
    type Output = Rgb;
    fn mul(self, rhs: f64) -> Rgb {
        let r = ((self.r as f64) * rhs) as u8;
        let g = ((self.g as f64) * rhs) as u8;
        let b = ((self.g as f64) * rhs) as u8;
        Rgb::new(r, g, b)
    }
}
