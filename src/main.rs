/*
Author : Mark T
  Date : 6/21/2023

  Main file for running processes
*/

// External Imports
extern crate image;
extern crate csscolorparser;
extern crate log;

// Project Crates
mod math;
mod structs;
mod colors;
mod utils;
mod cli;
mod save;
mod gpu;

use crate::structs::{Complex, Config, MathFrame};
use crate::cli::Args;

use crate::colors::color::get_color;
use crate::colors::shadows::get_shadow;
use crate::math::formula::get_formula;
use crate::save::get_save_method;

// std imports
use std::env;
use std::time::Instant;

use log::{warn, Level, Metadata, Record};

// External Crates
use clap::Parser;

static LOGGER: Logger = Logger;

struct Logger;

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}


/// Main function of the program
fn main() {

    log::set_logger(&LOGGER).unwrap();

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

        background: csscolorparser::parse(&cli_args.background.as_str()).unwrap(),
        foreground: csscolorparser::parse(&cli_args.foreground.as_str()).unwrap(),

        // rgba: cli_args.rgba,
        travel_distance: cli_args.travel_distance,

        save_method: cli_args.save_method,
        filename: cli_args.filename,

        rgba: cli_args.rgba | cli_args.gpu, // forces rgba if using gpu
        gpu: cli_args.gpu,
        chunk_sizes: cli_args.chunk_sizes,
        compression_threads: cli_args.compression_threads,

        math_frame: MathFrame {
            factor_x: (cli_args.factor_x / (cli_args.pixels as f64 - 1.0)) as f32,
            factor_y: (cli_args.factor_y / (cli_args.pixels as f64 - 1.0)) as f32,
            offset_x: cli_args.offset_x as f32,
            offset_y: cli_args.offset_y as f32,
        },
        logs: cli_args.logs,
    };

    log::set_max_level(cli_args.logs);

    if cli_args.julia {
        config.c_init = Some(Complex {
            real: cli_args.c_real as f32,
            imaginary: cli_args.c_imaginary as f32,
        });
    }

    let now = Instant::now();

    // Sets the save method before generation (For ensuring this is tested before the image is
    // generated)

    // Runs Config
    let res = match config.gpu {
        true => utils::gpu_eval(&config),
        false => utils::cpu_eval(&config),
    };

    if let Err(e) = res {
        warn!("Error Occurred in function evaluation: {:?}", e);
    }

    // Show Completion Message
    log::info!("[Finished in {:.2?}]", now.elapsed());
}
