pub struct OpacityConfig {
    /// Shade grows downward (255 -> 0) if light
    ///                       max -> min
    /// Otherwise, shade grows upward (0 -> 255)
    ///                                min -> max
    light: bool,
    /// should be less than or equal to max
    min: f64,
    /// should be greater than or equal to min
    max: f64,
}

impl OpacityConfig {
    fn assert_valid(&self) {
        assert!(self.min <= self.max);
        assert!(((0.)..=1.).contains(&self.min));
        assert!(((0.)..=1.).contains(&self.max));
    }
    pub const fn light_default() -> Self {
        Self {
            light: true,
            min: 20. / 255.,
            max: 1.,
        }
    }
    pub const fn dark_default() -> Self {
        Self {
            light: false,
            min: 20. / 255.,
            max: 1.,
        }
    }
    /// Does not check for correctness.
    ///
    /// Ensure that min and max are between 0 and 1, and min is less than or equal to max.
    pub const fn new(light: bool, min: f64, max: f64) -> Self {
        Self { light, min, max }
    }

    /// Panics if min or max are not between 0 and 1, or if min is greater than max
    pub fn light(min_opacity: f64, max_opacity: f64) -> Self {
        let res = Self {
            light: true,
            min: min_opacity,
            max: max_opacity,
        };
        res.assert_valid();
        res
    }

    /// Panics if min or max are not between 0 and 1, or if min is greater than max
    pub fn dark(min_opacity: f64, max_opacity: f64) -> Self {
        let res = Self {
            light: false,
            min: min_opacity,
            max: max_opacity,
        };
        res.assert_valid();
        res
    }

    /// Returns a portion of the range that determines the opacity
    pub const fn opacity_range(&self) -> f64 {
        self.max - self.min
    }

    ///Get the shade value based on a percentage of shade, 0-1
    pub const fn get_opacity(&self, percent_of_opacity: f64) -> f64 {
        let range = self.opacity_range();

        // this is the value of the range based on the shade
        let range_percent = range * percent_of_opacity;
        if self.light {
            self.min + range_percent
        } else {
            self.max - range_percent
        }
    }

    /// returns a value between 0-255
    pub const fn get_opacity_u8(&self, percent_of_opacity: f64) -> u8 {
        (self.get_opacity(percent_of_opacity) * 255.) as u8
    }
}
