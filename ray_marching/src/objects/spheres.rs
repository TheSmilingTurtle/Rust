use crate::vectors::Vector;
use objectBuilders::sphereBuilder::SphereBuilder;

struct Sphere {
    centre: Vector,
    radius: f64,

    strf: Fn(Sphere, Vector)
}

impl Sphere {
    fn new(self, centre: Vector, radius: T) -> SphereBuilder {
        SphereBuilder{
            centre = Some(centre),
            radius = Some(radius),
            strf = None
        }
    }
}

impl Sdf for Sphere {

    fn sdf(&self, pos: &Vector) -> f64 {
        self.centre.dist(pos) - self.radius()
    }

    fn surf(&self, pos: &Vector) -> Vector {
        pos + (self.centre - pos).to_normed() * self.sdf()
    }
}