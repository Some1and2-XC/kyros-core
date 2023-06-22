// #![allow(non_snake_case)]
// #![allow(dead_code)]
// #![allow(unused_variables)]


/*
Author : @Some1and2
  Date : 6/21/2023

Main file for running processes
*/

// use std::{fs};
mod math;

extern crate image;
use std::time::{SystemTime, UNIX_EPOCH};


fn get_math_value(value: u32, max_ref: u32) -> f64 {
    // Function for getting the mathematical space
    // Value from "value"
    4f64 * (value as f64) / (max_ref as f64 - 1f64) - 2f64
}

fn eval_function(size_x: u32, size_y: u32, max_i: u32) {
    /*
        Function for getting the value of each point
    */

    fn get_point_value( mut z: math::Complex, max_i: u32 ) -> u32 {
        /*
            Function for getting the value of a particular point
            Returns the amount of iterations completed
        */

        // Sets Initial Value
        let c = z.clone();

        // Gets Mandelbrot Value
        for iteration in 0..max_i {
            z = z * z + c;
            if z.is_greater(2f64) {
                return iteration;
            }
        }
        return max_i;
    }

    // Sets Image Values
    let mut img = image::ImageBuffer::new(size_x, size_y);
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let r = (0.2 * x as f32) as u8;
        let b = (0.2 * y as f32) as u8;
        *pixel = image::Rgb([r, 0, b]);
    }

    // Goes through each pixel
    for i in 0..size_y {
        for j in 0..size_x {

            // Sets Initial Value
            let z = math::Complex{ real : get_math_value(j, size_x), imaginary : get_math_value(i, size_y), };
            // Gets output of z value
            let z_output = get_point_value( z, max_i );
            let pixel = img.get_pixel_mut(j as u32, i as u32);
            let image::Rgb(data) = *pixel;
            let n = (255 * z_output / max_i) as u8;
            *pixel = image::Rgb([data[0], n, data[2]]);
        }
        print!("\t {:.2}% | {} / {}\r", 100f64*(i as f64 + 1f64) / size_y as f64, i+1, size_y);
    }
    println!();
    img.save("out.png").unwrap();
}


fn main() {
    // Main function of the program

    // Defines Initial Values
    let size_x = 2048u32;
    let size_y = 2048u32;
    let max_i = 1024u32;

    // Ensures User wants to continue
    {
        // Answer check to make sure the its equals to "y"
        println!("Are you sure you want to continue with the size {} x {} [ y | n ]?", size_x, size_y);
        let mut v: String = String::default();
        let _ = std::io::stdin().read_line(&mut v).unwrap();

        assert_eq!(v
            .to_lowercase()
            .chars()
            .nth(0)
            .unwrap()
            , 'y');
    }

    let start_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // Configures state
    eval_function(size_x, size_y, max_i);

    let end_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    println!("[Finished in {:.1}s]", end_time as i64 - start_time as i64);

    // Show Completion Message
    {
        println!("FINISHED!");
        let _ = std::io::stdin().read_line(&mut String::new());
    }
}
