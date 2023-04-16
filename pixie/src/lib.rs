mod colour;
mod error;
mod picture;

mod prelude {
    pub use crate::colour::{Colour, Rgb8, G8};
    pub use crate::picture::Picture;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
