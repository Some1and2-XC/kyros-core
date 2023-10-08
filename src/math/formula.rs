#![allow(non_snake_case)]

use super::structs;
use super::super::Args;

use clap::error::ErrorKind;
use clap::CommandFactory;

/*
# Purpose
This section of the code is for defining the different functions that are used
to generate images. This is the function that gets run on each pixel. 
# Definitions
SD
    SD stands for standard. This is the standard way of generating the mandelbrot set. 
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

/// Sets Bootleg hashmap cuz rust is a great language sent from god
///   FORMULAS.0 == Key Value
///   FORMULAS.1 == Function Value
///   FORMULAS.2 == Documentation Value
const FORMULAS: [(&str, &dyn Fn(structs::Complex, structs::Complex) -> structs::Complex, &str);4] = [
    ("SD"  , &SD  , "Standard z = z^2 + c"),
    ("R"   , &R   , "Custom Rabbit Generator"),
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
