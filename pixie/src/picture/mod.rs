mod picture_builder;

use super::colour::Colour;
use picture_builder::PictureBuilder;

struct Picture {
    bounds: (usize, usize),
    pixels: Vec<Colour>,
}

impl Picture {
    fn new() -> PictureBuilder {
        PictureBuilder::new()
    }
}
