use num::Complex;

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
        /*
        pub fn point_to_pixel(&self, point: Complex<f64>, canvas: &Picture) -> (usize, usize) {
            (
                ((point.re - self.top_left.re) / (canvas.bounds.0 as f64)) as usize,
                ((self.top_left.im - point.im) / (canvas.bounds.1 as f64)) as usize
            )
        }*/

        pub fn pixel_to_point(&self, pixel: (usize, usize), canvas: &Picture) -> Complex<f64> {
            self.top_left + Complex::<f64>::new(
                ((pixel.0 as f64) / (canvas.bounds.0 as f64)) * self.width,
                - ((pixel.1 as f64) / (canvas.bounds.1 as f64)) * self.height
            )
        }

    }

    pub fn complex_round(num: Complex<f64>) -> Complex<f64> {
        let decimals = 2usize;
        let y = 10usize.pow(decimals as u32) as f64;

        Complex::new(
            (num.re * y).round() / y,
            (num.im * y).round() / y
        )
        
    }

}

fn convergence_calculator(lamb: Complex<f64>) -> Option<u8> {
    let one = Complex::<f64>::new(1., 0.);

    let mut last_element = Complex::<f64>::new(0.2, 0.);

    let mut repetition_vec: Vec<Complex<f64>> = vec![utils::complex_round(last_element)];

    for _ in 0..255 {
        last_element = lamb*last_element*(one-last_element);

        if last_element.norm() < -0.5 || last_element.norm() > 1.5 || last_element.is_nan() {
            return None;
        }

        if repetition_vec.contains(&utils::complex_round(last_element)) {
            return Some((repetition_vec.len() + 1) as u8)
        }

        repetition_vec.push(utils::complex_round(last_element));
    }

    None
}

fn main() -> Result<(), std::io::Error>{
    let grid = utils::Grid::new(
        Complex::new(
            -2.1, 1.2
        ),
        Complex::new(
            4.1, -1.2
        )
    );

    let mut canvas = utils::Picture::new(
        31 * 100,
        12 * 100
    );

    for i in 0..canvas.bounds.0 {
        for j in 0..canvas.bounds.1 {
            match convergence_calculator(
                grid.pixel_to_point(
                    (i, j), 
                    &canvas))
            {
                Some(x) => canvas.paint_pixel((i, j), x)?,
                None    => ()
            };
        }
    }

    canvas.export("test.png")?;

    Ok(())
}
