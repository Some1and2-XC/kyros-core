
/*
Author : Mark T
  Date : 10/17/2023

  File for general utilities
*/

use super::*;

/// Function for getting image from configuration and generator function. 
pub fn eval_function(config: &Config) -> image::RgbImage {

    // Unpacks Image Configuration
    let c_init: Option<Complex> = config.c_init;

    let generator_function = get_formula(&config.gen_formula.as_str());
    let color_function = get_color(&config.color_formula.as_str());
    let shadow_function = get_shadow(&config.shadow_formula.as_str());

    // Sets Initial 'c' Value (If set)
    let mut c = Complex { real: 0f64, imaginary: 0f64, };
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

    let mut z: Complex;
    let mut old_z: Complex;

    // Initializes Image Buffer
    let mut img = image::ImageBuffer::new(config.size_x, config.size_y);

    // Goes through each pixel
    for i in 0..config.size_y {
        for j in 0..config.size_x {

             // Sets Initial Z Value
            z = Complex {
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
        if config.progress {
            print!("\t {:.2}% | {} / {}\r", 100.0 * (i as f64 + 1.0) / config.size_y as f64, i+1, config.size_y);
        }
    }
    if config.progress {
        println!();
    }
    return img;
}

