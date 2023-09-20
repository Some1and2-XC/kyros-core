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
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;

fn error_exit(error_msg: String) {
    /*
    Function for exiting the program early with an error message. 
     */
    print!("[Exit code : 1 | {:?}]", error_msg);
}

fn get_math_value(value: u32, max_ref: u32) -> f64 {
    // Function for getting the mathematical space
    // Value from "value"
    4f64 * (value as f64) / (max_ref as f64 - 1f64) - 2f64
}

fn eval_function(size_x: u32, size_y: u32, max_i: u64, generator_function: Box<&dyn math::formula::Generator>) {
    /*
       Function for getting the value of each point
       */
    let get_point_value: &dyn math::formula::Generator = *generator_function;

    // Sets Image Values
    let mut img = image::ImageBuffer::new(size_x, size_y);
    for (_x, _y, pixel) in img.enumerate_pixels_mut() {
        *pixel = image::Rgb([255, 255, 255]);
    }

    let c = math::structs::Complex { real: 0f64, imaginary: 0f64, };

    // Goes through each pixel
    for i in 0..size_y {
        for j in 0..size_x {

             // Sets Initial Value
            let z = math::structs::Complex {
                real : get_math_value(j, size_x),
                imaginary : get_math_value(i, size_y),
            };

            let z_output = get_point_value.formula(max_i, c, z);
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

    // let size_x = 131_072u32;
    // let size_y = 131_072u32;
    let size_x = 1024u32;
    let size_y = 1024u32;
    let max_i = 2048u64;

    // Ensures User wants to continue
    {
        // Answer check to make sure the its equals to "y"
        print!("Are you sure you want to continue with the size {} x {} [ y | n ]: ", size_x, size_y);
        let _ = std::io::stdout().flush();
        let mut v: String = String::default();
        let _ = std::io::stdin().read_line(&mut v).unwrap();

        assert_eq!(v
            .to_lowercase()
            .chars()
            .nth(0)
            .unwrap()
            , 'y');
    }

    // Sets the starting time
    let start_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs_f64();

    // Configures state
    // Sets Generator
    print!("Enter generation Function: ");
    let _ = std::io::stdout().flush().unwrap();
    let mut gen_key = String::new();
    let _ = std::io::stdin().read_line(&mut gen_key);

    let mut generators: HashMap<String, Box<&dyn math::formula::Generator>> = HashMap::new();
    generators.insert("SD".to_string(),  Box::new(&math::formula::SD));
    generators.insert("BS".to_string(),  Box::new(&math::formula::BS));
    generators.insert("R".to_string(),   Box::new(&math::formula::R));
    generators.insert("SYM".to_string(), Box::new(&math::formula::SYM));

    let generator_function: Box<&dyn math::formula::Generator>;

    generator_function = match generators.get(gen_key.trim()) {
        Some(function_found) => function_found.to_owned(),
        None => {
            error_exit("Function generation method not found!".to_string());
            std::process::exit(1);
        }
    };

    // Runs Config
    eval_function(size_x, size_y, max_i, generator_function);

    // Finished Timings
    let end_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs_f64();

    // Show Completion Message
    println!("[Finished in {:.1}s]", end_time - start_time);
    let _ = std::io::stdin().read_line(&mut String::new());
}
