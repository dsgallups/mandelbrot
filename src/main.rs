use mandelbrot::{App, MBrotRaster, Mandelbrot, MandelbrotWindow};
use nalgebra::Vector2;

fn main() {
    println!("mandelbrot set gen");

    let window_width = 1920;
    let window_height = 1080;
    let mandelbrot = Mandelbrot::light_default();
    let window = App::new(window_width, window_height, mandelbrot);

    window.run();
}
