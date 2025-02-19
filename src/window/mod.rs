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
    translation: Complex,
    zoom: Float,
    precision: u32,
}

impl MandelbrotWindow {
    /**
    So we want to begin on a box that's centered on the origin
    where the plot value of 2.0 is touching the left.
    Centered at zero
    */
    #[allow(clippy::new_without_default)]
    pub fn new(precision: u32) -> Self {
        Self {
            translation: Complex::new(precision),
            zoom: Float::new(precision),
            precision,
        }
    }

    pub fn zoom(&mut self, amt: f64) {
        self.zoom *= Float::with_val(self.precision, amt);
    }

    // translates by an amount relative to the current zoom
    pub fn translate(&mut self, x_amt: f64, y_amt: f64) {
        let amount = Complex::with_val(self.precision, (x_amt, y_amt));
        //multiply by the zoom
        let as_zoomed = &amount * &self.zoom;

        self.translation += Complex::with_val(self.precision, as_zoomed);
    }

    fn percent_from_bottom_left(&self, x: usize, y: usize) -> (f64, f64) {
        //let diff_x = self.width - x;
        let perc_x = x as f64 / self.window_width as f64;
        let perc_y = y as f64 / self.window_height as f64;

        (perc_x, perc_y)
    }
    pub fn render(
        &mut self,
        window_buf: &mut Vec<u32>,
        window_width: usize,
        window_height: usize,
        mandelbrot: &Mandelbrot,
    ) {
        window_buf
            .par_iter_mut()
            .enumerate()
            .for_each(|(i, rgb_byte)| {
                let x = i % window_width;
                let y = i / window_height;
                let (perc_x, perc_y) = self.percent_from_bottom_left(x, y);
                let (plot_x, plot_y) = self.plot.get_pixel_from_percent(perc_x, perc_y);
                let pix = mandelbrot.get_point_color(plot_x, plot_y);
                *rgb_byte = pix.into_rgb_byte();
            });
    }
}
