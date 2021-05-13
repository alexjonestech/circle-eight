use crate::model::register::Register;

pub struct Cpu {
    pub register: Register,
    pub stack: [u16; 16],
    pub stack_pointer: u8,
    pub program_counter: usize
}

impl Cpu {
    pub fn init() -> Cpu {
        return Cpu { 
            register: Register::init(), 
            stack: [0; 16],
            stack_pointer: 0,
            program_counter: 0x200 
        }
    }
}