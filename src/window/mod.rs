use image::Rgba as ImageRgba;
use nalgebra::Vector2;

use crate::{DefaultImage, Mandelbrot};
use rayon::iter::ParallelIterator;

mod view;
pub use view::*;

pub struct Window {
    window_dimensions: Vector2<u32>,
    plot_dimensions: Vector2<f64>,
    translation: Vector2<f64>,

    /// zoom is applied after translation
    zoom: f64,
}

impl Window {
    /// this is probably wrong, but essentially im going to make the window height dependent on the plot ratio
    pub fn new(
        window_width: u32,
        plot_dimensions: Vector2<f64>,
        translation: Vector2<f64>,
        zoom: f64,
    ) -> Self {
        let height_per_width = plot_dimensions.y / plot_dimensions.x;
        let window_height = (height_per_width * window_width as f64) as u32;
        Self {
            window_dimensions: Vector2::new(window_width, window_height),
            plot_dimensions,
            translation,
            zoom,
        }
    }

    pub fn width(&self) -> u32 {
        self.window_dimensions.x
    }
    pub fn height(&self) -> u32 {
        self.window_dimensions.y
    }

    fn percent_from_bottom_left(&self, x: u32, y: u32) -> (f64, f64) {
        //let diff_x = self.width - x;
        let perc_x = x as f64 / self.width() as f64;
        let perc_y = y as f64 / self.height() as f64;

        (perc_x, perc_y)
    }

    pub fn render(&self, mandelbrot: &Mandelbrot) -> DefaultImage {
        let height = self.window_dimensions.y;
        let mut image = DefaultImage::new(self.window_dimensions.x, height);

        let window_view = Plot::new_from_dims(self.plot_dimensions, self.translation, self.zoom);

        image.par_enumerate_pixels_mut().for_each(|(x, y, pixel)| {
            // reversed y
            let (perc_x, perc_y) = self.percent_from_bottom_left(x, height - y);
            let (x, y) = window_view.get_pixel_from_percent(perc_x, perc_y);
            let color = mandelbrot.get_point_color(x, y);
            *pixel = ImageRgba([color.r, color.g, color.b, color.a]);
        });

        image
    }
}
