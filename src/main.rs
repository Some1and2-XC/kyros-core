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
use std::env::args;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;
use crate::math::structs;

type GenDataType = f64;

#[derive(Debug, Default)]
struct Config {
    count:                       u64, // Index of the generated image
    c_init: Option<structs::Complex>, // Initial C value for when swap_zc is used
    size_x:                      u32, // Sets Image Width
    size_y:                      u32, // Sets Image Height
    max_i:                       u64, // Sets Maximum Iterations for Generator
    gen_formula:              String, // Specifies Formula for Generator
}

fn error_exit(error_msg: String) {
    /*
    Function for exiting the program early with an error message. 
     */
    print!("[Exit code : 1 | {:?}]", error_msg);
}

fn input(out_msg: String) -> String {
    /*
    Function for easily getting input values from stdin
    */

    print!("{}", out_msg);
    let _ = std::io::stdout().flush();
    let mut v: String = String::default();
    let _ = std::io::stdin().read_line(&mut v).unwrap();
    return v.trim().to_string();
}

fn interactive_config() -> Config {

    // Initializes Configuration Values
    let mut configuration = Config::default();

    // Sets max_i
    configuration.max_i = 1024;

    // Gets Dimensions
    configuration.size_x = match input("Enter image size (px): ".to_string()).parse() {
        Ok(n) => n,
        Err(_) => {
            error_exit("Function generation method not found!".to_string());
            std::process::exit(1);
        }
    };
    configuration.size_y = configuration.size_x;

    // Sets Generator
    configuration.gen_formula = input("Enter generation function: ".to_string());

    // Sets c & z swap
    configuration.c_init = match input("Swap 'C' & 'Z' values? (Generate Julia Set) [ y | n ]: ".to_string()).as_str() {
        "y" => Some(structs::Complex{real: 0.25, imaginary: 0.25}),
        _ => None,
    };

    // Ensures User wants to continue
    let confirmation = input(format!("Are you sure you want to continue with the size {} x {} [ y | n ]: ", configuration.size_x, configuration.size_y));

    // Validates responce
    if !(confirmation.to_lowercase().chars().nth(0).unwrap().to_string() == "y".to_string()) {
        error_exit("Early exit, declined continue on size check".to_string());
        std::process::exit(1);
    }

   return configuration;
}

fn get_math_value(value: u32, max_ref: u32) -> f64 {
    // Function for getting the mathematical space
    // Value from "value"
    4f64 * (value as f64) / (max_ref as f64 - 1f64) - 2f64
}

fn eval_function(config: Config, generator_function: &dyn Fn(u64, structs::Complex, structs::Complex) -> f64) {
    /*
       Function for getting the value of each point
       */
    // Sets Image Values
    
    let size_x: u32 = config.size_x;
    let size_y: u32 = config.size_y;
    let max_i: u64 = config.max_i;

    let mut img = image::ImageBuffer::new(size_x, size_y);
    for (_x, _y, pixel) in img.enumerate_pixels_mut() {
        *pixel = image::Rgb([255, 255, 255]);
    }

    let c = math::structs::Complex { real: 0f64, imaginary: 0f64, };

    // Goes through each pixel
    for i in 0..size_y {
        for j in 0..size_x {

             // Sets Initial Value
            let mut z = math::structs::Complex {
                real : get_math_value(j, size_x),
                imaginary : get_math_value(i, size_y),
            };

            for i in 0..max_i {
                z = generator_function(max_i, c, z);
            }

            // let z_output = generator_function(max_i, c, z);
            let pixel = img.get_pixel_mut(j, i);
            // Gets color value
            let out_rgb: (u8, u8, u8);

            if z_output == 0. {out_rgb = (255, 255, 255)}
            else if z_output == max_i as f64 {out_rgb = (0, 0, 0)}
            else {
                out_rgb = hsv::hsv_to_rgb(
                    ( 9f64 * z_output as f64 ) % 360f64,
                    1f64,
                    1f64,
                );
            };

            *pixel = image::Rgb([out_rgb.0, out_rgb.1, out_rgb.2]);
        }
        print!("\t {:.2}% | {} / {}\r", 100f64*(i as f64 + 1f64) / size_y as f64, i+1, size_y);
    }
    println!();
    println!("Saving File!");
    img.save("out.png").unwrap();
}


fn main() {
    // Main function of the program

    // Defines Initial Values

    let cli_args: Vec<String> = args().collect();
    println!("{:?}", cli_args);

    let config: Config = interactive_config();

    println!("Config : {:?}", config);

    // Initializes generators into a hashmap
    let mut generators: HashMap<String, &dyn Fn(u64, structs::Complex, structs::Complex) -> GenDataType> = HashMap::new();
    generators.insert("SD".to_string(),  &math::formula::SD);
    generators.insert("BS".to_string(),  &math::formula::BS);
    generators.insert("R".to_string(),   &math::formula::R);
    generators.insert("SYM".to_string(), &math::formula::SYM);

    let generator_function: &dyn Fn(u64, structs::Complex, structs::Complex) -> GenDataType;

    generator_function = match generators.get(&config.gen_formula) {
        Some(function_found) => function_found,
        None => {
            error_exit("Function generation method not found!".to_string());
            std::process::exit(1);
        }
    };

    // Sets the starting time
    let start_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs_f64();

    // Runs Config
    eval_function(config, generator_function);

    // Finished Timings
    let end_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs_f64();

    // Show Completion Message
    println!("[Finished in {:.1}s]", end_time - start_time);
    let _ = std::io::stdin().read_line(&mut String::new());
}
