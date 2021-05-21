use crate::util::constant::BYTES;

pub fn has_bit_at(n: usize, byte: u8) -> bool {
    let bit = (byte >> (7 - n)) & 0b1;
    return bit == 1;
}

pub fn head_from(opcode: u16) -> u16 {
    return (opcode & 0xF000) >> 3*BYTES;
}

pub fn nibble_from(opcode: u16) -> u8 {
    return (opcode & 0x000F) as u8;
}

pub fn byte_from(opcode: u16) -> u8 {
    return (opcode & 0x00FF) as u8;
}

pub fn register_from(opcode: u16) -> usize {
    return ((opcode & 0x0F00) >> 2*BYTES) as usize;
}

pub fn registers_from(opcode: u16) -> (usize,usize) {
    return (
        ((opcode & 0x0F00) >> 2*BYTES) as usize,
        ((opcode & 0x00F0) >> 1*BYTES) as usize
    );
}

pub fn address_from(opcode: u16) -> usize {
    return (opcode & 0x0FFF) as usize;
}