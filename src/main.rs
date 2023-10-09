/*
Author : Mark T
  Date : 6/21/2023

  Main file for running processes
*/

mod math;

extern crate image;

use std::mem::size_of;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::color::get_color;
use crate::math::formula::get_formula;
// Project Crates
use crate::math::structs;
mod color;

// CLI Crates
use clap::Command;
use clap::Arg;
use clap::ArgAction;
use clap::error::ErrorKind;
use clap::Parser;
use clap::CommandFactory;
// use clap::Subcommand;

/// Main object for defining generation configuration. 
#[derive(Debug, Default)]
struct Config {
    count:                       u64, // Index of the generated image
    c_init: Option<structs::Complex>, // Initial C value for when swap_zc is used
    size_x:                      u32, // Sets Image Width
    size_y:                      u32, // Sets Image Height
    max_i:                       u64, // Sets Maximum Iterations for Generator
    gen_formula:              String, // Specifies Formula for Generator
    color_formula:            String, // Specifies Formula for Colors
    math_frame:            MathFrame,
}

/// Struct for factor & offset for math space
/// This is used to calculate where each pixel is mapped to
#[derive(Debug, Default)]
struct MathFrame {
    static_x_math_space_factor: f64,
    static_x_math_space_offset: f64,

    static_y_math_space_factor: f64,
    static_y_math_space_offset: f64,   
}

static ABOUT_CLI_ARGS: &str = "
 ~ Kyros
A CLI tool for generating fractal images. 
";


const LONG_ABOUT_CLI_ARGS: &str = "
 ~ Kyros
A CLI tool for generating fractal images. 
Woah this is a long description!
";

#[derive(Parser, Debug)]
#[command(about=ABOUT_CLI_ARGS)]
#[command(long_about=LONG_ABOUT_CLI_ARGS)]
#[command(version)]
struct Args {

    /// The amount of pixels to generate
    #[arg(short, long, default_value_t = 256, value_name="INT")]
    pixels: u32,

    /// The amount of iterations to run per pixel
    #[arg(short, long, default_value_t = 1024, value_name="INT")]
    iterations: u64,

    /// The generation function to use
    #[arg(short, long, default_value_t=("SD".to_string()), value_name="STR", long_help="Sets the generation function to use. \nSet this value to 'HELP' for more information.")] // The Compiler lies, parentheses are needed
    formula: String,

    /// Specifies color function to use
    #[arg(long, default_value_t=("ROTATIONAL".to_string()), value_name="STR")]
    color: String,

    /// Uses Julia set style generation
    #[arg(short, long, default_value_t=false, value_name="BOOL")]
    julia: bool,

    /// Confirm image generation
    #[arg(short, long, required(true))]
    y_confirm: bool,
}

/// Function for getting image from configuration and generator function. 
fn eval_function(config: &Config) -> image::RgbImage {

    // Unpacks Image Configuration
    let size_x: u32 = config.size_x;
    let size_y: u32 = config.size_y;
    let max_i: u64 = config.max_i;
    let c_init: Option<structs::Complex> = config.c_init;
    let generator_function = get_formula(&config.gen_formula.as_str());
    let color_function = get_color(&config.color_formula.as_str());

    // Sets Initial 'c' Value (If set)
    let mut c = math::structs::Complex { real: 0f64, imaginary: 0f64, };
    let is_julia: bool = match c_init {
        Some(value) => {
            c = value;
            true
        },
        None => false,
    };
 
    let static_x_math_space_factor = config.math_frame.static_x_math_space_factor;
    let static_x_math_space_offset = config.math_frame.static_x_math_space_offset;

    let static_y_math_space_factor = config.math_frame.static_y_math_space_factor;
    let static_y_math_space_offset = config.math_frame.static_y_math_space_offset;

    let mut z: math::structs::Complex;

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
                real      : static_x_math_space_factor * j as f64 - static_x_math_space_offset,
                imaginary : static_y_math_space_factor * i as f64 - static_y_math_space_offset,
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
            // let out_rgb: (u8, u8, u8);

            let out_rgb = color_function(z_output, max_i);

            *pixel = image::Rgb([out_rgb.0, out_rgb.1, out_rgb.2]);
        }
        print!("\t {:.2}% | {} / {}\r", 100.0 * (i as f64 + 1.0) / size_y as f64, i+1, size_y);
    }
    println!();
    return img;
}


/// Main function of the program
fn main() {

    // Defines values from CLI arguments
    let cli_args = Args::parse();

    let mut config = Config {
        count: 0,
        c_init: None,
        size_x: cli_args.pixels,
        size_y: cli_args.pixels,
        max_i: cli_args.iterations,
        gen_formula: cli_args.formula,
        color_formula: cli_args.color,
        math_frame: MathFrame {
            static_x_math_space_factor: 4.0 / (size_x as f64 - 1.0),
            static_x_math_space_offset: 2.0,
            static_y_math_space_factor: 4.0 / (size_y as f64 - 1.0),
            static_y_math_space_offset: 2.0,
        }
    };

    if cli_args.julia {
        config.c_init = Some(structs::Complex {
            real: 0.08004012786314796,
            imaginary: -0.6359321976472476,
        });
    }


    println!("{:?}", config);

    // Sets the starting time
    let start_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs_f64();

    // Runs Config, gets 32 byte img object
    let img = eval_function(&config);
    println!("Saving File...");
    img.save(format!("out#{}.png", config.count)).unwrap();

    // Finished Timings
    let end_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs_f64();

    // Show Completion Message
    println!("[Finished in {:.2}s]", end_time - start_time);
    let _ = std::io::stdin().read_line(&mut String::new());
}
