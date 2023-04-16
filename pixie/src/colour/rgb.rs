#[derive(Debug, Clone, Copy)]
pub struct Rgb8 {
    r: u8,
    g: u8,
    b: u8,
}

impl Rgb8 {
    pub fn to_vec(self: Self) -> Vec<u8> {
        vec![self.r, self.g, self.b]
    }
}
