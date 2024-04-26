/*
  File for general utilities
*/

extern crate minijinja;

use minijinja::{context, Environment};

use super::*;
use std::error::Error;
use std::str;
use crate::gpu::run_glsl;
use crate::colors::profiles::get_profile;

/// Function for getting image from configuration and generator function. 
pub fn cpu_eval(config: &Config) -> Result<(), Box<dyn Error>> {

    let save_method = get_save_method(config.save_method.as_str());

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
                z = generator_function.method(c, z);

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

            // Adds a pixel
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
                }
            );
        }
        if config.logs >= Level::Info {
            print!("\t {:.2}% | {} / {}\r", 100.0 * (i as f64 + 1.0) / config.size_y as f64, i+1, config.size_y);
        }
    }
    if config.logs >= Level::Info {
        println!();
    }

    return save_method.method(img.as_slice(), config);
}

static TEMPLATE: &str = include_str!(
    concat!(env!("CARGO_MANIFEST_DIR"), "/comp.glsl")
);

pub fn gpu_eval(config: &Config) -> Result<(), Box<dyn Error>> {

    /// Takes a Vec<f64> and returns a string that looks like 1.00000, 2.00000, 3.00000
    /// Returns Option<None> if the result isn't the expected length
    fn get_arr_str_with_len(in_arr: Vec<f64>, expected_length: usize) -> Option<String> {
        if in_arr.len() != expected_length {
            log::debug!("Invalid length of array.");
            log::debug!("Expected length: `{expected_length}`.");
            log::debug!("Got Array: `{in_arr:?}`.");
            return None;
        }
        return Some(in_arr
            .iter()
            .map(|v| format!("{v:.5?}"))
            .collect::<Vec<String>>()
            .join(", ")
            .into()
        );
    }

    let color_function = get_color(&config.color_formula.as_str());
    let shadow_function = get_shadow(&config.shadow_formula.as_str());
    let generator_function = get_formula(&config.gen_formula.as_str());

    // Sets value for math constant 'c'
    let c: [f64; 2] = match config.c_init {
        Some(value) => {
            [value.real, value.imaginary]
        },
        None => [0.0, 0.0],
    };

    let color_profile = get_profile(&config);

    let compiled_shader = {
        let mut env = Environment::new();
        env.add_template(
            "compute_shader",
            TEMPLATE
            ).unwrap();
        let compute_shader = env.get_template("compute_shader").unwrap();

        compute_shader.render(context!(
            formula => generator_function.gpu_method(),
            rate_of_color_change => format!("{:.1}", config.rate_of_color_change),
            background => get_arr_str_with_len(config.background.to_array().into(), 4).unwrap(),
            foreground => get_arr_str_with_len(config.foreground.to_array().into(), 4).unwrap(),
            max_i => format!("{:.1}", config.max_i),
            c_init => get_arr_str_with_len(c.into(), 2).unwrap(),

            ))
            .unwrap()
            .replace("\\n", "\n")
            .replace("\\t", "\t")
            .replace("\\r", "\r")
    };

    log::debug!("{}", compiled_shader);
    return run_glsl(compiled_shader, config);
}
