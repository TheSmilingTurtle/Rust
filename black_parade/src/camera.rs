use super::vectors::Vector;
use super::objects::Objects;
use std::vec::Vec;

use image::ColorType;
use image::png::PNGEncoder;
use std::fs::File;

#[derive(Debug)]
pub struct Camera {
    _centre: Vector,
    normal: Vector,

    top_left: Vector, //is useful

    width: usize,
    height: usize,
    pixel_dist: f64,

    max_depth: f64,
    touch_depth: f64,

    pixels: Vec<u8>, //(for now)

    default: u8
}

impl Camera {
    pub fn new(
                centre: Vector, 
                normal: Vector, 
                pixel_dist: f64, 
                (width, height): (usize, usize), 
                (max_depth, touch_depth): (f64, f64),
                default: u8
               ) -> Camera {
        Camera{
                _centre: centre,
                normal: normal, 
                pixel_dist: pixel_dist,
                width: width, height: height,
                max_depth: max_depth, touch_depth: touch_depth,
                top_left: centre + Vector::new(0.0, width as f64, height as f64)/2.0*(pixel_dist as f64), //FIXME: does not yet use the normal
                pixels: vec![0; width*height],
                default: default
              }
    }

    pub fn write_image(self, filename : &str) -> Result<(), std::io::Error> {
        let output = File::create(filename)?;
        
        let encoder = PNGEncoder::new(output);
        encoder.encode(&self.pixels as &[u8], self.width as u32, self.height as u32, ColorType::Gray(8))?;
        
        Ok(())
    }

    pub fn render(
        &mut self,
        objects: Vec<Objects>,
    ) -> Result<(), String> {

        for row in 0..self.height {
            for col in 0..self.width {

                self.pixels[row*self.width + col] = match self.march(self.pixel_to_pos(row, col), &objects) {
                    Some(dist) => dist as u8,
                    None => self.default
                };
            }
        }

        Ok(())
    }

    fn pixel_to_pos(
            &self, 
            row: usize, 
            col: usize
        ) -> (Vector, Vector){  //FIXME: only works for normal = (1, 0, 0) and centre = (0, 0, 0)
        (self.top_left - Vector::new(0.0, row as f64, col as f64)*self.pixel_dist, self.normal) //for simpler future integration of lens effects
    }

    fn march(
            &self, 
            (pos, normal): (Vector, Vector),
            objects: &Vec<Objects>
        ) -> Option<f64> {

        let mut new_pos = pos;
        let mut _count: u8 = 0;

        loop {
            match objects.iter()
                        .map(|x| x.sdf(new_pos))
                        .min_by(|a, b| a.partial_cmp(b).unwrap())
                        .unwrap()
                    {
                x if x > self.max_depth => return None,
                x if x > self.touch_depth => {new_pos = new_pos + normal * x},
                _ => return Some(new_pos.dist(pos))
            }

            //_count += 1;
        }
    }    
    
}