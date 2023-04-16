#[derive(Debug, Clone, Copy)]
pub struct G8 {
    value: u8,
}

impl G8 {
    pub fn to_vec(self: Self) -> Vec<u8>{
        vec![self.value]
    }
}