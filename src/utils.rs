
/*
Author : Mark T
  Date : 10/17/2023

  File for general utilities
*/

use std::io::Read;

use super::*;

use crate::colors::profiles::get_profile;

/// Function for getting image from configuration and generator function. 
pub fn eval_function(config: &Config) -> Vec<u8> {

    let color_function = get_color(&config.color_formula.as_str());
    let shadow_function = get_shadow(&config.shadow_formula.as_str());
    let generator_function = get_formula(&config.gen_formula.as_str());

    // Sets Initial 'c' Value (If set)
    let mut c = Complex { real: 0f64, imaginary: 0f64, };
    let is_julia: bool = match config.c_init {
        Some(value) => {
            c = value;
            true
        },
        None => false,
    };

    // Sets Math Values
    let x_math_space_factor = config.math_frame.factor_x;
    let y_math_space_factor = config.math_frame.factor_y;

    let x_math_space_offset = config.math_frame.offset_x;
    let y_math_space_offset = config.math_frame.offset_y;

    let mut z: Complex;
    let mut old_z: Complex;

    let max_i = config.max_i as f64;

    let color_profile = get_profile(&config);

    // Initializes Image Buffer
    // let mut img = ImageBuffer::new(config.size_x, config.size_y);
    let mut img: Vec<u8> = Vec::with_capacity((config.size_x * config.size_y) as usize);

    // Goes through each pixel
    for i in 0..config.size_y {
        for j in 0..config.size_x {

            // Sets Initial Z Value
            z = Complex {
                real      : x_math_space_factor * j as f64 + x_math_space_offset,
                imaginary : y_math_space_factor * i as f64 + y_math_space_offset,
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

            // Gets pixel pointer
            // img.put_pixel(j, i,
            img.extend(
                {
                    let out = match z_output {
                        x if x == 0.0 => color_profile.get_background().to_owned(),
                        x if (x >= max_i && !config.travel_distance) => color_profile.get_foreground().to_owned(),
                        _ => color_profile.method(
                            color_function(z_output, &config).rem_euclid(360.0),
                            shadow_function(z_output).rem_euclid(360.0),
                        ),
                    };
                    out[0..(3 + config.rgba as usize)].to_owned().iter()
                    // out.truncate(3 + config.rgba as usize);
                    // out.to_owned()
                }
            );
        }
        if config.progress {
            print!("\t {:.2}% | {} / {}\r", 100.0 * (i as f64 + 1.0) / config.size_y as f64, i+1, config.size_y);
        }
    }
    if config.progress {
        println!();
    }

    return img;
}

