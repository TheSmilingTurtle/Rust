use crate::utils::vectors::Vector;
use crate::utils::colours::Colour;
use crate::scene::objects::objects::Objects::{Object, Empty};

pub trait Sdf {
    fn sdf(&self, pos: Vector) -> f64;

    fn surf(&self, pos: Vector) -> Vector;

    fn dist(&self, pos: Vector) -> f64;
}

pub trait Col {
    fn get_colour(&self, pos: &Vector) -> Colour;
}

pub enum Objects<T> 
where T: Sdf + Col {
    Object(T),
    Empty
}

impl<T> Sdf for Objects<T> 
where T: Sdf + Col{
    fn sdf(&self, pos: Vector) -> f64 {
        match self {
            Object(x)   => x.sdf(pos),
            Empty        => 0.
        }
    }

    fn surf(&self, pos: Vector) -> Vector {
        match self {
            Object(x)   => x.surf(pos),
            Empty        => pos,
        }
    }

    fn dist(&self, pos: Vector) -> f64 {
        match self {
            Object(x)   => x.dist(pos),
            Empty        => 0.
        }
    }

}