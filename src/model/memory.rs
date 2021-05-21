use std::fs::File;
use std::io::Read;

pub struct Memory {
    reserved: [u8; 0x200],
    access: [u8; 0xE00]
}

impl Memory {
    pub fn get(&self, address: usize) -> u8 {
        return if address < 0x200 
            { self.reserved[address] } else 
            { self.access[address-0x200] }
    }

    pub fn get_range(&self, address: usize, length: usize) -> &[u8] {
        if address < 0x200 && address+length > 0x200 
            { panic!("Cannot get range accross boundary") }
        return if address < 0x200 
            { &self.reserved[address..address+length] } else 
            { &self.access[(address-0x200)..(address-0x200)+length] }
    }

    pub fn set(&mut self, address: usize, value: u8) {
        if address < 0x200 { panic!("Cannot set reserved memory") }
        self.access[address-0x200] = value;
    }

    pub fn set_range(&mut self, address: usize, values: &[u8]) {
        if address < 0x200 { panic!("Cannot set reserved memory") }
        &self.access[(address-0x200)..(address-0x200)+values.len()]
            .clone_from_slice(values);
    }

    pub fn init(path: String) -> Memory {
        return Memory {
            reserved: load_digit_sprites(),
            access: load_rom(path)
        }
    }
}

fn load_digit_sprites() -> [u8; 0x200] {
    let mut digit_sprites = [0u8; 0x200];
    digit_sprites[0..80]
        .clone_from_slice(&[
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80  // F
        ]);
    return digit_sprites;
}

fn load_rom(path: String) -> [u8; 0xE00] {
    let mut rom = [0u8; 0xE00];
    File::open(path).unwrap()
         .read(&mut rom).unwrap();
    return rom;
}
