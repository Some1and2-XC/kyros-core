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

use crate::structs::{Complex, Config, MathFrame};
use crate::cli::Args;

use crate::colors::color::get_color;
use crate::colors::shadows::get_shadow;
use crate::math::formula::get_formula;

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
        },
        progress: cli_args.progress,
    };

    if cli_args.julia {
        config.c_init = Some(Complex {
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
    if config.progress {
        println!("Saving File...");
    }
    img.save(format!("out#{}.png", config.count)).unwrap();

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
