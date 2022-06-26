//use crate::scene::objects::objects::Objects;

pub trait Ray {
    /* Trait for calculating light rays to and/or from the light*/
}

pub enum Lights<T> 
where T: Ray {
    Light(T),
    None
}

impl<T> Ray for Lights<T> 
where T: Ray {
    /* Needs implementation */
}