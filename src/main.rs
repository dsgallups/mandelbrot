use image::{ImageBuffer, Rgba, RgbaImage};
use chrono::Utc;
//use std::thread;
//use std::sync::{Mutex, Arc};


#[allow(dead_code)]
enum ShadingType {
    OpacityAndColor,
    OpacityOnly,
    ColorOnly
}

//Important parameters
const WIDTH: usize = 1920*4;
const LIGHT: bool = true;
const SHADING_TYPE: ShadingType = ShadingType::OpacityOnly;


const PLOT_ZOOM: f64 = 1.0;

//note these are the X,Y coordinates for the graph, not the final image. The coordinates have been otherwise flipped.
const PLOT_TRANSFORM_X: f64 = 0.0;
const PLOT_TRANSFORM_Y: f64 = 0.0;
const X_START: f64 = (-2.0 * PLOT_ZOOM) + PLOT_TRANSFORM_X;
const X_END: f64 = (0.5 * PLOT_ZOOM) + PLOT_TRANSFORM_X;
const Y_START: f64 = (-1.2 * PLOT_ZOOM) + PLOT_TRANSFORM_Y;
const Y_END: f64 = (1.2 * PLOT_ZOOM) + PLOT_TRANSFORM_Y;
const HEIGHT: usize = (((X_END - X_START) / (Y_END - Y_START)) * WIDTH as f64) as usize;

const N_ITER: u32 = 255;
const BEGIN_SHADE_AT_N: u32 = 10;
const NUM_SHADES: u8 = 5;
const FIRST_SHADE_VAL_IFN_LIGHT: u8 = 20;   //shades grow upward (0 -> 255)
const FIRST_SHADE_VAL_IF_LIGHT: u8 = 255;   //shades grow downward (255 -> 0)
const PATH: &str = "mandelbrot_at_";

//const N_THREADS: u32 = 6;
//const DELTA_X: f64 = (X_END - X_START) / (HEIGHT as f64);
//const DELTA_Y: f64 = (Y_END - Y_START) / (WIDTH as f64);

fn init_plot() -> Vec<Vec<u32>> {
    let mut r: Vec<Vec<u32>> = Vec::new();

    for _ in 0..HEIGHT {
        let mut h: Vec<u32> = Vec::new();

        for _ in 0..WIDTH {
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
    let delta_x = (X_END - X_START) / (HEIGHT as f64);
    let delta_y = (Y_END - Y_START) / (WIDTH as f64);

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

fn create_image(plot: &Vec<Vec<u32>>) -> ImageBuffer<Rgba<u8>, Vec<u8>> {

    let mut image = RgbaImage::new(WIDTH as u32, HEIGHT as u32);
    for x in 0..plot.len() {
        for y in 0..plot[x].len() {

            let pixel_iteration_count = plot[x][y];
            //then we want it to be white

            //apply the value in which to start shading
            let pixel_modified_count: i32 = pixel_iteration_count as i32 - BEGIN_SHADE_AT_N as i32;
        
            let max = 1 + N_ITER - BEGIN_SHADE_AT_N;

            //note: HEIGHT - x - 1 is to rotate image around the x axis
            if pixel_modified_count < 0 {
                image.put_pixel(y as u32, HEIGHT as u32 - x as u32 - 1, Rgba([0, 0, 0, 0]))
            } else {
                
                let mut pixel_value = ((pixel_modified_count as f64 / max as f64) * (FIRST_SHADE_VAL_IF_LIGHT as f64)) as u8;
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
                    pixel_value = FIRST_SHADE_VAL_IFN_LIGHT + ((pixel_modified_count as f64 / max as f64) * (255.0 - FIRST_SHADE_VAL_IFN_LIGHT as f64)) as u8;
                    pixel_value = 255 - pixel_value;
                }
                
                //calculate the number of values in a shade
                let values_in_shade = 255 / NUM_SHADES;
                
                //run modulo of the pixel value, and subtract that from the pixel
                pixel_value = pixel_value - (pixel_value % values_in_shade);
                
                match SHADING_TYPE {
                    ShadingType::ColorOnly =>  image.put_pixel(y as u32, HEIGHT as u32 - x as u32 - 1, Rgba([pixel_value, pixel_value, pixel_value, 255])),
                    ShadingType::OpacityOnly => if LIGHT 
                        {image.put_pixel(y as u32, HEIGHT as u32 - x as u32 - 1, Rgba([255, 255, 255, opacity]))} else 
                        {image.put_pixel(y as u32, HEIGHT as u32 - x as u32 - 1, Rgba([0, 0, 0, opacity]))},
                    ShadingType::OpacityAndColor => image.put_pixel(y as u32, HEIGHT as u32 - x as u32 - 1, Rgba([pixel_value, pixel_value, pixel_value, opacity]))
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
       
        }
    }

    image
}

fn save_image(img: ImageBuffer<Rgba<u8>, Vec<u8>>) {
    let now = Utc::now();
    let current_time = now.format("%y-%m-%d-%H%M%S").to_string();
    let mut new_path = String::from(PATH);
    new_path.push_str(&current_time);
    new_path.push_str(".png");
    println!("Saving image to {}.", new_path);
    img.save(&new_path).unwrap();
}

fn main() {
    println!("Hello, world!");
    println!("HEIGHT = {}", HEIGHT);

    //initialize the plot
    let mut plot = init_plot();

    println!("Plot generated");

    println!("Plot Length - (HEIGHT, WIDTH) = ({}, {})", plot.len(), plot[0].len());

    //now draw the mandelbrot set on the plot
    insert_mandelbrot(&mut plot);
    println!("Mandelbrot points inserted!");

    //now create an image based on the plot    
    let image = create_image(&plot);
    println!("Image created!");

    save_image(image);
    println!("Image saved!");
    println!("---------------------------------------");
    println!("Image Statistics: ");
    println!("Top Left Corner:      ({:.4}, {:.4})", X_END, Y_END);
    println!("Bottom Right Corner:  ({:.4}, {:.4})", X_START, Y_START);
    println!("Zoom:                 ({:.4})", PLOT_ZOOM);



    /*
    so we need to build this mandelbrot set and turn it into an SVG

    start by making a function that determines if a point is in the mandelbrot set
    
    then you basically make iterations of points. you have
    X, Y start and X,Y end that define this map
    then the specificity of each point is defined by
    the delta of (x_end - x_start)/HEIGHT and (y_end - y_start)/WIDTH

    */
}
