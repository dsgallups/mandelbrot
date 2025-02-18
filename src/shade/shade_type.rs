use super::{ColorConfig, OpacityConfig};

pub enum ShadingType {
    OpacityColor {
        color: ColorConfig,
        opacity: OpacityConfig,
    },
    Opacity(OpacityConfig),
    Color(ColorConfig),
}

impl ShadingType {
    /// See [`OpacityConfig::light`]
    pub fn light_opacity(min_opacity: f64, max_opacity: f64) -> Self {
        Self::Opacity(OpacityConfig::light(min_opacity, max_opacity))
    }
    /// See [`OpacityConfig::dark`]
    pub fn dark_opacity(min_opacity: f64, max_opacity: f64) -> Self {
        Self::Opacity(OpacityConfig::dark(min_opacity, max_opacity))
    }
    pub const fn opacity(opacity_config: OpacityConfig) -> Self {
        Self::Opacity(opacity_config)
    }
}
