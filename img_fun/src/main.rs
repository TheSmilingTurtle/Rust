use image::{Rgb, RgbImage, Pixel};
use num::Complex;

const SIZE: (u32, u32) = (4000, 4000);
const SIDELENGTHS: (f64, f64) = (4., 4.);
const OFFSET: (f64, f64) = (-2., 2.);

fn colour(i: usize) -> Rgb<u8> {
    //Rgb([255 - 5 * i as u8, 255 - 5 * i as u8, 255 - 5 * i as u8]) //black and white for depth 51
    //Rgb( [ 255 - (i * 7 + 20) as u8, 255 - (i * i + 20) as u8, 255 - (i * i / 15 + 50) as u8 ] )

    Rgb([
        (i * 5 + 100).clamp(0, 255) as u8,
        (i * 5).clamp(0, 255) as u8,
        (i * 5 + 100).clamp(0, 255) as u8,
    ]) //Purple, need ot set (0, 0, 0) manually
}

fn pixel_to_point(x: f64, y: f64) -> Complex<f64> {
    Complex::new(
        (x / (SIZE.0 as f64) * SIDELENGTHS.0 as f64) + OFFSET.0,
        (-y / (SIZE.1 as f64) * SIDELENGTHS.1 as f64) + OFFSET.1,
    )
}

fn func(x: u32, y: u32) -> Rgb<u8> {
    let mut z = Complex::new(0.5, 0.);
    let _p = Complex::new(1.0, 0.);
    let c = pixel_to_point(x as f64, y as f64);

    for i in 0..51 {
        z = c * z * (_p - z); //c * (z.exp() - z);

        if z.norm_sqr() > 100. {
            return colour(i);
        }
    }

    Rgb([0 as u8, 0 as u8, 0 as u8])
}

fn main() {
    let start = std::time::Instant::now();

    let image = RgbImage::from_fn(SIZE.0, SIZE.1, func);
    let duration = start.elapsed().as_millis();

    println!("It took {}ms", duration);

    let _ = image.save("./test.png");
}
