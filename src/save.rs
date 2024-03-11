#![allow(non_snake_case)]
#![allow(unused_imports)]

use super::*;
use std::ops::Deref;
use std::io::Cursor;
use std::error::Error;

use base64::{Engine as _, engine::general_purpose};

use clap::error::ErrorKind;
use clap::CommandFactory;

use image::{DynamicImage, save_buffer, ColorType, ImageEncoder, ImageBuffer, PixelWithColorType, EncodableLayout};
use image::codecs::png::PngEncoder;

pub trait Save {
    fn get_alias(&self) -> String;
    fn get_description(&self) -> String;
    fn method(&self, image_buffer: DynamicImage, config: &Config) -> Result<(), Box<dyn Error>>;
}

pub struct PNG {}

impl Save for PNG {
    fn get_alias(&self) -> String { "PNG".into() }
    fn get_description(&self) -> String { "Saves Image as PNG.".into() }
    fn method(&self, image_buffer: DynamicImage, config: &Config) -> Result<(), Box<dyn Error>> {
        // save_buffer(format!("{}.png", config.filename), image_buffer, image_buffer.width(), image_buffer.height());
        image_buffer.save(format!("{}.png", config.filename)).unwrap();
        return Ok(());
    }
}

pub struct B64 {}

impl Save for B64 {
    fn get_alias(&self) -> String { "B64".into() }
    fn get_description(&self) -> String { "Sends base-64 encoded PNG image to std-out.".into() }
    fn method(&self, image_buffer: DynamicImage, _config: &Config) -> Result<(), Box<dyn Error>> {
        let mut png_buf = Vec::new();
        {
            let encoder = PngEncoder::new(&mut png_buf);

            let _ = encoder.write_image(
                image_buffer.as_bytes(),
                image_buffer.width(),
                image_buffer.height(),
                image::ColorType::Rgb8);
        }

        let mut b64 = String::new();
        let _ = general_purpose::STANDARD.encode_string(png_buf, &mut b64);
        print!("{}", b64);
        Ok(())
    }
}

/// Function for getting the method for saving images from config
pub fn get_save_method(save_method: &str) -> &dyn Save {

    // Makes array of methods which can be used
    let methods: Vec<&dyn Save> = vec![
        &PNG{},
        &B64{},
    ];

    // Iterates through methods to find one that matches
    for method in methods.clone() {
        if method.get_alias() == save_method.to_string() {
            return method;
        }
    }

    let saves_string: String = methods
        .iter()
        .map(|v| format!("  {}\t{}", v.get_alias(), v.get_description()))
        .collect::<Vec<String>>()
        .join("\n");

    // If not found throw error
    Args::command().error(
        ErrorKind::InvalidValue,
        format!("Save method '{}' not found!\n\nAllowed save methods:\n{}", save_method, saves_string)
    ).exit();
}
