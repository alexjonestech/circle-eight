use crate::model::register::Register;
use crate::model::timer::Timer;
use crate::model::stack::Stack;

pub struct Cpu {
    pub register: Register,
    pub timer: Timer,
    pub stack: Stack,
    pub program_counter: usize
}

impl Cpu {
    pub fn init() -> Cpu {
        return Cpu { 
            register: Register::init(), 
            timer: Timer::init(),
            stack: Stack::init(),
            program_counter: 0x200
        }
    }
}