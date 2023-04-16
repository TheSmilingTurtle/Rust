pub mod colour;
pub mod error;
pub mod picture;

pub mod prelude {
    pub use crate::colour::{Colour, Rgb8, G8};
    pub use crate::picture::Picture;
}


#[cfg(test)]
mod tests {
    use crate::{prelude::*, error::Error};
    use rand::prelude::*;

    #[test]
    fn random() -> Result<(), Error> {

        fn test_generator(_: usize, _: usize) -> Colour {
            Colour::new("Rgb", vec![thread_rng().gen_range(0..255), thread_rng().gen_range(0..255), thread_rng().gen_range(0..255)]).unwrap()
        }
        
        Picture::build()
            .bounds(100, 100)
            .from_fn_par(test_generator, 4)?
            .save("random.png")?;

        Ok(())
    }

    #[test]
    fn circle() -> Result<(), Error> {
        
        fn circle_generator(x: usize, y: usize) -> Colour {
            if (x as isize - 500).pow(2) + (y as isize - 500).pow(2) <= 500isize.pow(2) + 1 {
                Colour::new("Grey", vec![0]).unwrap()
            }
            else {
                Colour::new("Grey", vec![255]).unwrap()
            }
        }
        
        Picture::build()
            .bounds(1001, 1001)
            .from_fn_par(circle_generator, 4)?
            .save("circle.png")?;
        

        Ok(())
    }

    #[test]
    fn circle_closure() -> Result<(), Error> {

        Picture::build()
            .bounds(1000, 1000)
            .from_fn_par(|x, y|
                if (x as isize - 500).pow(2) + (y as isize - 500).pow(2) <= 499isize.pow(2) {
                    Colour::new("Grey", vec![0]).unwrap()
                }
                else {
                    Colour::new("Grey", vec![255]).unwrap()
                },
                100)?
            .save("circle_closure.png")?;
        
        Ok(())
    }

    #[test]
    fn sin() -> Result<(), Error> {

        Picture::build()
            .bounds(1000, 1000)
            .from_fn_par(|x, _|
                    Colour::new("Rgb", vec![(( (x as f64 / 500.).sin() * 255.) as u8), (( (x as f64 / 250.).sin() * 255.) as u8), (( (x as f64 / 125.).sin() * 255.) as u8)]).unwrap()
                ,6)?
            .save("sin.png")?;
        
        Ok(())
    }

}
