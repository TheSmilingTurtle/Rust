use super::super::vectors::Vector;

#[derive(Debug, Copy, Clone)]
pub struct Line {
    a: Vector,
    b: Vector,

    ab: Vector, //is normed, therefore needs associated len 
    len: f64,

    radius: f64,

    _dominance: bool
}

impl Line {
    pub fn new(
                a: Vector,
                b: Vector,
                radius: f64,
                dominance: bool,
            ) -> Line {
        Line{
            a: a,
            b: b,
            ab: (b - a).to_normed(),
            len: (b - a).norm(),
            radius: radius,
            _dominance: dominance,
        }
    }

    pub fn sdf(self, pos: Vector) -> f64 {
        match self.ab*(pos - self.a) {
            x if x < 0.0        => (self.a - pos).norm() - self.radius,
            x if x > self.len   => (self.b - pos).norm() - self.radius,
            x                   => ((self.a + self.ab * x) - pos).norm() - self.radius
        }
    }
}