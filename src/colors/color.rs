#![allow(non_snake_case)]

use super::super::*;

use clap::error::ErrorKind;
use clap::CommandFactory;

pub trait Colors {
    fn get_alias(&self) -> String;
    fn get_description(&self) -> String;
    fn method(&self, n: f64, config: &Config) -> f64;
    fn gpu_method(&self) -> String;
}

struct ROTATIONAL {}
impl Colors for ROTATIONAL {
    fn get_alias(&self) -> String { "ROTATIONAL".into() }
    fn get_description(&self) -> String { "Simple rotational color based on iteration value".into() }
    fn method(&self, n: f64, config: &Config) -> f64 { n * config.rate_of_color_change }
    fn gpu_method(&self) -> String {
        "return n;".into()
    }
}

struct SINUSOIDAL {}
impl Colors for SINUSOIDAL {
    fn get_alias(&self) -> String { "SINUSOIDAL".into() }
    fn get_description(&self) -> String { "Sinusoidal color values generated between set values".into() }
    fn method(&self, n: f64, config: &Config) -> f64 {
        let max_value: f64 = 277.0;
        let min_value: f64 = 420.0;

        // 0.0174532925199 = pi / 180
        return (max_value - min_value) * 0.5 * (n * config.rate_of_color_change * 0.0174532925199).cos() +
            (max_value + min_value) * 0.5;
    }
    fn gpu_method(&self) -> String {
        "
            float max_value = 277.0;
            float min_value = 420.0;

            return (max_value - min_value) * 0.5 * cos(n * 0.0174532925199) +
                (max_value + min_value) * 0.5;
        ".into()
    }
}

/// Function for getting the color formula from config
pub fn get_color(color: &str) -> &dyn Colors {

    let colors: Vec<&dyn Colors> = vec![
        &ROTATIONAL {},
        &SINUSOIDAL {},
    ];

    // Tries to find function in colors array
    for method in colors.clone() {
        if method.get_alias() == color.to_string() {
            return method;
        }
    }

    let color_string: String = colors
        .iter()
        .map(|v| format!("  {}\t{}", v.get_alias(), v.get_description()))
        .collect::<Vec<String>>()
        .join("\n");

    // If not found throw error
    Args::command().error(
        ErrorKind::InvalidValue,
        format!("Color method '{}' not found!\n\nAllowed Colors:\n{}", color, color_string)
    ).exit();
}
