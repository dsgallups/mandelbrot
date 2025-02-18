use mandelbrot::Mandelbrot;

fn main() {
    println!("Mandelbrot set generator");

    let mandelbrot = Mandelbrot::default();

    mandelbrot.save();
}
