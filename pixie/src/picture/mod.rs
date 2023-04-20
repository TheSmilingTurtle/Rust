mod picture_builder;

use crate::colour::Colour;
use crate::error::Error;
pub use picture_builder::PictureBuilder;

use image::save_buffer;
use image::ColorType;

use image::ImageBuffer;

#[derive(Debug)]
pub struct Picture {
    pub bounds: (usize, usize),
    pub pixels: Vec<Colour>,
}

impl Picture {
    pub fn build() -> PictureBuilder {
        PictureBuilder::new()
    }

    pub fn save(&self, path: &str) -> Result<&Self, Error> {
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
            match self.pixels[0] {
                Colour::Grey(_) => ColorType::L8,
                Colour::Rgb(_) => ColorType::Rgb8,
            },
        );

        match res {
            Ok(_) => Ok(self),
            Err(_) => Err(Error::SaveFailed),
        }
    }
}
