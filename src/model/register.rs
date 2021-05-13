pub struct Register {
    pub v: [u8; 16],
    pub i: u16,
    pub delay_timer: u8,
    pub sound_timer: u8
}

impl Register {
    pub fn init() -> Register {
        return Register {
            v: [0; 16],
            i: 0,
            delay_timer: 0,
            sound_timer: 0
        }
    }
}