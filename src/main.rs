// #![allow(dead_code)]
// #![allow(unused_variables)]


/*
Author : Mark T
  Date : 6/21/2023

  Main file for running processes
*/

mod math;

extern crate image;

use hsv;

use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;

use crate::math::structs;

use clap::error::ErrorKind;
use clap::Parser;
use clap::CommandFactory;

// type GenDataType = f64;
type GenDataType = structs::Complex;

#[derive(Debug, Default)]
struct Config {
    count:                       u64, // Index of the generated image
    c_init: Option<structs::Complex>, // Initial C value for when swap_zc is used
    size_x:                      u32, // Sets Image Width
    size_y:                      u32, // Sets Image Height
    max_i:                       u64, // Sets Maximum Iterations for Generator
    gen_formula:              String, // Specifies Formula for Generator
}

static ABOUT_CLI_ARGS: &str = "
 ~ Kyros
A CLI tool for generating fractal images. 
";

#[derive(Parser, Debug)]
#[command(about=ABOUT_CLI_ARGS)]
#[command(version, long_about = None)]
struct Args {

    /// The amount of pixels to generate
    #[arg(short, long, default_value_t = 256, value_name="INT")]
    pixels: u32,

    /// The amount of iterations to run per pixel
    #[arg(long, default_value_t = 1024, value_name="INT")]
    iterations: u64,

    /// The generation function to use
    #[arg(short, long, default_value_t=("SD".to_string()), value_name="STR")] // The Compiler lies, parentheses are needed
    formula: String,

    /// Flag to confirm image generation
    #[arg(short, long, default_value_t = false)]
    y_confirm: bool,
}

/// Function for getting image from configuration and generator function. 
fn eval_function(config: &Config, generator_function: &dyn Fn(structs::Complex, structs::Complex) -> GenDataType) -> image::RgbImage {
    // Unpacks Image Configuration
    let size_x: u32 = config.size_x;
    let size_y: u32 = config.size_y;
    let max_i: u64 = config.max_i;
    let c_init: Option<structs::Complex> = config.c_init;
    
    let mut c = math::structs::Complex { real: 0f64, imaginary: 0f64, };
 
    let static_x_math_space_factor: f64 = 4.0 / (size_x as f64 - 1.0);
    let static_y_math_space_factor: f64 = 4.0 / (size_y as f64 - 1.0);

    let mut z: math::structs::Complex;

    // Sets Initial 'c' Value (If set)
    let is_julia: bool = match c_init {
        Some(value) => {
            c = value;
            true
        },
        None => false,
    };

    // Initializes Image Buffer
    let mut img = image::ImageBuffer::new(size_x, size_y);
    for (_x, _y, pixel) in img.enumerate_pixels_mut() {
        *pixel = image::Rgb([255, 255, 255]);
    }

    // Goes through each pixel
    for i in 0..size_y {
        for j in 0..size_x {

             // Sets Initial Z Value
            z = math::structs::Complex {
                real : static_x_math_space_factor * j as f64 - 2.0,
                imaginary : static_y_math_space_factor * i as f64 - 2.0,
            };

            if is_julia == false { c = z; }

            // Runs Math
            let mut iteration: u64 = 0;
            loop {
                if iteration == max_i { break; }
                if z.is_greater(2.0) { break; }
                z = generator_function(c, z);
                iteration += 1;
            };

            let z_output = iteration as f64;

            let pixel = img.get_pixel_mut(j, i);
            // Gets color value
            let out_rgb: (u8, u8, u8);

            if z_output == 0. {out_rgb = (255, 255, 255)}
            else if z_output == max_i as f64 {out_rgb = (0, 0, 0)}
            else {
                out_rgb = hsv::hsv_to_rgb(
                    ( 9f64 * z_output as f64 ) % 360.0,
                    1.0,
                    1.0,
                );
            };

            *pixel = image::Rgb([out_rgb.0, out_rgb.1, out_rgb.2]);
        }
        print!("\t {:.2}% | {} / {}\r", 100.0 * (i as f64 + 1.0) / size_y as f64, i+1, size_y);
    }
    println!();
    return img;
}


/// Main function of the program
fn main() {
    // Defines Initial Values
    let cli_args = Args::parse();

    if !cli_args.y_confirm {
        Args::command().error(
            ErrorKind::MissingRequiredArgument,
            "Use '-y' to generate image from configuration. Note a '.png' file will be created."
        ).exit();
    }

    let config = Config {
        count: 0,
        c_init: None,
        size_x: cli_args.pixels,
        size_y: cli_args.pixels,
        max_i: cli_args.iterations,
        gen_formula: cli_args.formula,
    };

    println!("{:?}", config);

    // Initializes generators into a hashmap
    let mut generators: HashMap<String, &dyn Fn(structs::Complex, structs::Complex) -> GenDataType> = HashMap::new();
    generators.insert("SD".to_string(),  &math::formula::SD);
    generators.insert("R".to_string(),   &math::formula::R);
    generators.insert("BS".to_string(),  &math::formula::BS);
    generators.insert("SYM".to_string(), &math::formula::SYM);

    let generator_function: &dyn Fn(structs::Complex, structs::Complex) -> GenDataType;

    generator_function = match generators.get(&config.gen_formula) {
        Some(function_found) => function_found,
        None => {
            Args::command().error(
                ErrorKind::InvalidValue,
                format!("Function generation method '{}' not found!", config.gen_formula)
            ).exit();
        }
    };

    // Sets the starting time
    let start_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs_f64();

    // Runs Config, gets 32 byte img object
    let img = eval_function(&config, generator_function);
    println!("Saving File!");
    img.save(format!("out#{:}.png", config.count)).unwrap();

    // Finished Timings
    let end_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs_f64();

    // Show Completion Message
    println!("[Finished in {:.2}s]", end_time - start_time);
    let _ = std::io::stdin().read_line(&mut String::new());
}
