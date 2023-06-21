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

const SIZE_X: &'static usize = &255;
const SIZE_Y: &'static usize = &255;
const MAX_I : &'static u32 = &1024;

fn set_initial_values(state: &mut [[math::Complex; *SIZE_X]; *SIZE_Y]) {
    /* 
    Function sets initial values for the array. 
    Sets each value to be coordinates of mathematical space
    */

    fn get_math_value(value: usize, max_ref: usize) -> f64 {
        // Function for getting a mathematical space
        // Value from "Value"
        4f64 * (value as f64) / (max_ref as f64 - 1f64) - 2f64
    }

    // Itterates through all values and sets real / imaginary values to be coordinates in mathematical space
    for i in 0..*SIZE_Y {
        for j in 0..*SIZE_X {
            state[i][j] = math::Complex {
                real : get_math_value(j, *SIZE_X),
                imaginary : get_math_value(i, *SIZE_Y),
            }
        }
    }
}

fn eval_function(state: &mut [[math::Complex; *SIZE_X]; *SIZE_Y]) -> [[u32; *SIZE_X]; *SIZE_Y] {
    /*
        Function for getting the value of each point
    */

    let mut out_values = [[0u32; *SIZE_X]; *SIZE_Y];

    // Goes through each pixel
    for i in 0..*SIZE_Y {
        for j in 0..*SIZE_X {

            // Sets Variables
            let mut z = state[i][j];
            let c = z.clone();

            // Does Math
            for iteration in 0..*MAX_I {
                z = z * z + c;
                if z.is_greater(2f64) {
                    out_values[i][j] = iteration;
                    break
                }
                if iteration == *MAX_I - 1 {
                    out_values[i][j] = *MAX_I;
                }
            }
        }
    }
    out_values
}

fn save_image(state: &mut [[u32; *SIZE_X]; *SIZE_Y]) {
    /* 
        Function that saves the 2D list of numbers as an image. 
    */

    let mut img = image::ImageBuffer::new(*SIZE_X as u32, *SIZE_Y as u32);
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let r = (0.2 * x as f32) as u8;
        let b = (0.2 * y as f32) as u8;
        *pixel = image::Rgb([r, 0, b]);
    }
    for i in 0..*SIZE_Y {
        for j in 0..*SIZE_X {
            let pixel = img.get_pixel_mut(j as u32, i as u32);
            let image::Rgb(data) = *pixel;
            let n = (255 * state[i][j] / *MAX_I) as u8;
            *pixel = image::Rgb([data[0], n, data[2]]);
        }
    }

    img.save("out.png").unwrap();

}

fn main() {
    // Main function of the program

    // Ensures User wants to continue
    {
        // Answer check to make sure the its equals to "y"
        println!("Are you sure you want to continue with the size {} x {} [ y | n ]?", SIZE_X, SIZE_Y);
        let mut v: String = String::default();
        let _ = std::io::stdin().read_line(&mut v).unwrap();

        assert_eq!(v
            .to_lowercase()
            .chars()
            .nth(0)
            .unwrap()
            , 'y');
    }

    // Configures state
    let mut state = [ [math::Complex { real : 0f64, imaginary : 0f64 }; *SIZE_X]; *SIZE_Y ];
    set_initial_values(&mut state);
    let mut out_values = eval_function(&mut state);

    save_image(&mut out_values);

    // Show Completion Message
    {
        println!("FINISHED!");
        let _ = std::io::stdin().read_line(&mut String::new());
    }
}
