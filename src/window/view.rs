use nalgebra::Vector2;

pub struct Plot {
    left: f64,
    right: f64,
    top: f64,
    bottom: f64,
}

impl Plot {
    /// Maps a window width and height, transform, and zoom into a window.
    ///
    /// X is to the right.
    /// Y is up.
    ///
    /// Transform assumes middle of the window
    ///
    /// Zoom is applied after the transform.
    pub fn new_from_dims(dimensions: Vector2<f64>, transform: Vector2<f64>, zoom: f64) -> Self {
        let width = dimensions.x;
        let height = dimensions.y;
        let initial_left = transform.x - (width / 2.);
        let initial_right = transform.x + (width / 2.);
        let initial_top = transform.y + (height / 2.);
        let initial_bottom = transform.y - (height / 2.);

        Self {
            left: initial_left * zoom,
            right: initial_right * zoom,
            top: initial_top * zoom,
            bottom: initial_bottom * zoom,
        }
    }

    pub const fn new(top: f64, right: f64, bottom: f64, left: f64) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }
    pub fn zoom(&mut self, amt: f64) {
        self.left *= amt;
        self.right *= amt;
        self.top *= amt;
        self.bottom *= amt;
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
