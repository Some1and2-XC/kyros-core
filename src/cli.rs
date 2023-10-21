#![allow(unused_parens)]

/*
Author : Mark T
  Date : 10/17/2023

  File for storing CLI Configuration
*/

use clap::Parser;

static ABOUT_CLI_ARGS: &str = "
 ~ Kyros
A CLI tool for generating fractal images.

Example:
  kyros --pixels 512 --formula R --color ROTATIONAL --shadow minimal --travel-distance -y
";


const LONG_ABOUT_CLI_ARGS: &str = "
 ~ Kyros
A CLI tool for generating fractal images. 

Example:
  kyros --pixels 512       \
        --formula R        \
        --color ROTATIONAL \
        --shadow minimal   \
        --travel-distance -y

The 'pixels' flag refers to the size of the image, (both the amount of pixels in the x & y direction.)
The 'formula' flag refers to the formula that is used to get a value to pass to the color generation. 
The 'color' flag refers to the formula that generates a hue value.
The 'shadow' flag refers to the formula that generates the light / dark value of each pixel.
The 'travel-distance' flag changes the measurement of the generator to instead of measuring iterations, measuring mathematical travel distance.

Getting more help:
Potential values for the formula, color and shadow flags can be retreived by passing an invalid values (such as 'HELP') to them.
";

#[derive(Parser, Debug)]
#[command(about=ABOUT_CLI_ARGS)]
#[command(long_about=LONG_ABOUT_CLI_ARGS)]
#[command(version)]
pub struct Args {

    /// The amount of pixels to generate
    #[arg(short, long, default_value_t = 256, value_name="INT")]
    pub pixels: u32,

    /// The amount of iterations to run per pixel
    #[arg(short, long, default_value_t = 1024, value_name="INT")]
    pub iterations: u64,

    /// The generation function to use
    #[arg(short, long, default_value_t=("SD".to_string()), value_name="STR", long_help="Sets the generation function to use. \nSet this value to 'HELP' for more information.")] // The Compiler lies, parentheses are needed
    pub formula: String,

    /// Specifies color function to use
    #[arg(long, default_value_t=("ROTATIONAL".to_string()), value_name="STR")]
    pub color: String,

    /// Specifies shadow function to use
    #[arg(long, default_value_t=("none".to_string()), value_name="STR")]
    pub shadow: String,

    /// Uses Julia set style generation
    #[arg(short, long, default_value_t=false, value_name="BOOL")]
    pub julia: bool,

    /// Uses Travel Distance to color pixels
    #[arg(long, default_value_t=false, value_name="BOOL")]
    pub travel_distance: bool,

    /// Flag for showing progress
    #[arg(long, default_value_t=false, value_name="BOOL")]
    pub progress: bool,

    /// Confirm image generation
    #[arg(short, long, required(true))]
    pub y_confirm: bool,
}
