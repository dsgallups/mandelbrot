use image::Rgba as ImageRgba;
use minifb::{Key, Window as ClientWindow, WindowOptions};
use nalgebra::Vector2;
use rug::{Complex, Float};

use crate::{DefaultImage, Mandelbrot};
use rayon::iter::{
    IndexedParallelIterator as _, IntoParallelRefMutIterator as _, ParallelIterator,
};

mod view;
pub use view::*;

pub struct MandelbrotWindow {
    pub window_width: usize,
    pub window_height: usize,
    plot: Plot,
}

impl MandelbrotWindow {
    /**
    So we want to begin on a box that's centered on the origin
    where the plot value of 2.0 is touching the left.
    Centered at zero
    */
    pub fn new(window_width: usize, window_height: usize) -> Self {
        let plot_width = 2.;
        let height_per_width = window_height as f64 / window_width as f64;
        let plot_height = height_per_width * plot_width;

        let plot_dims = Vector2::new(plot_width, plot_height);

        let plot = Plot::new_from_dims(plot_dims, Vector2::zeros(), 1.0);

        //let plot = Plot::new(1., 0.5, -1., -2.);
        Self {
            window_width,
            window_height,
            plot,
        }
    }

    fn percent_from_bottom_left(&self, x: usize, y: usize) -> (f64, f64) {
        //let diff_x = self.width - x;
        let perc_x = x as f64 / self.window_width as f64;
        let perc_y = y as f64 / self.window_height as f64;

        (perc_x, perc_y)
    }
    pub fn render(&mut self, window_buf: &mut Vec<u32>, mandelbrot: &Mandelbrot) {
        let height = self.window_height;
        let width = self.window_width;

        println!("render called!");

        window_buf
            .par_iter_mut()
            .enumerate()
            .for_each(|(i, rgb_byte)| {
                let x = i % width;
                let y = i / height;
                let (perc_x, perc_y) = self.percent_from_bottom_left(x, y);
                let (plot_x, plot_y) = self.plot.get_pixel_from_percent(perc_x, perc_y);
                let pix = mandelbrot.get_point_color(plot_x, plot_y);
                //println!("pix: {}", pix.into_rgb_byte());
                //*rgb_byte = 0x00FFFFFF;
                *rgb_byte = pix.into_rgb_byte();
            });
        println!("render complete");
    }
}
