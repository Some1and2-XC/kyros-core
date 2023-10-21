/*
Author : @Some1and2
  Date : 6/21/2023

File for containing the logic for the Complex Struct
*/

use std::ops::{ Add, Sub, Mul };

/// Main object for defining generation configuration. 
#[derive(Debug, Default)]
pub struct Config {
    pub count:                       u64, // Index of the generated image
    pub c_init:          Option<Complex>, // Initial C value for when swap_zc is used
    pub size_x:                      u32, // Sets Image Width
    pub size_y:                      u32, // Sets Image Height
    pub max_i:                       u64, // Sets Maximum Iterations for Generator
    pub gen_formula:              String, // Specifies Formula for Generator
    pub color_formula:            String, // Specifies Formula for Colors
    pub shadow_formula:           String, // Specifies Formula for Shadows
    pub travel_distance:            bool, // Speifies if the output color value should be based on travel distance
    pub math_frame:            MathFrame,
    pub progress:                   bool,
}

/// Struct for factor & offset for math space
/// This is used to calculate where each pixel is mapped to
#[derive(Debug, Default)]
pub struct MathFrame {
    pub x_math_space_factor: f64,
    pub x_math_space_offset: f64,

    pub y_math_space_factor: f64,
    pub y_math_space_offset: f64,   
}

// Sets up Complex Struct
#[derive(Debug, Clone, Copy)]
pub struct Complex {
	pub real: f64,
	pub imaginary: f64,
}

// Sets up Addition rules for Complex Numbers
impl Add for Complex {
	type Output = Complex;

	fn add(self, other: Complex) -> Complex {
		Complex {
			real : self.real + other.real,
			imaginary : self.imaginary + other.imaginary,
		}
	}
}

impl Sub for Complex {
    type Output = Complex;

    fn sub(self, other: Complex) -> Complex {
        Complex {
            real : self.real - other.real,
            imaginary : self.imaginary - other.imaginary,
        }
    }
}

// Sets up Multiplication rules for Complex Numbers
impl Mul for Complex {
	type Output = Complex;

	fn mul(self, other: Complex) -> Complex {
		Complex {
			real : self.real * other.real - self.imaginary * other.imaginary,
			imaginary : self.real * other.imaginary + self.imaginary * other.real,
		}
	}
}

impl Complex {
	// Sets up Comparison rules for Complex Numbers
	pub fn is_greater(self, other: f64) -> bool {
		(self.real * self.real + self.imaginary * self.imaginary) > (other * other)
	}
}
