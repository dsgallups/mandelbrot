mod image;
pub use image::*;

mod vec;
pub use vec::*;

use crate::Rgba;

pub trait MBrotRaster {
    fn init_raster(width: u32, height: u32) -> Self;

    // takes in a function to get back an rgba value
    fn draw_mandelbrot<F>(&mut self, request_pixel: F)
    where
        F: Fn(u32, u32) -> Rgba + Send + Sync;

    fn save_mandelbrot(&self);
}
