use image::{ImageBuffer, Rgba, RgbaImage};
use rayon::iter::ParallelIterator;

mod shade;
pub use shade::*;
mod plot;
pub use plot::*;

#[allow(dead_code)]
mod old_main;

pub struct Mandelbrot {
    shade: ShadeConfig,
    plot: PlotConfig,
}
impl Mandelbrot {
    pub fn new(plot_config: PlotConfig, shade_config: ShadeConfig) -> Self {
        Self {
            plot: plot_config,
            shade: shade_config,
        }
    }
    pub fn point_in_graph(&self, x: f64, y: f64) -> u32 {
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
            if (zx * zx) + (zy * zy) > 4.0 || i > self.shade.n_iter {
                break;
            }
            i += 1;
        }
        i
    }
    pub fn rasterize(&self) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let mut image = RgbaImage::new(self.plot.width, self.plot.height);
        // for (x, row) in plot.iter().enumerate() {
        //     row.par_iter().enumerate().for_each(|(y, cell)| {});
        // }

        image.par_enumerate_pixels_mut().for_each(|(x, y, pixel)| {
            /*


            */
            let pixel_iteration_count = self.point_in_graph(x as f64, y as f64);
            //then we want it to be white

            //apply the value in which to start shading
            let pixel_modified_count: i32 =
                pixel_iteration_count as i32 - self.shade.begin_shading_at_iteration() as i32;

            let max = 1 + self.shade.num_iterations() - self.shade.begin_shading_at_iteration();

            //note: HEIGHT - x - 1 is to rotate image around the x axis
            if pixel_modified_count < 0 {
                *pixel = Rgba([0, 0, 0, 0]);
            } else {
                let mut pixel_value = ((pixel_modified_count as f64 / max as f64)
                    * (FIRST_SHADE_VAL_IF_LIGHT as f64))
                    as u8;
                /*if pixel_value == 0 {
                    println!("------------------------------------------------");
                    println!("pixel value:              {}", pixel_value);
                    println!("pixel_iteration_count:    {}", pixel_iteration_count);
                    println!("BEGIN_SHADE_AT_N:         {}", BEGIN_SHADE_AT_N);
                    println!("pixel_modified_count:     {}", pixel_modified_count);
                    println!("max:                      {}", max);
                    println!("FIRST_SHADE_VAL_IF_LIGHT: {}", FIRST_SHADE_VAL_IF_LIGHT);

                }*/

                let opacity = pixel_value;

                if !LIGHT {
                    pixel_value = FIRST_SHADE_VAL_IFN_LIGHT
                        + ((pixel_modified_count as f64 / max as f64)
                            * (255.0 - FIRST_SHADE_VAL_IFN_LIGHT as f64))
                            as u8;
                    pixel_value = 255 - pixel_value;
                }

                //calculate the number of values in a shade
                let values_in_shade = 255 / NUM_SHADES;

                //run modulo of the pixel value, and subtract that from the pixel
                pixel_value = pixel_value - (pixel_value % values_in_shade);

                match SHADING_TYPE {
                    ShadeConfig::ColorOnly => {
                        *pixel = Rgba([pixel_value, pixel_value, pixel_value, 255]);
                    }
                    ShadeConfig::OpacityOnly => {
                        if LIGHT {
                            *pixel = Rgba([255, 255, 255, opacity]);
                        } else {
                            *pixel = Rgba([0, 0, 0, opacity]);
                        }
                    }
                    ShadeConfig::OpacityAndColor => {
                        *pixel = Rgba([pixel_value, pixel_value, pixel_value, opacity]);
                    }
                }
                //image.put_pixel(y as u32, HEIGHT as u32 - x as u32 - 1, Rgba([pixel_value, pixel_value, pixel_value, opacity]));
                //image.put_pixel(y as u32, HEIGHT as u32 - x as u32 - 1, Rgba([0, 0, 0, opacity]));
            }
            /*

                So now we need to calculate the pixel value given that there is a starting pixel value
                let's say that the iteration count is 10
                for a total iteration of 13
                and the starting pixel value is 85
                of course, the highest value of a pixel is 255

                so the equation will be
                85 + ((255-85) * 10/13)



            */
        });

        image
    }
}
