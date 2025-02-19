use nalgebra::Vector2;
use rug::{Complex, Float};

pub struct Plot {
    bottom_left: Complex,
    dimensions: Complex,
}

impl Plot {
    pub fn new_for_window(
        window_width: usize,
        window_height: usize,
        translation: &Complex,
        zoom: &Float,
        precision: u32,
    ) -> Self {
        /*
            so the bottom left is calculated by first

            getting the value of the center.


            so let's get the width.

            we will say that the width is
            the zoom times the window width,
            and the height is
            the zoom times the window height.

        */
        let dimensions = Complex::with_val(precision, (window_width, window_height));
        let half = Float::with_val(precision, 0.5);

        let plot_dimensions = Complex::with_val(precision, &dimensions * zoom);

        let half_dims = Complex::with_val(precision, &plot_dimensions * &half);

        let bottom_left = translation - &half_dims;

        Self {
            bottom_left: Complex::with_val(precision, bottom_left),
            dimensions: plot_dimensions,
        }
    }

    /// Returns the pixel based on the provided percentage values from the bottom left
    pub fn get_pixel_from_percent(&self, perc_x: f64, perc_y: f64) -> (f64, f64) {
        let diff_x = self.right - self.left;
        let diff_y = self.top - self.bottom;

        let x = self.left + (diff_x * perc_x);
        let y = self.bottom + (diff_y * perc_y);

        (x, y)
    }
}
