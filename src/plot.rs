pub struct PlotConfig {
    width: u32,
    height: u32,
}

impl PlotConfig {
    pub const fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub const fn width(&self) -> u32 {
        self.width
    }
    pub const fn height(&self) -> u32 {
        self.height
    }

    pub fn percent_from_left(&self, x: u32) -> f64 {
        let diff = self.width - x;
        diff as f64 / self.width as f64
    }

    pub fn percent_from_bottom_left(&self, x: u32, y: u32) -> (f64, f64) {
        let diff_x = self.width - x;
        let perc_x = diff_x as f64 / self.width as f64;

        let diff_y = self.height - y;

        let perc_y = diff_y as f64 / self.height as f64;

        (perc_x, perc_y)
    }

    /// Y points up
    pub fn percent_from_bottom(&self, y: u32) -> f64 {
        let diff = self.height - y;

        diff as f64 / self.height as f64
    }
}
