
use image::{ImageBuffer, Rgba, RgbaImage};
//use std::thread;
//use std::sync::{Mutex, Arc};

//Parameters
const WIDTH: usize = 2000;
const HEIGHT: usize = 2000;
const X_START: f64 = -2.0;
const X_END: f64 = 0.5;
const Y_START: f64 = -1.0;
const Y_END: f64 = 1.0;

const N_ITER: u32 = 40;
const BEGIN_SHADE_AT_N: u32 = 30;
const NUM_SHADES: u8 = 30;
const PATH: &str = "mandelbrot-new.png";

//const N_THREADS: u32 = 6;


//const DELTA_X: f64 = (X_END - X_START) / (WIDTH as f64);
//const DELTA_Y: f64 = (Y_END - Y_START) / (HEIGHT as f64);

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

    /*
    Started making multithreaded vers because bored
    //based on the number of threads, divide them into equal sections. We will
    //just make n columns based on n threads
    let column_size = plot.len() as u32 / N_THREADS;

    //yes, this does mean one thread will most likely outlast all the others. sux
    let mut handles = Vec::new();

    
    for i in 0..N_THREADS {
        let offset = column_size * i;
        let plot_y_len = plot[0].len();

        
        let handle = thread::spawn(move || {
            let mut plt: Vec<Vec<u32>> = Vec::new();

            for x in 0..column_size {
                let mut row: Vec<u32> = Vec::new();
                for y in 0..plot_y_len {
                    let point_x = X_START + (DELTA_X * ((x + offset) as f64));
                    let point_y = X_START + (DELTA_Y * (y as f64));
                    row.push(point_in_mandelbrot_set(point_x, point_y));
                }
                plt.push(row);
            }

            return (plt, offset);
        });

        handles.push(handle);
    }

    for handle in handles {
        let handle_plot = handle.join().unwrap();

    }
    */


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

fn create_image(plot: &mut Vec<Vec<u32>>, image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) {

    for x in 0..plot.len() {
        for y in 0..plot[x].len() {

            let v = plot[x][y];
            //then we want it to be white
            let max = N_ITER;
            let mut pixel_value = 255 - (((v as f64 / max as f64) * 255.0) as u8);
            
            //calculate the number of values in a shade
            let values_in_shade = 255 / NUM_SHADES;
            
            //run modulo of the pixel value, and subtract that from the pixel
            pixel_value = pixel_value - (pixel_value % values_in_shade);
            
            image.put_pixel(x as u32, y as u32, Rgba([pixel_value, pixel_value, pixel_value, 255 - pixel_value]));
       
       
       //create five groups of pixels. So
            //0-12 for example should have value of 0
            //13-26 should have value of 13

            /*
                example: 40 values, 10 shades
            
                0-3
                4-7
                9-12
                13-16
                ...
                36-40

                take a value of 15

                how do we know 3 is in shade 0

                40 values / 10 = 4

                4 % 4 = 0
                4 - 0  = 4

                15 % 4 = 3
                15 - 3 = 12

                40 % 4 = 0
                40 - 0 = 40

            */
        }
    }
}

fn save_image(mut img: ImageBuffer<Rgba<u8>, Vec<u8>>) {
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
    let mut image = RgbaImage::new(WIDTH as u32, HEIGHT as u32);
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
