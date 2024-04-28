#[allow(unused_variables)]

use super::super::*;
use hsv;

pub trait ColorProfile {
    fn get_foreground(&self) -> &Vec<u8>;
    fn get_background(&self) -> &Vec<u8>;
    fn method(&self, value: f64, max_i: f64) -> Vec<u8>;
}

pub struct RgbProfile {
    pub foreground: Vec<u8>,
    pub background: Vec<u8>,
}

impl ColorProfile for RgbProfile {

    fn get_foreground(&self) -> &Vec<u8> { &self.foreground }
    fn get_background(&self) -> &Vec<u8> { &self.background }

    fn method(&self, hue: f64, value: f64) -> Vec<u8> {
        Vec::from(
            {
                let color = hsv::hsv_to_rgb(
                    hue,
                    1.0,
                    value
                );
                [color.0, color.1, color.2, 255]
            }
        )
    }
}

pub fn get_profile(config: &Config) -> Box<dyn ColorProfile> {

    let background = config.background.to_linear_rgba_u8();
    let foreground = config.foreground.to_linear_rgba_u8();

    return Box::new(
        RgbProfile{
            background: Vec::from([
                background.0,
                background.1,
                background.2,
                background.3,
            ]),
            foreground: Vec::from([
                foreground.0,
                foreground.1,
                foreground.2,
                foreground.3,
            ]),
        }
    );
}
