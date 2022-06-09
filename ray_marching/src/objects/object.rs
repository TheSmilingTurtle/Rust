use crate::vectors::Vector;

trait Sdf {
    fn sdf(&self, pos: &Vector) -> f64 { /* SDF code here */ }

    fn surf(&self, pos: &Vector) -> Vector { /* Nearest Point on the surface of the object*/ }
}

trait Colour {
    fn get_colour(&self, pos: &Vector) -> Colour { /* Get the colour of the object at that point*/ }
}

enum Objects<T> 
where T: Sdf + Colour{
    Object(T),
    None
}

impl Sdf for Object {
    fn sdf(&self, pos: &Vector) -> f64 {
        if let Object(x) == self {
            x.sdf()
        }
        else {
            0.
        }
    }

    fn surf(&self, pos: &Vector) -> f65 {
        if let Object(x) == self {
            x.surf()
        }
        else {
            pos
        }
    }

}