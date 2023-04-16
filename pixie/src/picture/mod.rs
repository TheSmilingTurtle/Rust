mod picture_builder;

use super::colour::Colour;
use crate::error::Error;
pub use picture_builder::PictureBuilder;

use image::save_buffer;
use image::ColorType;

pub struct Picture {
    bounds: (usize, usize),
    pixels: Vec<Colour>,
}

impl Picture {
    pub fn build() -> PictureBuilder {
        PictureBuilder::new()
    }

    pub fn save(self, path: String) -> Result<Self, Error> {
        let buf = self
            .pixels
            .iter()
            .flat_map(|x| x.to_vec())
            .collect::<Vec<_>>();

        let res = save_buffer(
            path,
            &buf,
            self.bounds.0 as u32,
            self.bounds.1 as u32,
            ColorType::Rgb8,
        );

        match res {
            Ok(_) => Ok(self),
            Err(_) => Err(Error::SaveFailed),
        }
    }
}
