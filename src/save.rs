#![allow(non_snake_case)]
#![allow(unused_imports)]

use super::*;
use std::ops::Deref;
use std::io::Cursor;

use base64::{Engine as _, engine::general_purpose};

use clap::error::ErrorKind;
use clap::CommandFactory;

use tempfile::tempfile;

use image::{Rgb, Pixel, save_buffer, ColorType, ImageEncoder, ImageBuffer, PixelWithColorType, EncodableLayout};
use image::flat::Error;
use image::codecs::png::PngEncoder;

pub trait Save {
    fn method<P, Container>(image_buffer: ImageBuffer<P, Container>, config: &Config) -> Result<(), Error>
    where
        P: Pixel<Subpixel = u8>,
        P: PixelWithColorType,
        [P::Subpixel]: EncodableLayout,
        Container: Deref<Target = [P::Subpixel]>;
}

pub struct PNG {}

impl Save for PNG {
    fn method<P, Container>(image_buffer: ImageBuffer<P, Container>, config: &Config) -> Result<(), Error> 
    where
        P: Pixel,
        P: PixelWithColorType,
        [P::Subpixel]: EncodableLayout,
        Container: Deref<Target = [P::Subpixel]>,
    {
        // save_buffer(format!("{}.png", config.filename), image_buffer, image_buffer.width(), image_buffer.height());
        image_buffer.save(format!("{}.png", config.filename)).unwrap();
        return Ok(());
    }
}

pub struct B64 {}

impl Save for B64 {
    fn method<P, Container>(image_buffer: ImageBuffer<P, Container>, _config: &Config) -> Result<(), Error> 
    where
        P: Pixel<Subpixel = u8>,
        P: PixelWithColorType,
        [P::Subpixel]: EncodableLayout,
        Container: Deref<Target = [P::Subpixel]>,
    {
        let mut png_buf = Vec::new();
        {
            let encoder = PngEncoder::new(&mut png_buf);

            let _ = encoder.write_image(
                image_buffer.as_raw().as_ref(),
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

fn PNG(image_buffer: ImageBuffer<Rgb<u8>, Vec<u8>>, config: &Config) -> Result<(), Error> {
    image_buffer.save(format!("{}.png", config.filename)).unwrap();
    return Ok(());
}

fn B64(image_buffer: ImageBuffer<Rgb<u8>, Vec<u8>>, _config: &Config) -> Result<(), Error> {
    let mut png_buf = Vec::new();
    {
        let encoder = PngEncoder::new(&mut png_buf);
        let _ = encoder.write_image(
            &image_buffer.clone().into_raw(),
            image_buffer.width(),
            image_buffer.height(),
            image::ColorType::Rgb8);
    }

    let mut b64 = String::new();
    let _ = general_purpose::STANDARD.encode_string(png_buf, &mut b64);
    print!("{}", b64);
    Ok(())
}

const SAVE_METHODS: [(&str, &dyn Fn(ImageBuffer<Rgb<u8>, Vec<u8>>, &Config) -> Result<(), Error>, &str);2] = [
    ("PNG", &PNG, "Saves Image as PNG."),
    ("B64", &B64, "Sends base-64 encoded PNG image to std-out."),
];

/// Function for getting the method for saving images from config
pub fn get_save_method(save_method: &str) -> &dyn Fn(ImageBuffer<Rgb<u8>, Vec<u8>>, &Config) -> Result<(), Error> {

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
