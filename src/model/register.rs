pub struct Register {
    pub v: [u8; 16],
    pub i: usize
}

impl Register {
    pub fn init() -> Register {
        return Register {
            v: [0; 16],
            i: 0
        }
    }
}