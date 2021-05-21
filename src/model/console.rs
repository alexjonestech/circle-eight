use std::time::Instant;

use crate::model::cpu::Cpu;
use crate::model::memory::Memory;
use crate::model::io::Io;
use crate::model::instruction::{read, execute};
use crate::util::constant::{ WIDTH, HEIGHT, CLOCK_NANOS, CLOCKS_PER_REFRESH };

use minifb::{Key, KeyRepeat};


pub struct Console {
    pub cpu: Cpu,
    pub memory: Memory,
    pub io: Io,
    pub clock: u64
}

impl Console {
    pub fn boot(rom_path: String) {
        let console = &mut Console::init(rom_path);
        while console.is_running() {
            let time = std::time::Instant::now();
            console.cycle();
            sleep_remaining(time);
        }
    }

    fn cycle(&mut self) {
        self.clock_cycle();
        if self.clock == CLOCKS_PER_REFRESH {
            self.refresh_cycle();
        }
    }

    fn clock_cycle(&mut self) {
        let opcode = read(self);
        self.cpu.program_counter += 2;
        execute(opcode, self);
        self.clock += 1;
    }

    fn refresh_cycle(&mut self) {
        self.io.window
            .update_with_buffer(
                &self.io.buffer, 
                WIDTH, HEIGHT)
            .unwrap();
        self.cpu.timer.update();
        self.update_inputs();
        self.clock = 0;
    }

    fn update_inputs(&mut self) {
        self.io.input = 
            self.io.window
                .get_keys_pressed(KeyRepeat::No)
                .map(|mut keys| keys.pop())
                .flatten();
    }

    fn is_running(&self) -> bool {
        return self.io.window.is_open() 
            && self.io.input != Some(Key::Escape);
    }
    
    fn init(rom_path: String) -> Console {
        return Console { 
            cpu: Cpu::init(), 
            memory: Memory::init(rom_path), 
            io: Io::init(),
            clock: 0
        };
    }
}

fn sleep_remaining(time: Instant) {
    std::thread::sleep(
        std::time::Duration::from_nanos(
            CLOCK_NANOS.saturating_sub(
                time.elapsed().as_nanos() as u64)));
}