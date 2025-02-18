#![doc = r#"

Spitballings ideas

Maybe the window should be directly converting what pixel it wants to get the color
back.

So the mandelbrot can relay all possible points

but it's the window that requests the calculation at a point?

Thinking through this...

Really all the mandelbrot needs is its shade values.
it should NOT care about what the window's translation is.
"#]

use crate::{ColorConfig, OpacityConfig, Rgb, Rgba, ShadingType};
/// Defines a graph that is used to determine the pixel color of a point in the set
pub struct Mandelbrot {
    shade: ShadingType,
    num_iterations: u32,
    begin_shade_at_n: u32,
}

impl Default for Mandelbrot {
    fn default() -> Self {
        Self {
            shade: ShadingType::opacity(OpacityConfig::light_default()),
            num_iterations: 255,
            begin_shade_at_n: 10,
        }
    }
}

impl Mandelbrot {
    const DEFAULT_ITER: u32 = 255;
    const DEFALT_BEGIN_SHADE: u32 = 10;
    /// See [`OpacityConfig::light`]
    pub fn light_opacity(min_opacity: f64, max_opacity: f64) -> Self {
        Self {
            shade: ShadingType::light_opacity(min_opacity, max_opacity),
            num_iterations: Self::DEFAULT_ITER,
            begin_shade_at_n: Self::DEFALT_BEGIN_SHADE,
        }
    }

    /// See [`OpacityConfig::dark`]
    pub fn dark_opacity(min_opacity: f64, max_opacity: f64) -> Self {
        Self {
            shade: ShadingType::dark_opacity(min_opacity, max_opacity),
            num_iterations: Self::DEFAULT_ITER,
            begin_shade_at_n: Self::DEFALT_BEGIN_SHADE,
        }
    }

    pub const fn red() -> Self {
        Self {
            shade: ShadingType::color(ColorConfig::red(true)),
            num_iterations: Self::DEFAULT_ITER,
            begin_shade_at_n: Self::DEFALT_BEGIN_SHADE,
        }
    }

    pub const fn red_opacity() -> Self {
        Self {
            shade: ShadingType::OpacityColor {
                color: ColorConfig::red(true),
                opacity: OpacityConfig::light_default(),
            },
            num_iterations: Self::DEFAULT_ITER,
            begin_shade_at_n: Self::DEFALT_BEGIN_SHADE,
        }
    }

    pub const fn light_default() -> Self {
        Self {
            shade: ShadingType::opacity(OpacityConfig::light_default()),
            num_iterations: Self::DEFAULT_ITER,
            begin_shade_at_n: Self::DEFALT_BEGIN_SHADE,
        }
    }
    pub const fn dark_default() -> Self {
        Self {
            shade: ShadingType::opacity(OpacityConfig::dark_default()),
            num_iterations: Self::DEFAULT_ITER,
            begin_shade_at_n: Self::DEFALT_BEGIN_SHADE,
        }
    }

    /// This returns the value of a point in the mandelbrot set. This is irrespective of what the window does.
    pub const fn point_in_set(&self, x: f64, y: f64) -> u32 {
        let mut zx: f64 = 0.0;
        let mut zy: f64 = 0.0;

        let mut i: u32 = 0;
        loop {
            //Z(n+1) = Z(n)^2 + c
            let xt = zx * zy;
            zx = (zx * zx) - (zy * zy) + x;
            zy = 2.0 * xt + y;

            //this is pythagoreans theorum without square root because
            //ya know, power intensive
            //(sqrt of (zx^2 + zy^2)) > 2.0
            if (zx * zx) + (zy * zy) > 4.0 || i > self.num_iterations {
                break;
            }
            i += 1;
        }
        i
    }

    pub const fn point_is_in_set(&self, x: f64, y: f64) -> bool {
        let mut zx: f64 = 0.0;
        let mut zy: f64 = 0.0;

        let mut i: u32 = 0;
        loop {
            //Z(n+1) = Z(n)^2 + c
            let xt = zx * zy;
            zx = (zx * zx) - (zy * zy) + x;
            zy = 2.0 * xt + y;

            //this is pythagoreans theorum without square root because
            //ya know, power intensive
            //(sqrt of (zx^2 + zy^2)) > 2.0
            if (zx * zx) + (zy * zy) > 4.0 {
                return false;
            }

            if i > self.num_iterations {
                return true;
            }

            i += 1;
        }
    }
    pub const fn num_shades(&self) -> u32 {
        self.num_iterations.saturating_sub(self.begin_shade_at_n)
    }
    pub const fn get_color(&self, iteration_value: u32) -> Rgba {
        // subtract from the begin_shade value
        let modified_shade_value = iteration_value.saturating_sub(self.begin_shade_at_n);

        let percent_of_shade = (modified_shade_value as f64) / self.num_shades() as f64;

        match &self.shade {
            ShadingType::Opacity(opacity) => {
                Rgb::splat(255).with_alpha(opacity.get_opacity_u8(percent_of_shade))
            }
            ShadingType::OpacityColor { color, opacity } => color
                .get_color(percent_of_shade)
                .with_alpha(opacity.get_opacity_u8(percent_of_shade)),
            ShadingType::Color(color) => color.get_color(percent_of_shade).into_rgba(),
        }
    }
    pub fn get_point_color(&self, x: f64, y: f64) -> Rgba {
        let iter_count = self.point_in_set(x, y);
        self.get_color(iter_count)
    }
}

#[test]
fn test_coordinate_in_set() {
    let mandel = Mandelbrot::default();
    let x = -0.6;
    let in_set = mandel.point_is_in_set(x, 0.);

    assert!(in_set)
}

#[test]
fn test_coordinate_not_in_set() {
    let mandel = Mandelbrot::default();
    let x = 0.6;
    let in_set = mandel.point_is_in_set(x, 0.);
    assert!(!in_set);
}
