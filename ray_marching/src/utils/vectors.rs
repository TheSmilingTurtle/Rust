use std::ops::{Add, Sub, Mul, Div, Neg};
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, Copy)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vector {

    pub fn new<T: Into<f64>, G: Into<f64>, F: Into<f64>>(x: T, y: G, z: F) -> Vector
    { Vector{
        x: x.into(),
        y: y.into(),
        z: z.into()
        }
    }

    pub fn norm(self) -> f64 { (self * self).sqrt() }

    pub fn dist(self, vec: Vector) -> f64 { (self - vec).norm() }

    pub fn to_normed(self) -> Vector { self/self.norm() }

    pub fn cross(self, vec: Vector) -> Vector { Vector::new(
        self.y * vec.z - self.z * vec.y,
        self.z * vec.x - self.x * vec.z,
        self.x * vec.y - self.y * vec.x
        )
    }
}

impl<T> Mul<T> for Vector 
where T: Into<f64> {
    type Output = Vector;

    fn mul(self, factor: T) -> Vector {
        let c: f64 = factor.into();
        Vector::new::<f64, f64, f64>(
            self.x*c,
            self.y*c, 
            self.z*c
        )
    }
}

impl<T> Div<T> for Vector 
where T: Into<f64> {
    type Output = Vector;

    fn div(self, factor: T) -> Vector {
        let c: f64 = factor.into();
        Vector::new::<f64, f64, f64>(
            self.x/c, 
            self.y/c, 
            self.z/c
        )
    }
}

impl Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, vec: Vector) -> Vector {
        vec * self
    }
}

impl Mul<Vector> for i32 {
    type Output = Vector;

    fn mul(self, vec: Vector) -> Vector {
        let c: f64 = f64::from(self); 
        vec * c
    }
}

impl Mul<Vector> for Vector {
    type Output = f64;
    fn mul(self, v: Vector) -> f64 { self.x * v.x + self.y * v.y + self.z * v.z }
}

impl Add<Vector> for Vector {
    type Output = Vector;
    fn add(self, v: Vector) -> Vector {
        Vector::new(
            self.x + v.x, 
            self.y + v.y, 
            self.z + v.z
        ) 
    }
}

impl Neg for Vector {
    type Output = Vector;
    fn neg(self) -> Vector {
        Vector::new(
            -self.x, 
            -self.y, 
            -self.z
        ) 
    }
}

impl Sub<Vector> for Vector {
    type Output = Vector;
    fn sub(self, v: Vector) -> Vector { self + (-v)}
}

impl PartialEq for Vector {

    fn eq(&self, &vec: &Vector) -> bool {
        self.x == vec.x && self.y == vec.y && self.z == vec.z
    }
}
impl Display for Vector {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}
