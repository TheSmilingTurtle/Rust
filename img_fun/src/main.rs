use image::{ RgbImage, Rgb };
use num::Complex;

const SIZE: (u32, u32) = (2000, 1000);
const TOTAL: (usize, usize) = (4, 2);
const OFFSET: (isize, isize) = (2, -1);

fn colour(i: usize) -> Rgb<u8> {
    Rgb( [ (i * 7 + 20) as u8, (i * i + 20) as u8, (i * i / 15 + 50) as u8 ] )

    //Rgb( [ (i * 5 + 100) as u8, (i * 5) as u8, (i * 5 + 100) as u8] )
}

fn pixel_to_point(x: f64, y: f64) -> Complex<f64> {
    Complex::new(
        (x/(SIZE.0 as f64) * TOTAL.0 as f64) - OFFSET.0 as f64,
        (-y/(SIZE.1 as f64) * TOTAL.1 as f64) - OFFSET.1 as f64
    )
}

fn func(x: u32, y: u32) -> Rgb<u8> {
    let mut z = Complex::new(0.5, 0.);
    let point = Complex::new(1.0, 0.);
    let constant = pixel_to_point(x as f64, y as f64);

    for i in 0..50 {
        z = constant * z * (point - z);
        if z.norm() > 2. {
            return colour(i);
        }
    }

    colour(50)
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