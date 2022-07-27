pub enum Colour {
    Colour(Rgba),
    Default
}

pub struct Rgba {
    r: u8,
    g: u8,
    b: u8,
    a: u8
}