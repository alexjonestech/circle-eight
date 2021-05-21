pub struct Timer {
    pub delay: u8,
    pub sound: u8
}

impl Timer {
    pub fn init() -> Timer {
        return Timer {
            delay: 0,
            sound: 0
        }
    }

    pub fn update(&mut self) {
        if self.delay > 0 { self.delay -= 1 }
        if self.sound > 0 { self.sound -= 1 }
    }
}