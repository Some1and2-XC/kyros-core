// External Imports
extern crate image;
extern crate csscolorparser;
extern crate log;

// Project Crates
pub mod math;
pub mod structs;
pub mod colors;
pub mod utils;
pub mod cli;
pub mod save;
pub mod gpu;
pub mod gpu_thread_utils;
pub mod validator;
pub mod open_writer;

pub use crate::structs::{Complex, Config, MathFrame};
pub use crate::cli::Args;

pub use crate::colors::color::get_color;
pub use crate::colors::shadows::get_shadow;
pub use crate::math::formula::get_formula;
pub use crate::save::get_save_method;

pub use log::{warn, Level, Metadata, Record};

