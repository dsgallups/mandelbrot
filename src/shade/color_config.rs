use super::Rgb;

pub struct ColorConfig {
    color: Rgb,
    light: bool,
}
impl ColorConfig {
    pub const fn red(light: bool) -> Self {
        Self {
            color: Rgb::red(),
            light,
        }
    }

    pub const fn new_single_color(color: Rgb, light: bool) -> Self {
        Self { light, color }
    }

    ///Get the color value based on a percentage of shade, 0-1
    pub const fn get_color(&self, percent_of_shade: f64) -> Rgb {
        // this is the value of the range based on the shade
        if self.light {
            self.color.const_mul(percent_of_shade)
        } else {
            self.color.const_mul(1. - percent_of_shade)
        }
    }
}
