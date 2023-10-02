/*
Author : @Some1and2
  Date : 6/21/2023

File for containing the logic for the Complex Struct
*/

use std::ops::{ Add, Sub, Mul };

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
