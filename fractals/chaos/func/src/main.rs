mod utils {
    use image::ColorType;
    use image::codecs::png::PngEncoder;
    use image::ImageEncoder;
    use std::fs::File;

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

        fn paint_pixel(&mut self, pixel: (usize, usize), colour: u8) -> Result<(), std::io::Error> {
            if pixel.0 >= self.bounds.0 || pixel.1 >= self.bounds.1 {
                return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("Out of bounds, image bounds are ( {}, {}), ( {}, {}) is out of bounds.", self.bounds.0, self.bounds.1, pixel.0, pixel.1)));
            }

            self.image[pixel.1 * self.bounds.0 + pixel.0] = colour;
            Ok(())
        }

        pub fn paint_circle(&mut self, centre: (usize, usize), radius: usize, colour: u8) -> Result<(), std::io::Error> {
            if centre.0 > self.bounds.0 || centre.1 > self.bounds.1 || centre.0 < radius || centre.1 < radius {
                return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("Out of bounds, image bounds are ( {}, {}), ( {}, {}) is out of bounds.", self.bounds.0, self.bounds.1, centre.0, centre.1)));
            }

            let squared_r = radius * radius;
            let doubled_r = 2 * radius;

            for i in 0..=(doubled_r) {
                for j in 0..=(doubled_r) {
                    if i * i + j * j + squared_r <= doubled_r * (i + j) {
                        let _ = self.paint_pixel((centre.0 - radius + i, centre.1 - radius + j), colour);
                    }
                }
            }

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

}

fn main() -> Result<(), std::io::Error>{
    let iterations = 200usize;
    let first_term = 0.2f32;

    let min_lamb = -2f32;
    let max_lamb = 6f32;

    let mut canvas = utils::Picture::new(3000, 1000);

    for i in 0..(3000) {
        let mut out_list = vec![first_term];
        let lamb = min_lamb + (max_lamb * i as f32)/3000.;

        for j in 0..iterations {
            let n = out_list[j];
            out_list.push(lamb * n * (1.0 - n));
        }

        let x_pos = (i as f64 /(3000.) * (canvas.bounds.0 as f64)) as usize;

        for k in {
            let mut temp = out_list.into_iter().skip(20).collect::<Vec<f32>>();
            temp.sort_by(|a, b| a.partial_cmp(b).unwrap());
            temp.dedup();
            temp
            }
        {
            let _ = canvas.paint_circle((x_pos, ((1.5 - k)/2.0 * (canvas.bounds.1 as f32)) as usize), 0, 0);
        }
    }

    
    canvas.export("test.png")?;

    Ok(())
}
