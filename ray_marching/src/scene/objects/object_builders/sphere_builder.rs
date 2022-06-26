use crate::utils::vectors::Vector;
use super::super::spheres::Sphere;

pub(in super::super) struct SphereBuilder {
    pub(in super::super) centre: Vector,
    pub(in super::super) radius: f64,

    pub(in super::super) strf: Option<fn(Sphere, Vector) -> f64>
}

impl SphereBuilder {
    pub fn add_strf(mut self, strf: fn(Sphere, Vector) -> f64) -> SphereBuilder{
        if self.strf == None { self.strf = Some(strf); }
        self
    }

    pub fn build(self) -> Sphere {
        Sphere{
            centre: self.centre,
            radius: self.radius,

            strf: match self.strf {
                Some(x) => x,
                None    => |_, _| 0.
            }
        }
    }  
}