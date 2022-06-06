use std::ops::{Add, Sub, Neg, Mul, Div};

#[derive(Debug, Clone, Copy)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

#[allow(dead_code)]
impl Vector{
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector{x: x, y: y, z: z}
    }

    /// Has form (x, y, z)
    pub fn to_string(self) -> String {
        format!("({}, {}, {})", self.x, self.y, self.z)
    }

    /// Norm of the Vector
    pub fn norm(self) -> f64 {
        (self * self).sqrt()
    }

    /// Distance to another Vector
    pub fn dist(self, vec: Vector) -> f64 {
        (self - vec).norm()
    }

    /// Norms Vector
    pub fn to_normed(self) -> Vector {
        self/self.norm()
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, vec: Vector) -> Vector {
        Vector{x: (self.x + vec.x), y: (self.y + vec.y), z: (self.z + vec.z)}
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        Vector{x: -self.x, y: -self.y, z: -self.z}
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, vec: Vector) -> Vector {
        self + (-vec)
    }
}

impl Mul<Vector> for Vector {
    type Output = f64;

    fn mul(self, vec: Vector) -> f64 {
        (self.x * vec.x) + (self.y * vec.y) + (self.z * vec.z)
    }    
}

impl Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, scal: f64) -> Vector {
        Vector{x: (self.x * scal), y: (self.y * scal), z: (self.z * scal)}
    }    
}

impl Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, vec: Vector) -> Vector {
        Vector{x: (vec.x * self), y: (vec.y * self), z: (vec.z * self)}
    }
}

impl Div<f64> for Vector {
    type Output = Vector;

    fn div(self, scal: f64) -> Vector {
        Vector{x: (self.x / scal), y: (self.y / scal), z: (self.z / scal)}
    }
}