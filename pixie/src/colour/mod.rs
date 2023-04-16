mod g8;
mod rgb;

use crate::error::Error;

use super::colour::Colour::{Grey, Rgb};
pub use g8::G8;
pub use rgb::Rgb8;

#[derive(Debug, Clone, Copy)]
pub enum Colour {
    Grey(G8),
    Rgb(Rgb8),
}

impl Colour {
    pub fn new(t: &str, vals: Vec<u8>) -> Result<Self, Error> {
        match t {
            "Grey" => Ok(Grey(G8::new(vals[0]))),
            "Rgb" => Ok(Rgb(Rgb8::new(vals[0], vals[1], vals[2]))),
            _ => Err(Error::InvalidColour),
        }
    }

    pub fn to_vec(self) -> Vec<u8> {
        match self {
            Grey(x) => x.to_vec(),
            Rgb(x) => x.to_vec(),
        }
    }
}
