#[derive(Debug, Clone, Copy)]
pub struct G8 {
    value: u8,
}

impl G8 {
    pub fn new(value: u8) -> Self {
        G8 { value }
    }
    pub fn to_vec(self) -> Vec<u8> {
        vec![self.value]
    }
}
