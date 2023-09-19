// #![allow(dead_code)]
// #![allow(unused_variables)]


/*
Author : @Some1and2
  Date : 6/21/2023

  Main file for running processes
  */

// mod math;

mod math;

extern crate image;

use hsv;
use std::time::{SystemTime, UNIX_EPOCH};

fn get_math_value(value: u32, max_ref: u32) -> f64 {
	// Function for getting the mathematical space
	// Value from "value"
	4f64 * (value as f64) / (max_ref as f64 - 1f64) - 2f64
}

fn eval_function(size_x: u32, size_y: u32, max_i: u32, get_point_value: dyn math::formula::Generator) {
	/*
	   Function for getting the value of each point
	   */

    // Sets Image Values
	let mut img = image::ImageBuffer::new(size_x, size_y);
	for (_x, _y, pixel) in img.enumerate_pixels_mut() {
		*pixel = image::Rgb([255, 255, 255]);
	}

    let c = math::structs::Complex { real:0f64, imaginary: 0f64, };

	// Goes through each pixel
	for i in 0..size_y {
		for j in 0..size_x {

             // Sets Initial Value
			let z = math::structs::Complex {
                real : get_math_value(j, size_x),
                imaginary : get_math_value(i, size_y),
            };
            let z_output = get_point_value(max_i, c, z); // Gets output of z value
			let pixel = img.get_pixel_mut(j, i); // gets pixel reference for img[i][j]

			// Gets color value
			let out_rgb: (u8, u8, u8);

			if z_output == 0 {out_rgb = (255, 255, 255)}
			else if z_output == max_i {out_rgb = (0, 0, 0)}
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
	let max_i = 2048u32;

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

	// Sets the starting time
	let start_time = SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.unwrap()
		.as_secs_f64();

	// Configures state
    // Sets Generator
    let generator: &dyn math::formula::Generator = math::formula::SD;

	eval_function(size_x, size_y, max_i, generator);

	let end_time = SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.unwrap()
		.as_secs_f64();

	println!("[Finished in {:.1}s]", end_time as i64 - start_time as i64);

	// Show Completion Message
	{
		println!("FINISHED!");
		let _ = std::io::stdin().read_line(&mut String::new());
	}
}
