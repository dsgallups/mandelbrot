
use image::{ImageBuffer, Rgb, RgbImage};

//Parameters
const WIDTH: usize = 64000;
const HEIGHT: usize = 64000;
const X_START: f64 = -2.0;
const X_END: f64 = 0.5;
const Y_START: f64 = -1.0;
const Y_END: f64 = 1.0;
const N_ITER: u32 = 255;
const PATH: &str = "mandelbrot-testing.png";

fn init_plot() -> Vec<Vec<u32>> {
    let mut r: Vec<Vec<u32>> = Vec::new();

    for _ in 0..WIDTH {
        let mut h: Vec<u32> = Vec::new();

        for _ in 0..HEIGHT {
            h.push(0);
        }
        
        r.push(h);
    }

    r
}

fn point_in_mandelbrot_set(x: f64, y: f64) -> u32 {

    let mut zx: f64 = 0.0;
    let mut zy: f64 = 0.0;

    let mut i: u32 = 0;
    loop {

        //Z(n+1) = Z(n)^2 + c
        let xt = zx*zy;
        zx = (zx * zx) - (zy * zy) + x;
        zy = 2.0 * xt + y;

        //this is pythagoreans theorum without square root because
        //ya know, power intensive
        //(sqrt of (zx^2 + zy^2)) > 2.0
        //So if 
        if (zx*zx) + (zy*zy) > 4.0 || i > N_ITER {
            /*if (i >= 1) {
                println!("Val on break: {}", (zx*zx) + (zy*zy));
                println!("i = {}", i);
            }*/
            break;
        }
        i += 1;
    }

    return i;
}

fn insert_mandelbrot(plot: &mut Vec<Vec<u32>>) {
    //So we loop through the plot. we calculate the 
    //point at each plot based on x_start and x_end
    let delta_x = (X_END - X_START) / (WIDTH as f64);
    let delta_y = (Y_END - Y_START) / (HEIGHT as f64);

    for x in 0..plot.len() {
        for y in 0..plot[x].len() {
            
            //so we get x and y
            //point 0 is -2.0
            //point 1 is -2.0 + delta_x
            //point 2 is -2.0 + delta_x * 2
            let point_x = X_START + (delta_x * (x as f64));
            let point_y = Y_START + (delta_y * (y as f64));

            plot[x][y] = point_in_mandelbrot_set(point_x, point_y);
            //println!("({}, {})", point_x, point_y);

        }
    }

}

fn create_image(plot: &mut Vec<Vec<u32>>, image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {

    for x in 0..plot.len() {
        for y in 0..plot[x].len() {

            let mut v = plot[x][y];
            //then we want it to be white
            let max = N_ITER;
            let pixel_value = 255 - (((v as f64 / max as f64) * 255.0) as u8);
            
            image.put_pixel(x as u32, y as u32, Rgb([pixel_value, pixel_value, pixel_value]));
        }
    }
}

fn save_image(mut img: ImageBuffer<Rgb<u8>, Vec<u8>>) {

    img.put_pixel(30, 30, Rgb([255,0,0]));

    img.save(PATH).unwrap();
}
fn main() {
    println!("Hello, world!");

    //initialize the plot
    let mut plot = init_plot();

    println!("Plot generated");

    println!("Plot Length - (WIDTH, HEIGHT) = ({}, {})", plot.len(), plot[0].len());

    //now draw the mandelbrot set on the plot
    insert_mandelbrot(&mut plot);
    println!("Mandelbrot points inserted!");

    //now create an image based on the plot
    let mut image = RgbImage::new(WIDTH as u32, HEIGHT as u32);
    println!("Image initialized!");
    
    create_image(&mut plot, &mut image);
    println!("Image created!");

    save_image(image);
    println!("Image saved!");



    /*
    so we need to build this mandelbrot set and turn it into an SVG

    start by making a function that determines if a point is in the mandelbrot set
    
    then you basically make iterations of points. you have
    X, Y start and X,Y end that define this map
    then the specificity of each point is defined by
    the delta of (x_end - x_start)/WIDTH and (y_end - y_start)/HEIGHT

    */
}
