use super::super::vectors::Vector;

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    centre: Vector,
    radius: f64,
    dominance: bool,
}

impl Sphere {
    pub fn new(
                centre: Vector, 
                radius: f64, 
                dominance: bool
            ) -> Sphere {
        Sphere{
            centre: centre, 
            radius: radius, 
            dominance: dominance
        }
    }
    
    pub fn sdf(self, pos: Vector) -> f64{
        (self.centre - pos).norm() - self.radius
    }
}