#![allow(non_snake_case)]

use super::super::structs;
use super::super::Args;

use clap::error::ErrorKind;
use clap::CommandFactory;

/*
# Purpose
This section of the code is for defining the different functions that are used
to generate images. This is the function that gets run on each pixel. 
*/

pub trait Formula {
    fn get_alias(&self) -> String;
    fn get_description(&self) -> String;
    fn method(&self, c: structs::Complex, z: structs::Complex) -> structs::Complex;
    fn gpu_method(&self) -> String;
}

struct SD {}
impl Formula for SD {
    fn get_alias(&self) -> String { "SD".into() }
    fn get_description(&self) -> String { "Standard z = z^2 + c".into() }
    fn method(&self, c: structs::Complex, z: structs::Complex) -> structs::Complex {
        z * z + c
    }
    fn gpu_method(&self) -> String {
        "
        z = add(mult(z, z), c);
        ".trim().into()
    }
}

struct R {}
impl Formula for R {
    fn get_alias(&self) -> String { "R".into() }
    fn get_description(&self) -> String { "Custom Rabbit Generator".into() }
    fn method(&self, c: structs::Complex, z: structs::Complex) -> structs::Complex {
        let mut new_z = z * z + c;
        new_z.imaginary -= z.real;
        new_z.real -= z.imaginary;
        return new_z;
    }
    fn gpu_method(&self) -> String {
        "
        Complex old_z = z;
        z = add(mult(z, z), c);
        z.data.y -= old_z.data.x;
        z.data.x -= old_z.data.y;
        ".trim().into()
    }
}

struct ABR {}
impl Formula for ABR {
    fn get_alias(&self) -> String { "ABR".into() }
    fn get_description(&self) -> String { "Absolute Value Rabbit Generator".into() }
    fn method(&self, c: structs::Complex, z: structs::Complex) -> structs::Complex {
        let mut new_z = z * z;
        if new_z.imaginary < 0.0 {
            new_z.imaginary *= -1.0;
        }
        new_z.imaginary -= z.real;
        new_z.real -= z.imaginary;
        return new_z + c;
    }
    fn gpu_method(&self) -> String {
        "
        Complex old_z = z;
        z = mult(z, z);
        if (z.data.y < 0.0) {
            z.data.y *= -1.0;
        }
        z.data.y -= old_z.data.x;
        z.data.x -= old_z.data.y;
        z.data += c.data;
        ".trim().into()
    }
}

struct BS {}
impl Formula for BS {
    fn get_alias(&self) -> String { "BS".into() }
    fn get_description(&self) -> String { "Burning Ship Generator".into() }
    fn method(&self, c: structs::Complex, mut z: structs::Complex) -> structs::Complex {
        z = z * z;
        if z.imaginary > 0.0 {
            z.imaginary = z.imaginary * -1.0;
        }
        return z + c;
    }
    fn gpu_method(&self) -> String {
        "
        z = mult(z, z);
        if (z.data.y > 0.0) {
            z.data.y *= -1.0;
        }
        z.data += c.data;
        ".trim().into()
    }
}

struct SYM {}
impl Formula for SYM {
    fn get_alias(&self) -> String { "SYM".into() }
    fn get_description(&self) -> String { "A Symetrical Mandelbrot Like Generation".into() }
    fn method(&self, c: structs::Complex, z: structs::Complex) -> structs::Complex {
        z * z + c - z
    }
    fn gpu_method(&self) -> String {
        "
        z = add(
            add(mult(z, z), c),
            Complex(
                -z.data
            )
        );
        ".trim().into()
    }
}

/// Function for getting generator formula from FORMULAS const
pub fn get_formula(formula: &str) -> &dyn Formula {

    let formulas: Vec<&dyn Formula> = vec![
        &SD  {},
        &R   {},
        &ABR {},
        &BS  {},
        &SYM {},
    ];

    // Tries to find function in FORMULAS const
    for method in formulas.clone() {
        if method.get_alias() == formula.to_string() {
            return method;
        }
    }

    let formula_string: String = formulas
        .iter()
        .map(|v| format!("  {}\t{}", v.get_alias(), v.get_description()))
        .collect::<Vec<String>>()
        .join("\n");

    // If not found throw error
    Args::command().error(
        ErrorKind::InvalidValue,
        format!("Function generation method '{}' not found!\n\nAllowed Formulas:\n{}", formula, formula_string)
    ).exit();
}
