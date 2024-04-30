mod vectors;
mod camera;
mod objects;

use vectors::Vector;
use camera::Camera;
use objects::{Objects, lines::Line, spheres::Sphere};

//use std::env;

fn main() -> Result<(), String> {
    //let args: Vec<String> = env::args().collect();

    let mut camera = Camera::new(
        Vector::new(0.0, 0.0, 0.0),
        Vector::new(1.0, 0.0, 0.0).to_normed(),
        0.25,
        (1000, 1000),
        (255.0, 0.01),
        30
    );

    let objects: Vec<Objects> = vec![
        Objects::Sphere(
            Sphere::new(
                Vector::new(90., 0., 0.),
                50.0,
                |obj, pos| obj.dist(pos) + 2.*(obj.surf(pos).y/2.).sin() + 2.*(obj.surf(pos).z/2.).sin()
            )
        ),/*
        Objects::Line(
            Line::new(
                Vector::new(70.0, -30.0, 0.0),
                Vector::new(120.0, 10.0, 20.0),
                30.0,
                true
            )
        )*/
    ];

    //println!("{:#?}", objects);
    
    match camera.render(objects) {
        Ok(_) => (),
        Err(err) => panic!("Error occured in render: {:?}", err)
    }

    camera.write_image("out.png").ok().expect("Could not write image");

    Ok(())
}