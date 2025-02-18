use chrono::Utc;
use image::{ImageBuffer, Rgba as ImageRgba};
use rayon::iter::ParallelIterator;

use crate::{MBrotRaster, Rgba};

pub type DefaultImage = ImageBuffer<ImageRgba<u8>, Vec<u8>>;

impl MBrotRaster for DefaultImage {
    fn init_raster(width: u32, height: u32) -> Self {
        Self::new(width, height)
    }
    fn draw_mandelbrot<F>(&mut self, request_pixel: F)
    where
        F: Fn(u32, u32) -> Rgba + Send + Sync,
    {
        let height = self.height();
        self.par_enumerate_pixels_mut().for_each(|(x, y, pixel)| {
            let color = request_pixel(x, height - y);
            *pixel = ImageRgba([color.r, color.g, color.b, color.a]);
        })
    }
    fn save_mandelbrot(&self) {
        let now = Utc::now();
        let path = format!("mandelbrot_{}.png", now.format("%y-%m-%d-%H%M%S"));
        self.save(&path).unwrap();
    }
}
