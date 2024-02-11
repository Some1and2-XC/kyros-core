/*
Author : @Some1and2
  Date : 6/21/2023

File for containing the logic for the Complex Struct
*/

use std::ops::{ Add, Sub, Mul };

/// Main object for defining generation configuration. 
#[derive(Debug, Default)]
pub struct Config {
    pub c_init:          Option<Complex>, // Initial C value for when swap_zc is used
    pub size_x:                      u32, // Sets Image Width
    pub size_y:                      u32, // Sets Image Height
    pub max_i:                       u64, // Sets Maximum Iterations for Generator
    pub gen_formula:              String, // Specifies Formula for Generator
    pub color_formula:            String, // Specifies Formula for Colors
    pub rate_of_color_change:        f64, // Specifies the rate color changes for the color_formula
    pub shadow_formula:           String, // Specifies Formula for Shadows
    pub background:               String, // Specifies the background to use for the image
    pub rgba:                       bool, // Specifies if the image should be rgba or not
    pub travel_distance:            bool, // Speifies if the output color value should be based on travel distance
    pub save_method:              String, // Specifies the way the image should be saved
    pub filename:                 String, // Specifies the filename of the image
    pub math_frame:            MathFrame,
    pub progress:                   bool,
}

/// Struct for factor & offset for math space
/// This is used to calculate where each pixel is mapped to
#[derive(Debug, Default)]
pub struct MathFrame {
    pub factor_x: f64,
    pub factor_y: f64,

    pub offset_x: f64,
    pub offset_y: f64,   
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
