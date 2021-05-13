use crate::model::cpu::Cpu;
use crate::model::memory::Memory;
use crate::model::graphics::Graphics;
use crate::util::constant::{WIDTH, HEIGHT};
use crate::util::instruction::cycle;
use minifb::Key;

pub struct Console {
    pub cpu: Cpu,
    pub memory: Memory,
    pub graphics: Graphics
}

impl Console {
    pub fn boot() {
        let console = &mut Console::init();
        while console.is_running() {
            console.graphics.window
                .update_with_buffer(
                    &console.graphics.buffer, 
                    WIDTH, HEIGHT)
                .unwrap();
            cycle(console);
        }
    }

    fn is_running(&self) -> bool {
        return self.graphics.window.is_open() 
            && !self.graphics.window.is_key_down(Key::Escape);
    }
    
    fn init() -> Console {
        return Console { 
            cpu: Cpu::init(), 
            memory: Memory::init(), 
            graphics: Graphics::init() 
        };
    }
}