// External Imports
extern crate image;
extern crate csscolorparser;
extern crate log;

// Project Crates
/// Module for various mathematical Methods.
pub mod math;
/// Module for various structs used throughout the program.
pub mod structs;
/// Module for handling various color methods.
pub mod colors;
/// Module for managing different execution methods (Specifically CPU & GPU execution options).
pub mod execution;
/// Module for CLI configuration.
pub mod cli;
/// Module for handling various save methods.
pub mod save;
/// Module for setting up GPU execution.
pub mod gpu;
/// Module for handling the various threads used in GPU execution.
pub mod gpu_thread_utils;
// pub mod validator;
/// Module for the `Write` implementation passed to the parallel zlib compressor.
pub mod open_writer;

pub use log::{warn, Level, Metadata, Record};

