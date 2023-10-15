/*
Author : Mark T
  Date : 6/21/2023

  Main file for running processes
*/

mod math;
mod color;
mod shadows;
mod utils;

extern crate image;

use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::color::get_color;
use crate::shadows::get_shadow;
use crate::math::formula::get_formula;

// Project Crates
use crate::math::structs;

// CLI Crates
use clap::Parser;

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
    shadow_formula:           String, // Specifies Formula for Shadows
    travel_distance:            bool, // Speifies if the output color value should be based on travel distance
    math_frame:            MathFrame,
}

/// Struct for factor & offset for math space
/// This is used to calculate where each pixel is mapped to
#[derive(Debug, Default)]
struct MathFrame {
    x_math_space_factor: f64,
    x_math_space_offset: f64,

    y_math_space_factor: f64,
    y_math_space_offset: f64,   
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

    /// Specifies shadow function to use
    #[arg(long, default_value_t=("none".to_string()), value_name="STR")]
    shadow: String,

    /// Uses Julia set style generation
    #[arg(short, long, default_value_t=false, value_name="BOOL")]
    julia: bool,

    /// Uses Travel Distance to color pixels
    #[arg(long, default_value_t=false, value_name="BOOL")]
    travel_distance: bool,

    /// Confirm image generation
    #[arg(short, long, required(true))]
    y_confirm: bool,
}

/// Function for getting image from configuration and generator function. 
fn eval_function(config: &Config) -> image::RgbImage {

    // Unpacks Image Configuration
    let c_init: Option<structs::Complex> = config.c_init;

    let generator_function = get_formula(&config.gen_formula.as_str());
    let color_function = get_color(&config.color_formula.as_str());
    let shadow_function = get_shadow(&config.shadow_formula.as_str());

    // Sets Initial 'c' Value (If set)
    let mut c = math::structs::Complex { real: 0f64, imaginary: 0f64, };
    let is_julia: bool = match c_init {
        Some(value) => {
            c = value;
            true
        },
        None => false,
    };

    // Sets Math Values
    let x_math_space_factor = config.math_frame.x_math_space_factor;
    let x_math_space_offset = config.math_frame.x_math_space_offset;

    let y_math_space_factor = config.math_frame.y_math_space_factor;
    let y_math_space_offset = config.math_frame.y_math_space_offset;

    let mut z: math::structs::Complex;
    let mut old_z: math::structs::Complex;

    // Initializes Image Buffer
    let mut img = image::ImageBuffer::new(config.size_x, config.size_y);

    // Goes through each pixel
    for i in 0..config.size_y {
        for j in 0..config.size_x {

             // Sets Initial Z Value
            z = math::structs::Complex {
                real      : x_math_space_factor * j as f64 - x_math_space_offset,
                imaginary : y_math_space_factor * i as f64 - y_math_space_offset,
            };
            old_z = z;

            if is_julia == false { c = z; }

            let mut z_output: f64 = 0.0;

            // Runs Math
            for iteration in 0..config.max_i {
                if iteration == config.max_i { break; }
                if z.is_greater(2.0) { break; }
                z = generator_function(c, z);

                // Calculates Output
                if !config.travel_distance {
                    z_output += 1.0;
                }
                else {
                    z_output += (
                        (z.real - old_z.real) * (z.real - old_z.real) +
                        (z.imaginary - old_z.imaginary) * (z.imaginary - old_z.imaginary)
                    ).sqrt();
                    old_z = z;
                }
            };

            if config.travel_distance {
                
            }

            // Gets pixel pointer
            let pixel = img.get_pixel_mut(j, i);

            // Sets Pixel Value
            let out_rgb: (u8, u8, u8);
            if z_output == 0.0 {out_rgb = (255, 255, 255)}
            else if z_output == config.max_i as f64 {out_rgb = (0, 0, 0)}
            else {
                out_rgb = hsv::hsv_to_rgb(
                    color_function(z_output).rem_euclid(360.0),
                    1.0,
                    shadow_function(z_output).rem_euclid(360.0)
                );
            };
            *pixel = image::Rgb([out_rgb.0, out_rgb.1, out_rgb.2]);
        }
        print!("\t {:.2}% | {} / {}\r", 100.0 * (i as f64 + 1.0) / config.size_y as f64, i+1, config.size_y);
    }
    println!();
    return img;
}


/// Main function of the program
fn main() {

    // env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_BACKTRACE", "full");

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
        shadow_formula: cli_args.shadow,
        travel_distance: cli_args.travel_distance,
        math_frame: MathFrame {
            x_math_space_factor: 4.0 / (cli_args.pixels as f64 - 1.0),
            x_math_space_offset: 2.0,
            y_math_space_factor: 4.0 / (cli_args.pixels as f64 - 1.0),
            y_math_space_offset: 2.0,
        }
    };

    if cli_args.julia {
        config.c_init = Some(structs::Complex {
            real: 0.08004012786314796,
            imaginary: -0.6359321976472476,
        });
    }

    // println!("{:?}", config);

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
