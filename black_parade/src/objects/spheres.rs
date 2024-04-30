use super::super::vectors::Vector;

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    centre: Vector,
    radius: f64,

    strf: fn(Sphere, Vector) -> f64,
}

impl Sphere {
    pub fn new(
                centre: Vector, 
                radius: f64, 
                strfunc: fn(Sphere, Vector) -> f64
            ) -> Sphere {
        Sphere{
            centre: centre, 
            radius: radius, 
            strf: strfunc,
        }
    }

    pub fn dist(self, pos: Vector) -> f64 {
        (self.centre - pos).norm() - self.radius
    }

    pub fn surf(self, pos: Vector) -> Vector {
        self.centre + (self.centre - pos).to_normed()*self.radius
    }

    pub fn sdf(self, pos: Vector) -> f64{
        (self.strf)(self, pos)
    }
}