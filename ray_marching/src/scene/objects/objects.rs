use crate::utils::vectors::Vector;
use crate::utils::colours::Colour;
use crate::scene::objects::objects::Objects::{Object, Empty};

pub trait Sdf {
    fn sdf(&self, pos: &Vector) -> f64;

    fn surf(&self, pos: &Vector) -> Vector;
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
    fn sdf(&self, pos: &Vector) -> f64 {
        match self {
            Object(x)   => x.sdf(pos),
            Empty        => 0.
        }
    }

    fn surf(&self, pos: &Vector) -> Vector {
        if let Object(x) = self {
            x.surf(pos)
        }
        else {
            *pos
        }
    }

}