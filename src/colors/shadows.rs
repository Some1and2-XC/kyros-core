#![allow(non_snake_case)]

use super::super::Args;

use clap::error::ErrorKind;
use clap::CommandFactory;

/*
    Author : Mark T
      Date : 6/21/2023
*/

pub trait Colors {
    fn get_alias(&self) -> String;
    fn get_description(&self) -> String;
    fn method(&self, n: f64) -> f64;
    fn gpu_method(&self) -> String;
}

/// Rotational Coloring function for generation. Uses HSV rotational color. 
/*
struct NONE {}
impl Colors for NONE {
    fn get_alias(&self) -> String {
        "NONE".into()
    }
    fn get_description(&self) -> String {
        "\tDoesn't change values, sets all lightness values to '1'".into()
    }
    fn method(&self, n: f64) -> f64 {
        return 1.0;
    }
    fn gpu_method(&self) -> String {
        "1.0".into()
    }
}
*/

fn NONE(_: f64) -> f64 {
    // Gets color value
    return 1.0;
}

fn MINIMAL(n: f64) -> f64{
    return 0.125 * (n * 9.0).cos() + 0.815;
}

fn MODULUS(n: f64) -> f64 {
    let modulus_value = 3.0;

    return 1.0 - (n.rem_euclid(modulus_value) / modulus_value);
}

const SHADOWS: [(&str, &dyn Fn(f64) -> f64, &str);3] = [
    ("NONE"    , &NONE    , "\tDoesn't change values, sets all lightness values to '1'"),
    ("MINIMAL" , &MINIMAL, "Adds slight variance to values based on cos wave"),
    ("MODULUS" , &MODULUS , "Adds significant variance using a sawtooth wave"),
];

/// Function for getting the shadow formula from config
pub fn get_shadow(shadow: &str) -> &dyn Fn(f64) -> f64 {

    // Tries to find function in FORMULAS const
    for (key, value, _) in SHADOWS.iter() {
        if key == &shadow {
            return value;
        }
    }

    let shadow_string: String = SHADOWS
        .iter()
        .map(|v| format!("  {}\t{}", v.0, v.2))
        .collect::<Vec<String>>()
        .join("\n");

    // If not found throw error
    Args::command().error(
        ErrorKind::InvalidValue,
        format!("Shadow generation method '{}' not found!\n\nAllowed Shadows:\n{}", shadow, shadow_string)
    ).exit();
}
