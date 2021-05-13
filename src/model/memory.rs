pub struct Memory {
    pub reserved: [u16; 0x200/2],
    pub access: [u16; 0xE00/2]
}

impl Memory {
    pub fn init() -> Memory {
        return Memory {
            reserved: [0; 0x200/2],
            access: [0; 0xE00/2]
        }
    }
}