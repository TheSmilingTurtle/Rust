mod g8;
mod rgb;

pub use g8::G8;
pub use rgb::Rgb8;
use super::colour::Colour::{Grey, Rgb};


#[derive(Debug, Clone, Copy)]
pub enum Colour {
    Grey(G8),
    Rgb(Rgb8),
}

impl Colour {
    pub fn to_vec(self: Self) -> Vec<u8> {
        match self{
            Grey(x) => x.to_vec(),
            Rgb(x) => x.to_vec()
        }
    }
}