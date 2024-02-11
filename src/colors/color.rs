#![allow(non_snake_case)]

use super::super::*;

use clap::error::ErrorKind;
use clap::CommandFactory;

/*
    Author : Mark T
      Date : 6/21/2023
*/


/// Rotational Coloring function for generation. Uses HSV rotational color. 
fn ROTATIONAL(n: f64, config: &Config) -> f64 {
    // Gets color value
    return n * config.rate_of_color_change;
}

/// Sinusoidal Coloring function for generation. 
fn SINUSOIDAL(n: f64, config: &Config) -> f64 {

    let max_value: f64 = 277.0;
    let min_value: f64 = 420.0;

    // 0.0174532925199 = pi / 180
    return (max_value - min_value) * 0.5 * (n * config.rate_of_color_change* 0.0174532925199).cos() +
        (max_value + min_value) * 0.5;
}

const COLORS: [(&str, &dyn Fn(f64, &Config) -> f64, &str);2] = [
    ("ROTATIONAL", &ROTATIONAL, "Simple rotational color based on iteration value"),
    ("SINUSOIDAL", &SINUSOIDAL, "Sinusoidal color values generated between set values"),
];

/// Function for getting the color formula from config
pub fn get_color(color: &str) -> &dyn Fn(f64, &Config) -> f64 {

    // Tries to find function in FORMULAS const
    for (key, value, _) in COLORS.iter() {
        if key == &color {
            return value;
        }
    }

    let color_string: String = COLORS
        .iter()
        .map(|v| format!("  {}\t{}", v.0, v.2))
        .collect::<Vec<String>>()
        .join("\n");

    // If not found throw error
    Args::command().error(
        ErrorKind::InvalidValue,
        format!("Color generation method '{}' not found!\n\nAllowed Colors:\n{}", color, color_string)
    ).exit();
}
