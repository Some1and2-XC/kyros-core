#![allow(non_snake_case)]

use super::*;
use clap::error::ErrorKind;
use clap::CommandFactory;
use base64::{Engine as _, engine::general_purpose};
use image::codecs::png::PngEncoder;
use image::ImageEncoder;
use image::flat::Error;

fn PNG(image_buffer: image::RgbImage, config: &Config) -> Result<(), Error> {
    image_buffer.save(format!("out#{}.png", config.count)).unwrap();
    return Ok(());
}

fn B64(image_buffer: image::RgbImage, _config: &Config) -> Result<(), Error> {
    let mut png_buf = Vec::new();
    {
        let encoder = PngEncoder::new(&mut png_buf);
        let _ = encoder.write_image(&image_buffer.clone().into_raw(), image_buffer.width(), image_buffer.height(), image::ColorType::Rgb8);
    }

    let mut b64 = String::new();
    let _ = general_purpose::STANDARD.encode_string(png_buf, &mut b64);
    print!("{}", b64);
    Ok(())
}

const SAVE_METHODS: [(&str, &dyn Fn(image::RgbImage, &Config) -> Result<(), Error>, &str);2] = [
    ("PNG", &PNG, "Saves Image as PNG."),
    ("B64", &B64, "Sends base-64 encoded PNG image to std-out."),
];


/// Function for getting the method for saving images from config
pub fn get_save_method(save_method: &str) -> &dyn Fn(image::RgbImage, &Config) -> Result<(), Error> {

    // Tries to find function in FORMULAS const
    for (key, value, _) in SAVE_METHODS.iter() {
        if key == &save_method {
            return value;
        }
    }

    let saves_string: String = SAVE_METHODS
        .iter()
        .map(|v| format!("  {}\t{}", v.0, v.2))
        .collect::<Vec<String>>()
        .join("\n");

    // If not found throw error
    Args::command().error(
        ErrorKind::InvalidValue,
        format!("Save method '{}' not found!\n\nAllowed save methods:\n{}", save_method, saves_string)
    ).exit();
}
