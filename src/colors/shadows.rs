#![allow(non_snake_case)]

use crate::cli::Args;

use clap::error::ErrorKind;
use clap::CommandFactory;

/*
    Author : Mark T
      Date : 6/21/2023
*/

pub trait Shadows {
    fn get_alias(&self) -> String;
    fn get_description(&self) -> String;
    fn method(&self, n: f64) -> f64;
    fn gpu_method(&self) -> String;
}

/// Rotational Coloring function for generation. Uses HSV rotational color.
pub struct NONE {}
impl Shadows for NONE {
    fn get_alias(&self) -> String {
        "NONE".into()
    }
    fn get_description(&self) -> String {
        "\tDoesn't change values, sets all lightness values to '1'".into()
    }
    fn method(&self, _n: f64) -> f64 {
        return 1.0;
    }
    fn gpu_method(&self) -> String {
        "return 1.0;".into()
    }
}

pub struct MINIMAL {}
impl Shadows for MINIMAL {
    fn get_alias(&self) -> String { "MINIMAL".into() }
    fn get_description(&self) -> String { "Adds slight variance to values based on cos wave".into() }
    fn method(&self, n: f64) -> f64 {
        return 0.125 * (n * 9.0).cos() + 0.815;
    }
    fn gpu_method(&self) -> String {
        "return 0.125 * cos(n * 9.0) + 0.815;".into()
    }
}

pub struct MODULUS {}
impl Shadows for MODULUS {
    fn get_alias(&self) -> String { "MODULUS".into() }
    fn get_description(&self) -> String { "Adds significant variance using a sawtooth wave".into() }
    fn method(&self, n: f64) -> f64 {
        let modulus_value = 3.0;
        return 1.0 - (n.rem_euclid(modulus_value) / modulus_value);
    }
    fn gpu_method(&self) -> String {
        "
        float modulus_value = 3.0;
        return 1.0 - (mod(n, modulus_value) / modulus_value);
        ".into()
    }
}

/// Function for getting the shadow formula from config
pub fn get_shadow(shadow: &str) -> &dyn Shadows{
    let shadows: Vec<&dyn Shadows> = vec![
        &NONE {},
        &MINIMAL {},
        &MODULUS {},
    ];

    // Tries to find function in shadows array
    for method in shadows.clone() {
        if method.get_alias() == shadow.to_string() {
            return method;
        }
    }

    let shadow_string: String = shadows
        .iter()
        .map(|v| format!("  {}\t{}", v.get_alias(), v.get_description()))
        .collect::<Vec<String>>()
        .join("\n");

    // If not found throw error
    Args::command().error(
        ErrorKind::InvalidValue,
        format!("Shadow method '{}' not found!\n\nAllowed Shadows:\n{}", shadow, shadow_string)
    ).exit();
}
