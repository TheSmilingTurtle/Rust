struct SphereBuilder {
    centre: Vector,
    radius: f64,

    strf: Option<Fn() -> f64>
}

impl<T> SphereBuilder 
where T: Infor<f64> {
    fn add_strf(self, strf: Fn(Sphere, Vector) -> f64) -> SphereBuilder{
        if self.strf == None { self.strf = Some(strf); }
        self
    }

    fn build(self) -> Sphere {
        Sphere{
            centre = self.centre,
            radius = self.radius,

            strf = match self.strf {
                Some(x) => x,
                None    => |_, _| 0.
        }
    }
}