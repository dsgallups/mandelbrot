use mandelbrot::{MBrotRaster, Mandelbrot, Window};
use nalgebra::Vector2;

fn main() {
    println!("Mandelbrot set generator");

    //let mandelbrot = MandelbrotSimple::new_full(10000);

    let mandelbrot = Mandelbrot::light_default();

    let plot_width = 4.;
    let plot_height = (1080. / 1920.) * plot_width;

    let window = Window::new(
        1920,
        Vector2::new(plot_width, plot_height),
        Vector2::zeros(),
        1.0,
    );

    let image = window.render(&mandelbrot);
    image.save_mandelbrot();

    //let res: RgbaVec = mandelbrot.rasterize();
    //println!("res length: {}", res.len());
    //mandelbrot.save::<DefaultImage>();
}
