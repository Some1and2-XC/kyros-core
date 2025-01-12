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
use std::fs::File;
use std::time::Instant;

use log::{warn, Level, Metadata, Record};

// External Crates
use clap::Parser;
use png::Decoder;

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
#[tokio::main]
async fn main() {

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
        read_config: cli_args.read_config,
        chunk_size: cli_args.chunk_size.unwrap_or(cli_args.pixels as u64),
        compression_threads: cli_args.compression_threads,
        compression: cli_args.compression,

        math_frame: MathFrame {
            factor_x: (cli_args.factor_x / (cli_args.pixels as f64 - 1.0)) as f32,
            factor_y: (cli_args.factor_y / (cli_args.pixels as f64 - 1.0)) as f32,
            offset_x: cli_args.offset_x as f32,
            offset_y: cli_args.offset_y as f32,
        },
        logs: cli_args.logs,
    };

    if config.read_config {

        let filename = format!("{}.png", &config.filename);

        let decoder = Decoder::new(File::open(&filename).expect(&format!("Can't open file: `{}`", &filename)));
        let meta_reader = decoder.read_info().expect("Can't parse PNG headers!");
        let info = meta_reader.info();

        let width = info.width;
        let height = info.height;
        let bit_depth = info.bit_depth;
        let color_type = info.color_type;
        let compressed_text = info.compressed_latin1_text.iter();
        let mut compressed_config_txt = compressed_text
            .filter(|val| &val.keyword == "kyros_config")
            .collect::<Vec<_>>()
            .first()
            .expect("Can't find compressed chunk labeled `kyros_config`!")
            .to_owned().to_owned()
            ;

        compressed_config_txt.decompress_text().expect("Can't decompress image config!");

        println!("Width      : {}px", width);
        println!("Height     : {}px", height);
        println!("Bit Depth  : {:?}", bit_depth);
        println!("Color Type : {:?}", color_type);
        println!("\"kyros_config\": {}", compressed_config_txt.get_text().expect("Failed to read decompressed config!"));

        return;
    }

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
        true => utils::gpu_eval(&config).await,
        false => utils::cpu_eval(&config),
    };

    if let Err(e) = res {
        warn!("Error Occurred in function evaluation: {:?}", e);
    }

    // Show Completion Message
    log::info!("[Finished in {:.2?}]", now.elapsed());
}
