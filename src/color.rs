#![allow(non_snake_case)]

use super::Args;

use clap::error::ErrorKind;
use clap::CommandFactory;

/*
    Author : Mark T
      Date : 6/21/2023
*/


/// Rotational Coloring function for generation. Uses HSV rotational color. 
fn ROTATIONAL(n: f64, max_i: u64) -> (u8, u8, u8) {

    // Gets color value
    let out_rgb: (u8, u8, u8);

    if n == 0.0 {out_rgb = (255, 255, 255)}
    else if n == max_i as f64 {out_rgb = (0, 0, 0)}
    else {
        out_rgb = hsv::hsv_to_rgb(
            ( 9f64 * n as f64 ) % 360.0,
            1.0,
            1.0,
        );
    };
    return out_rgb;
}

const COLORS: [(&str, &dyn Fn(f64, u64) -> (u8, u8, u8));1] = [
    ("ROTATIONAL", &ROTATIONAL),
];

/// Function for getting the color formula from config
pub fn get_color(color: &str) -> &dyn Fn(f64, u64) -> (u8, u8, u8) {

    // Tries to find function in FORMULAS const
    for (key, value) in COLORS.iter() {
        if key == &color {
            return value;
        }
    }

    // If not found throw error
    Args::command().error(
        ErrorKind::InvalidValue,
        format!("Color generation method '{}' not found!", color)
    ).exit();
}
