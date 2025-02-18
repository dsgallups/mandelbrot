use rayon::{iter::IndexedParallelIterator as _, slice::IterMut};

use rayon::iter::{IntoParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

use crate::Rgba;

use super::MBrotRaster;

pub struct RgbaVec {
    width: usize,
    v: Vec<Rgba>,
}

impl RgbaVec {
    pub fn new(width: usize, height: usize) -> Self {
        let v = vec![Rgba::transparent(); width * height];
        Self { width, v }
    }
    pub fn len(&self) -> usize {
        self.v.len()
    }
    pub fn is_empty(&self) -> bool {
        self.v.is_empty()
    }
}

impl<'data> IntoParallelIterator for &'data mut RgbaVec {
    type Item = &'data mut Rgba;
    type Iter = IterMut<'data, Rgba>;
    fn into_par_iter(self) -> Self::Iter {
        (&mut self.v).into_par_iter()
    }
}

impl MBrotRaster for RgbaVec {
    fn init_raster(width: u32, height: u32) -> Self {
        Self::new(width as usize, height as usize)
    }
    fn draw_mandelbrot<F>(&mut self, request_pixel: F)
    where
        F: Fn(u32, u32) -> Rgba + Send + Sync,
    {
        let width = self.width;
        self.par_iter_mut().enumerate().for_each(|(i, pixel)| {
            let x = i % width;
            let y = i / width;
            *pixel = request_pixel(x as u32, y as u32);
        })
    }
    fn save_mandelbrot(&self) {}
}
