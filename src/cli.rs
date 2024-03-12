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
  kyros --pixels 512 --formula R --color ROTATIONAL --shadow MINIMAL --travel-distance --progress -y
";


const LONG_ABOUT_CLI_ARGS: &str = "
 ~ Kyros
A CLI tool for generating fractal images. 

Example:
kyros --pixels 512       \\
      --formula R        \\
      --color ROTATIONAL \\
      --shadow MINIMAL   \\
      --travel-distance  \\
      --progress         \\
      -y

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

    /// Specifies the rate of color change for the color function
    #[arg(long, default_value_t=9.0, value_name="FLOAT")]
    pub rate_of_color_change: f64,

    /// Specifies shadow function to use
    #[arg(long, default_value_t=("NONE".to_string()), value_name="STR")]
    pub shadow: String,

    /// Specifies which color to use for the background
    #[arg(long, default_value_t=("white".to_string()), value_name="STR")]
    pub background: String,

    /// Specifies which color to use for the foreground
    #[arg(long, default_value_t=("black".to_string()), value_name="STR")]
    pub foreground: String,

    /*
    /// Specifies if RGBA should be used for the image
    #[arg(long, default_value_t=false, value_name="BOOL")]
    pub rgba: bool,
    */

    /// Specifies the way the file should be saved
    #[arg(long, default_value_t=("PNG".to_string()), value_name="STR")]
    pub save_method: String,

    /// Specifies filename for output
    #[arg(long, default_value_t=("out".to_string()), value_name="STR")]
    pub filename: String,

    /// Uses Julia set style generation
    #[arg(short, long, default_value_t=false, value_name="BOOL")]
    pub julia: bool,

    /// Sets initial real value for julia generation
    #[arg(long, default_value_t=0.08004012786314796, value_name="FLOAT")]
    pub c_real: f64,

    /// Sets initial imaginary value for julia generation
    #[arg(long, default_value_t=-0.6359321976472476, value_name="FLOAT")]
    pub c_imaginary: f64,

    /// Sets the image factor for x
    #[arg(long, default_value_t=4.0, value_name="FLOAT")]
    pub factor_x: f64,

    /// Sets the image factor for y
    #[arg(long, default_value_t=4.0, value_name="FLOAT")]
    pub factor_y: f64,

    /// Sets the image offset for x
    #[arg(long, default_value_t=2.0, value_name="FLOAT")]
    pub offset_x: f64,

    /// Sets the image offset for y
    #[arg(long, default_value_t=2.0, value_name="FLOAT")]
    pub offset_y: f64,

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
