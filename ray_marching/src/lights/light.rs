use crate::objects::Object;

trait Ray {
    /* Trait for calculating light rays to and/or from the light*/
}

enum Lights<T> 
where T: Ray {
    Light(T),
    None
}

impl Ray for Lights {
    /* Needs implementation */
}