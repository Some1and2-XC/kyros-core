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


fn SD(c: structs::Complex, z: structs::Complex) -> structs::Complex {
    return z * z + c;
}

fn R(c: structs::Complex, z: structs::Complex) -> structs::Complex {
    let mut new_z = z * z + c;
    new_z.imaginary -= z.real;
    new_z.real -= z.imaginary;
    return new_z;
}

fn ABR(c: structs::Complex, z: structs::Complex) -> structs::Complex {
    let mut new_z = z * z;
    if new_z.imaginary < 0.0 {
        new_z.imaginary *= -1.0;
    }
    new_z.imaginary -= z.real;
    new_z.real -= z.imaginary;
    return new_z + c;
}

fn BS(c: structs::Complex, mut z: structs::Complex) -> structs::Complex {
    z = z * z;
    if z.imaginary > 0.0 {
        z.imaginary = z.imaginary * -1.0;
    }
    return z + c;
}

fn SYM(c: structs::Complex, z: structs::Complex) -> structs::Complex {
    return z * z + c - z;
}

/// Sets Bootleg hashmap for formulas
///   FORMULAS.0 == Key Value
///   FORMULAS.1 == Function Value
///   FORMULAS.2 == Documentation Value
const FORMULAS: [(&str, &dyn Fn(structs::Complex, structs::Complex) -> structs::Complex, &str);5] = [
    ("SD"  , &SD  , "Standard z = z^2 + c"),
    ("R"   , &R   , "Custom Rabbit Generator"),
    ("ABR" , &ABR , "Absolute Value Rabbit Generator"),
    ("BS"  , &BS  , "Burning Ship Generator"),
    ("SYM" , &SYM , "A Symetrical Mandelbrot Like Generation"),
];

/// Function for getting generator formula from FORMULAS const
pub fn get_formula(formula: &str) -> &dyn Fn(structs::Complex, structs::Complex) -> structs::Complex {

    // Tries to find function in FORMULAS const
    for (key, value, _) in FORMULAS.iter() {
        if key == &formula {
            return value;
        }
    }

    let formula_string: String = FORMULAS
        .iter()
        .map(|v| format!("  {}\t{}", v.0, v.2))
        .collect::<Vec<String>>()
        .join("\n");

    // If not found throw error
    Args::command().error(
        ErrorKind::InvalidValue,
        format!("Function generation method '{}' not found!\n\nAllowed Formulas:\n{}", formula, formula_string)
    ).exit();
}
