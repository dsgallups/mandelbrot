mod opacity_config;
pub use opacity_config::*;
mod color_config;
pub use color_config::*;
mod shade_type;
pub use shade_type::*;
mod color;
pub use color::*;

pub struct ShadeConfig {
    shade: ShadingType,
    num_iterations: u32,
    begin_shade_at_n: u32,
}

impl ShadeConfig {
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

    pub const fn num_iterations(&self) -> u32 {
        self.num_iterations
    }

    pub const fn begin_shading_at_iteration(&self) -> u32 {
        self.begin_shade_at_n
    }

    pub const fn num_shades(&self) -> u32 {
        self.num_iterations.saturating_sub(self.begin_shade_at_n)
    }

    pub const fn get_color(&self, iteration_value: u32) -> Rgba {
        // subtract from the begin_shade value
        let modified_shade_value = iteration_value.saturating_sub(self.begin_shade_at_n);

        let percent_of_opacity = (modified_shade_value as f64) / self.num_shades() as f64;

        match &self.shade {
            ShadingType::Opacity(opacity) => {
                Rgb::splat(0).with_alpha((opacity.get_opacity(percent_of_opacity) * 255.) as u8)
            }
            ShadingType::OpacityColor { .. } => {
                //
                todo!();
                //
            }
            ShadingType::Color(_) => {
                //
                todo!();
                //
            }
        };

        todo!()
    }
}
