use num::complex::Complex;

mod utils {
    use image::ColorType;
    use image::codecs::png::PngEncoder;
    use image::ImageEncoder;
    use std::fs::File;
    use num::Complex;

    #[derive(Debug)]
    pub struct Picture{
        image: Vec<u8>,
        pub bounds: (usize, usize)
    }

    impl Picture{
        pub fn new(x_bound: usize, y_bound: usize) -> Picture {
            return Picture{
                image: vec![255; x_bound * y_bound],
                bounds: (x_bound, y_bound)
            }
        }

        pub fn paint_pixel(&mut self, pixel: (usize, usize), colour: u8) -> Result<(), std::io::Error> {
            if pixel.0 > self.bounds.0 || pixel.1 > self.bounds.1 {
                return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("Out of bounds, image bounds are ( {}, {}), ( {}, {}) is out of bounds.", self.bounds.0, self.bounds.1, pixel.0, pixel.1)));
            }

            self.image[pixel.1 * self.bounds.0 + pixel.0] = colour;
            Ok(())
        }
        
        pub fn export(self, filename: &str) -> Result<(), std::io::Error> {
            let output = File::create(filename)?;
    
            let encoder = PngEncoder::new(output);
            match encoder.write_image(&self.image, self.bounds.0 as u32, self.bounds.1 as u32, ColorType::L8) {
                Ok(_) => Ok(()),
                _     => Err(std::io::Error::new(std::io::ErrorKind::Other, "Could not write the file"))
            }
        
        }

    }

    #[derive(Debug)]
    pub struct Grid {
        top_left: num::Complex<f64>,

        width: f64,
        height: f64
    }

    impl Grid {
        pub fn new(tl: num::Complex<f64>, br: num::Complex<f64>) -> Grid {
            return Grid{
                top_left: tl,

                height: tl.im - br.im,
                width:  br.re - tl.re
            }
        }

        pub fn pixel_to_point(&self, pixel: (usize, usize), canvas: &Picture) -> Complex<f64> {
            self.top_left + Complex::<f64>::new(
                ((pixel.0 as f64) / (canvas.bounds.0 as f64)) * self.width,
                - ((pixel.1 as f64) / (canvas.bounds.1 as f64)) * self.height
            )
        }

    }

    pub fn complex_round(num: Complex<f64>) -> Complex<isize> {
        //technically does not round the number, but just multiplies it by the digits and truncates the rest
        
        let mul: f64 = 100.; //for it to act as actual rounding, this needs to be a power of ten
        
        Complex::new(
            (num.re * mul) as isize,
            (num.im * mul) as isize
        )
        
    }

    pub fn parse_complex(s: &str) -> Option<Complex<f64>> {
        match s.find("+") {
            Some(i) => match (s[..i].parse::<f64>(), s[i + 1..].parse::<f64>()) {
                            (Ok(re), Ok(im)) => Some(Complex::new(re, im)),
                            _ => None
                        }
            None    => match s.find("-") {
                        Some(i) => match (s[..i].parse::<f64>(), s[i + 1..].parse::<f64>()) {
                                        (Ok(re), Ok(im)) => Some(Complex::new(re, -im)),
                                        _ => None
                                    }
                        None    => None
            }
        }
    }

}

fn convergence_calculator(lamb: Complex<f64>) -> Option<u8> {
    let one = Complex::<f64>::new(1., 0.);

    let mut last_element = Complex::<f64>::new(0.5, 0.);

    let mut repetition_vec = std::collections::HashSet::new();

    repetition_vec.insert(utils::complex_round(last_element));

    for _ in 0..255 {
        last_element = lamb*last_element*(one-last_element);

        if last_element.norm() < -0.5 || last_element.norm() > 1.5 || last_element.is_nan() {
            return None;
        }

        let rounded = utils::complex_round(last_element);

        if repetition_vec.contains(&rounded) {
            return Some((repetition_vec.len() + 1) as u8)
        }

        repetition_vec.insert(rounded);
    }

    None
}

fn main() -> Result<(), std::io::Error>{
    let args: Vec<String> = std::env::args().collect();

    let top_left = utils::parse_complex(&args[1][..(args[1].len() - 1)]).unwrap();

    let bottom_right = utils::parse_complex(&args[2][..(args[2].len() -1)]).unwrap();

    let precision = args[3].parse::<usize>().unwrap(); //does not actually represent amount of divisions

    let grid = utils::Grid::new(
        top_left,
        bottom_right
    );

    let mut canvas = utils::Picture::new(
        ((bottom_right.re-top_left.re) * precision as f64) as usize,
        ((top_left.im-bottom_right.im) * precision as f64) as usize
    );

    for i in 0..canvas.bounds.0 {
        for j in 0..canvas.bounds.1 {
            match convergence_calculator(
                    grid.pixel_to_point(
                        (i, j), 
                        &canvas
                    )
                )
            {
                Some(x) => canvas.paint_pixel((i, j), x)?,
                None    => ()
            };
        }
    }

    canvas.export("test.png")?;

    Ok(())
}
