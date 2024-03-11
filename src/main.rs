/*
Author : Mark T
  Date : 6/21/2023

  Main file for running processes
*/

// Project Crates
mod math;
mod structs;
mod colors;
mod utils;
mod cli;
mod save;

use crate::structs::{Complex, Config, MathFrame};
use crate::cli::Args;

use crate::colors::color::get_color;
use crate::colors::shadows::get_shadow;
use crate::math::formula::get_formula;
use crate::save::get_save_method;

use crate::utils::eval_function;

// std imports
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

// External Imports
extern crate image;

// External Crates
use clap::Parser;

/// Main function of the program
fn main() {

    // env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_BACKTRACE", "full");

    // Defines values from CLI arguments
    let cli_args = Args::parse();

    let mut config = Config {
        c_init: None,

        size_x: cli_args.pixels,
        size_y: cli_args.pixels,

        max_i: cli_args.iterations,

        gen_formula: cli_args.formula,
        color_formula: cli_args.color,
        rate_of_color_change: cli_args.rate_of_color_change,
        shadow_formula: cli_args.shadow,
        background: cli_args.background,
        
        rgba: cli_args.rgba,
        travel_distance: cli_args.travel_distance,

        save_method: cli_args.save_method,
        filename: cli_args.filename,

        math_frame: MathFrame {
            factor_x: cli_args.factor_x / (cli_args.pixels as f64 - 1.0),
            factor_y: cli_args.factor_y / (cli_args.pixels as f64 - 1.0),
            offset_x: cli_args.offset_x,
            offset_y: cli_args.offset_y,
        },
        progress: cli_args.progress,
    };

    if cli_args.julia {
        config.c_init = Some(Complex {
            real: cli_args.c_real,
            imaginary: cli_args.c_imaginary,
        });
    }

    // Sets the starting time
    let start_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs_f64();

    // Sets the save method before generation (For ensuring this is tested before the image is
    // generated)
    let save_method = get_save_method(&config.save_method.as_str());

    // Runs Config, gets 32 byte img object
    let img = eval_function(&config);
    if config.progress {
        println!("Saving File...");
    }

    // Saves Image
    let _ = save_method.method(image::DynamicImage::ImageRgb8(img), &config).unwrap();

    // img.save(format!("out#{}.png", config.count)).unwrap();

    // Finished Timings
    let end_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs_f64();

    // Show Completion Message
    if config.progress {
        println!("[Finished in {:.2}s]", end_time - start_time);
    }
    // let _ = std::io::stdin().read_line(&mut String::new());
}
