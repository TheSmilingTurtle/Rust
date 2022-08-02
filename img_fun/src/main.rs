use image::{ RgbImage, Rgb };
use num::Complex;

const SIZE: (u32, u32) = (30_000, 40_000);
const SIDELENGTHS: (f64, f64) = (3., 4.);
const OFFSET: (f64, f64) = (-1.5, 2.);

const fn colour(i: usize) -> Rgb<u8> {
    Rgb( [255 - 5 * i as u8, 255 - 5 * i as u8, 255 - 5 * i as u8] )
    //Rgb( [ (i * 7 + 20) as u8, (i * i + 20) as u8, (i * i / 15 + 50) as u8 ] )

    //Rgb( [ (i * 5 + 100) as u8, (i * 5) as u8, (i * 5 + 100) as u8] )
}

fn pixel_to_point(x: f64, y: f64) -> Complex<f64> {
    Complex::new(
        (x/(SIZE.0 as f64) * SIDELENGTHS.0 as f64) + OFFSET.0,
        (-y/(SIZE.1 as f64) * SIDELENGTHS.1 as f64) + OFFSET.1
    )
}

fn func(x: u32, y: u32) -> Rgb<u8> {
    let mut z = Complex::new( 0., 0.);
    let _p = Complex::new(1.0, 0.);
    let c = pixel_to_point(x as f64, y as f64);

    for i in 0..51 {

        z = {
            c * (z.exp() - z)
        };

        if z.norm_sqr() > 4. {
            return colour(i);
        }
    }

    colour(51)
}

fn main() {
    
    let start = std::time::Instant::now();

    let image = RgbImage::from_fn(
        SIZE.0, 
        SIZE.1, 
        func
    );
    let duration = start.elapsed().as_millis();

    println!("It took {}ms", duration);

    let _ = image.save("./test.png");
}