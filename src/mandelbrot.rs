use nalgebra::Vector2;

use crate::{MBrotRaster, PlotConfig, ShadeConfig, Window};

pub struct Mandelbrot {
    shade: ShadeConfig,
    plot: PlotConfig,
    window: Window,
}

impl Default for Mandelbrot {
    fn default() -> Self {
        let bottom_left = Vector2::new(-2., -1.2);
        let top_right = Vector2::new(0.5, 1.2);
        let image_width = 1920;
        let shade = ShadeConfig::light_default();

        Self::new_for_scaled_image(top_right, bottom_left, image_width, shade)
    }
}

impl Mandelbrot {
    pub fn sandbox_default() -> Self {
        let window = Window::new_from_dims(Vector2::new(4., 8.), Vector2::zeros(), 1.0);
        let plot_config = PlotConfig::new(1920, 1080);
        let shade = ShadeConfig::light_default();

        Self::new(window, plot_config, shade)
    }

    pub fn new_full(image_width: u32) -> Self {
        let bottom_left = Vector2::new(-2., -1.2);
        let top_right = Vector2::new(0.5, 1.2);
        let shade = ShadeConfig::light_default();

        Self::new_for_scaled_image(top_right, bottom_left, image_width, shade)
    }

    #[cfg(test)]
    pub fn test() -> Self {
        let bottom_left = Vector2::new(-2., -1.2);
        let top_right = Vector2::new(0.5, 1.2);
        let image_width = 1000;
        let shade = ShadeConfig::light_default();

        Self::new_for_scaled_image(top_right, bottom_left, image_width, shade)
    }

    pub fn num_iterations(&self) -> u32 {
        self.shade.num_iterations()
    }

    /// scales image height proportionally according to the dimensions of the graph
    pub fn new_for_scaled_image(
        graph_top_right: Vector2<f64>,
        graph_bottom_left: Vector2<f64>,
        image_width: u32,
        shade_config: ShadeConfig,
    ) -> Self {
        let window = Window::new(
            graph_top_right.y,
            graph_top_right.x,
            graph_bottom_left.y,
            graph_bottom_left.x,
        );

        let height_per_width =
            (graph_top_right.y - graph_bottom_left.y) / (graph_top_right.x - graph_bottom_left.x);

        let image_height = image_width as f64 * height_per_width;
        let plot = PlotConfig::new(image_width, image_height as u32);

        Self {
            window,
            plot,
            shade: shade_config,
        }
    }

    pub fn new(window: Window, plot: PlotConfig, shade_config: ShadeConfig) -> Self {
        Self {
            window,
            plot,
            shade: shade_config,
        }
    }

    pub fn new_windowed(
        dimensions: Vector2<f64>,
        transform: Vector2<f64>,
        zoom: f64,
        shade_config: ShadeConfig,
    ) -> Self {
        Self {
            window: Window::new_from_dims(dimensions, transform, zoom),
            plot: PlotConfig::new(dimensions.x as u32, dimensions.y as u32),
            shade: shade_config,
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
            if (zx * zx) + (zy * zy) > 4.0 || i > self.shade.num_iterations() {
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

            if i > self.shade.num_iterations() {
                return true;
            }

            i += 1;
        }
    }

    /// translates a point in the plot to a point in the mandelbrot set
    fn translate_plot_point(&self, x: u32, y: u32) -> (f64, f64) {
        let (perc_x, perc_y) = self.plot.percent_from_bottom_left(x, y);
        self.window.get_pixel_from_percent(perc_x, perc_y)
    }

    /// returns the value of a point in the plot based on the window.
    pub fn point_in_plot(&self, x: u32, y: u32) -> u32 {
        let (x, y) = self.translate_plot_point(x, y);
        self.point_in_set(x, y)
    }

    pub fn rasterize<R: MBrotRaster>(&self) -> R {
        let height = self.plot.height();
        let mut raster = R::init_raster(self.plot.width(), height);

        let get_point_in_plot = |x, y| {
            let pixel_iteration_count = self.point_in_plot(x, y);
            self.shade.get_color(pixel_iteration_count)
        };

        raster.draw_mandelbrot(get_point_in_plot);

        raster
    }

    pub fn save<R: MBrotRaster>(&self) {
        let raster: R = self.rasterize();
        raster.save_mandelbrot();
    }
}

#[test]
fn test_coordinate_in_set() {
    let mandel = Mandelbrot::test();
    let x = -0.6;
    let in_set = mandel.point_is_in_set(x, 0.);

    assert!(in_set)
}

#[test]
fn test_coordinate_not_in_set() {
    let mandel = Mandelbrot::test();
    let x = 0.6;
    let in_set = mandel.point_is_in_set(x, 0.);
    assert!(!in_set);
}

#[test]
fn test_coordinate_map() {
    // 1000 width
    let img_x = 1000;
    let mandel = Mandelbrot::new_full(img_x);
    let (perc_x, _) = mandel.plot.percent_from_bottom_left(img_x, 0);

    assert_eq!(perc_x, 1.);

    let (x, _) = mandel.translate_plot_point(img_x, 0);

    assert_eq!(x, 0.5);

    let (perc_x, _) = mandel.plot.percent_from_bottom_left(0, 0);

    assert_eq!(perc_x, 0.);
}
