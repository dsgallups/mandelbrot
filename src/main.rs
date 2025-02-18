use mandelbrot::{App, MBrotRaster, Mandelbrot, MandelbrotWindow};
use nalgebra::Vector2;

fn main() {
    println!("mandelbrot set gen");

    let window_width = 640;
    let window_height = 480;
    let mandelbrot = Mandelbrot::red_opacity();
    let window = App::new(window_width, window_height, mandelbrot);

    window.run();
}
