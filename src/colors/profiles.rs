#[allow(unused_variables)]

use image::{Rgb, Rgba};
use hsv;

pub trait ColorProfile<T> {
    fn get_foreground(&self) -> T;
    fn get_background(&self) -> T;
    fn method(&self, value: f64, max_i: f64) -> T;
}

pub struct RgbProfile {
    pub foreground: Rgb<u8>,
    pub background: Rgb<u8>,
}

impl ColorProfile<Rgb<u8>> for RgbProfile {

    fn get_foreground(&self) -> Rgb<u8> { self.foreground }
    fn get_background(&self) -> Rgb<u8> { self.background }

    fn method(&self, hue: f64, value: f64) -> Rgb<u8> {
        Rgb(
            hsv::hsv_to_rgb(
                hue,
                1.0,
                value
            ).into()
        )
    }
}

pub struct RgbaProfile {
    pub foreground: Rgba<u8>,
    pub background: Rgba<u8>,
}

impl ColorProfile<Rgba<u8>> for RgbaProfile {

    fn get_foreground(&self) -> Rgba<u8> { self.foreground }
    fn get_background(&self) -> Rgba<u8> { self.background }

    fn method(&self, hue: f64, value: f64) -> Rgba<u8> {
        let pixel = hsv::hsv_to_rgb(hue, 1.0, value);

        Rgba([pixel.0, pixel.1, pixel.2, 0])
    }
}
