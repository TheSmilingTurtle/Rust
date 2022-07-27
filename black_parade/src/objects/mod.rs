pub mod spheres;
pub mod lines;

use super::vectors::Vector;
use spheres::Sphere;
use lines::Line;

#[derive(Debug, Copy, Clone)]
pub enum Objects {
    Sphere(Sphere),
    Line(Line)
}

impl Objects {
    pub fn sdf(self, pos: Vector) -> f64 {
        match self {
            Objects::Line(x) => x.sdf(pos),
            Objects::Sphere(x)  => x.sdf(pos)
        }
    }
}