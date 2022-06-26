use std::vec::Vec;
use super::lights::{lights::{Ray, Lights}};
use super::objects::objects::{Objects, Sdf, Col};
use crate::camera::camera::Camera;

struct Scene<T, B> 
where T: Ray, B: Sdf + Col{
    lights: Vec<Lights<T>>, //might be a problem bc i might not be able to append two different types to this, so i might need to revert back to my wrapper method

    objects: Vec<Objects<B>>,

    camera: Camera
}