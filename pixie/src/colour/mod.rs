mod g8;
mod rgb;

use g8::G8;
use rgb::Rgb8;

#[derive(Debug, Clone, Copy)]
pub enum Colour {
    Grey(G8),
    Rgb(Rgb8),
}
