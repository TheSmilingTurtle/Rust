use crate::utils::vectors::Vector;
use super::camera::Camera;

pub(super) struct CameraBuilder {
    pub(super) normal: Vector,
    pub(super) centre: Vector,

    pub(super) top: Option<Vector>,
    pub(super) left: Option<Vector>,

    pub(super) width: u16,
    pub(super) height: u16,

    pub(super) pixel_width: Option<f64>,

    pub(super) custom_pos: Option<fn(Vector, Vector) -> Vector>,
    pub(super) custom_normal: Option<fn(Vector, Vector) -> Vector>
}

impl CameraBuilder {
    fn build(self) -> Camera { 
        
        Camera {
            normal: self.normal,
            width: self.width, height: self.height,
            centre: self.centre,
            top: { if let Some(x) = self.top { x } else { Vector::new(0, 0, 1) } }, //FIXME: Do the actual calculation, this is ony true for normal (1, 0, 0)
            left: { if let Some(x) = self.left { x } else { Vector::new(0, -1, 0) } },
            pixel_width: { if let Some(x) = self.pixel_width { x } else { 0.25 /* Default */ } },
            custom_normal: { if let Some(x) = self.custom_normal { x } else { |x, s| x + s } },
            custom_pos: { if let Some(x) = self.custom_normal { x } else { |_, s| s } }
        }
     }

    fn add_custom_normal(mut self, custom_normal: fn(Vector, Vector) -> Vector) -> CameraBuilder { 
        self.custom_normal = Some(custom_normal); 
        self 
    }

    fn add_custom_pos(mut self, custom_normal: fn(Vector, Vector) -> Vector) -> CameraBuilder { 
        self.custom_normal = Some(custom_normal); 
        self 
    }
}