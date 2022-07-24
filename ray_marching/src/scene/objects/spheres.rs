use crate::utils::vectors::Vector;
use super::object_builders::sphere_builder::SphereBuilder;
use crate::scene::objects::objects::Sdf;

pub struct Sphere {
    pub centre: Vector,
    pub radius: f64,

    pub strf: fn(&Sphere, Vector) -> f64
}

impl Sphere {
    fn new<T>(self, centre: Vector, radius: T) -> SphereBuilder 
    where T: Into<f64> {
        SphereBuilder{
            centre,
            radius: radius.into(),
            strf: None
        }
    }
}

impl Sdf for Sphere {
    fn surf(&self, pos: Vector) -> Vector {
        pos + (self.centre - pos).to_normed() * self.sdf(pos)
    }

    fn dist(&self, pos: Vector) -> f64 {
        self.centre.dist(pos) - self.radius
    }

    fn sdf(&self, pos: Vector) -> f64 {
        self.dist(pos) + (self.strf)(self, pos)
    }
}