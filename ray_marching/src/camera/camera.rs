use crate::utils::vectors::Vector;
use super::camera_builder::CameraBuilder;

pub struct Camera {
    pub(super) normal: Vector,
    pub(super) centre: Vector,

    pub(super) top: Vector,
    pub(super) left: Vector,

    pub(super) width: u16,
    pub(super) height: u16,

    pub(super) pixel_width: f64,

    pub(super) custom_pos: fn(Vector, Vector) -> Vector,
    pub(super) custom_normal: fn(Vector, Vector) -> Vector
}

impl Camera {
    fn new(normal: Vector, centre: Vector, (width, height): (u16, u16)) -> CameraBuilder {
        CameraBuilder{
            normal,
            centre,

            width, height,

            top: None,
            left: None,

            pixel_width: None,
            
            custom_normal: None,
            custom_pos: None
        }
    }

    fn pixel_to_point(&self, (row, col): (u16, u16)) -> (Vector, Vector) {
        let pixel: Vector = self.top * (self.height/2 - row) - self.left * (self.width/2 - col);
        ((self.custom_pos)(pixel, self.centre), (self.custom_normal)(pixel, self.normal))
    }
}