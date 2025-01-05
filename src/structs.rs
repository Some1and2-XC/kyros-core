/*
Author : @Some1and2
  Date : 6/21/2023

File for containing the logic for the Complex Struct
*/

extern crate csscolorparser;

use crate::cli::default_level_filter;

use std::ops::{ Add, Sub, Mul};

use log::LevelFilter;
use serde::{Deserialize, Serialize};
use vulkano::buffer::BufferContents;

/// Main object for defining generation configuration.
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub c_init:          Option<Complex>, // Initial C value for when swap_zc is used
    pub size_x:                      u32, // Sets Image Width
    pub size_y:                      u32, // Sets Image Height
    pub max_i:                       u64, // Sets Maximum Iterations for Generator
    pub gen_formula:              String, // Specifies Formula for Generator
    pub color_formula:            String, // Specifies Formula for Colors
    pub rate_of_color_change:        f64, // Specifies the rate color changes for the color_formula
    pub shadow_formula:           String, // Specifies Formula for Shadows
    pub background:csscolorparser::Color, // Specifies the background to use for the image
    pub foreground:csscolorparser::Color, // Specifies the foreground to use for the image
    pub travel_distance:            bool, // Specifies if the output color value should be based on travel distance
    pub rgba:                       bool, // Specifies if the image should be rgba or not
    pub gpu:                        bool, // Specifies if image should be generated on the GPU
    pub chunk_sizes:         Option<u64>, // Specifies the amount of pixels per chunk (width x height)
    pub compression_threads:       usize, // Specifies the amount of threads used for compression (with gpu gen)
    pub save_method:              String, // Specifies the way the image should be saved
    pub filename:                 String, // Specifies the filename of the image
    pub math_frame:            MathFrame,
    #[serde(skip, default = "default_level_filter")]
    pub logs:                LevelFilter,
}

/// Struct for factor & offset for math space
/// This is used to calculate where each pixel is mapped to
#[derive(Debug, Clone, Default, Serialize, Deserialize, BufferContents)]
#[repr(C)]
pub struct MathFrame {
    /// This factor is in pixel space. (Offset in x axis)
    pub factor_x: f32,
    /// This factor is in pixel space. (Offset in y axis)
    pub factor_y: f32,

    /// This offset is in math space. (Offset in x axis)
    pub offset_x: f32,
    /// This offset is in math space. (Offset in y axis)
    pub offset_y: f32,
}

// Sets up Complex Struct
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Complex {
    pub real: f32,
    pub imaginary: f32,
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
    pub fn is_greater(self, other: f32) -> bool {
        (self.real * self.real + self.imaginary * self.imaginary) > (other * other)
    }
}
