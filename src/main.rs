use mandelbrot::{DefaultImage, Mandelbrot};

fn main() {
    println!("Mandelbrot set generator");

    let mandelbrot = Mandelbrot::new_full(10000);

    //let res: RgbaVec = mandelbrot.rasterize();
    //println!("res length: {}", res.len());
    mandelbrot.save::<DefaultImage>();
}
