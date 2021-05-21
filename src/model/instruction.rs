use crate::model::console::Console;
use crate::model::io::Io;
use crate::util::constant::{
    INSTRUCTION_SIZE, 
    LEAST_SIGNIFICANT_BIT, 
    MOST_SIGNIFICANT_BIT,
    DIGIT_SPRITE_SIZE,
    BYTES
};
use crate::util::bit::*;

pub fn read(console: &mut Console) -> u16 {
    let bytes = console.memory.get_range(console.cpu.program_counter, 2);
    return (bytes[0] as u16) << 2*BYTES | bytes[1] as u16;
}

pub fn execute(opcode: u16, console: &mut Console) {
    match head_from(opcode) { 
        0x0 => match byte_from(opcode) {
            0xE0 => 
                clear_screen(console),
            0xEE => 
                return_from_subroutine(console),
            _    => 
                sys(address_from(opcode))},
        0x1 => 
            jump_to(
                address_from(opcode), 
                console),
        0x2 => 
            call_to(
                address_from(opcode), 
                console),
        0x3 => 
            skip_if_equal(
                register_from(opcode), 
                byte_from(opcode), 
                console),
        0x4 => 
            skip_if_not_equal(
                register_from(opcode), 
                byte_from(opcode), 
                console),
        0x5 =>
            skip_if_equal_registers(
                registers_from(opcode),
                console),
        0x6 => 
            load_into(
                register_from(opcode), 
                byte_from(opcode), 
                console),
        0x7 => 
            add_into(
                register_from(opcode), 
                byte_from(opcode), 
                console),
        0x8 => match nibble_from(opcode) {
            0x0 => 
                load_register_into(
                    registers_from(opcode),
                    console),
            0x1 => 
                or_register_into(
                    registers_from(opcode),
                    console),
            0x2 => 
                and_register_into(
                    registers_from(opcode),
                    console),
            0x3 => 
                xor_register_into(
                    registers_from(opcode),
                    console),
            0x4 => 
                add_register_into(
                    registers_from(opcode),
                    console),
            0x5 => 
                subtract_register_into(
                    registers_from(opcode),
                    console),
            0x6 => 
                right_shift(
                    register_from(opcode),
                    console),
            0x7 => 
                reverse_subtract_register_into(
                    registers_from(opcode),
                    console),
            0xE => 
                left_shift(
                    register_from(opcode),
                    console),
            _   => 
                panic!("Invalid instruction: {:04X}", opcode)}
        0x9 =>
            skip_if_not_equal_registers(
                registers_from(opcode),
                console),        
        0xA => 
            load_into_i(
                address_from(opcode), 
                console),
        0xB => 
            jump_with_offset_to(
                register_from(opcode),
                console),
        0xC => 
            random_into(
                register_from(opcode), 
                byte_from(opcode),
                console),
        0xD => 
            draw_sprite_from_memory(
                registers_from(opcode), 
                nibble_from(opcode) as usize, 
                console),
        0xE => match byte_from(opcode) {
            0x9E => 
                skip_if_key_pressed(
                    register_from(opcode),
                    console),
            0xA1 => 
                skip_if_key_not_pressed(
                    register_from(opcode),
                    console),
            _    => 
                panic!("Invalid instruction: {:04X}", opcode)},
        0xF => match byte_from(opcode) {
            0x07 => 
                load_delay_timer_into(
                    register_from(opcode), 
                    console),
            0x0A => 
                wait_for_key_press(
                    register_from(opcode), 
                    console),
            0x15 => 
                load_register_into_delay_timer(
                    register_from(opcode), 
                    console),
            0x18 => 
                load_register_into_sound_timer(
                    register_from(opcode), 
                    console),
            0x1E => 
                add_register_into_i(
                    register_from(opcode), 
                    console),
            0x29 => 
                load_digit_sprite_into_i(
                    register_from(opcode), 
                    console),
            0x33 => 
                load_decimal_digits_into_memory(
                    register_from(opcode), 
                    console),
            0x55 => 
                dump_registers_into_memory(
                    register_from(opcode), 
                    console),
            0x65 => 
                fill_registers_from_memory(
                    register_from(opcode), 
                    console),
            _    => panic!("Invalid instruction: {:04X}", opcode)},
        _   => 
            panic!("Invalid instruction: {:04X}", opcode)
    };
}

fn sys(_address: usize) {
    // noop
}

fn clear_screen(console: &mut Console) {
    console.io.clear_buffer();
}

fn return_from_subroutine(console: &mut Console) {
    console.cpu.program_counter = console.cpu.stack.pop();
}

fn jump_to(address: usize, console: &mut Console) {
    console.cpu.program_counter = address;
}

fn jump_with_offset_to(address: usize, console: &mut Console) {
    let offset = console.cpu.register.v[0] as usize;
    jump_to(address + offset, console);

}

fn call_to(address: usize, console: &mut Console) {
    console.cpu.stack.push(console.cpu.program_counter);
    jump_to(address, console);
}

fn skip_if_equal(x: usize, value: u8, console: &mut Console) {
    if console.cpu.register.v[x] == value {
        console.cpu.program_counter += INSTRUCTION_SIZE;
    }
}

fn skip_if_equal_registers(registers: (usize,usize), console: &mut Console) {
    let (x,y) = registers;
    let value = console.cpu.register.v[y];
    skip_if_equal(x, value, console);
}

fn skip_if_not_equal(x: usize, value: u8, console: &mut Console) {
    if console.cpu.register.v[x] != value {
        console.cpu.program_counter += INSTRUCTION_SIZE;
    }
}

fn skip_if_not_equal_registers(registers: (usize,usize), console: &mut Console) {
    let (x,y) = registers;
    let value = console.cpu.register.v[y];
    skip_if_not_equal(x, value, console);
}

fn skip_if_key_pressed(x: usize, console: &mut Console) {
    if console.io.input == Io::key_from(console.cpu.register.v[x]) {
        console.cpu.program_counter += INSTRUCTION_SIZE;
    }
}

fn skip_if_key_not_pressed(x: usize, console: &mut Console) {
    if console.io.input != Io::key_from(console.cpu.register.v[x]) {
        console.cpu.program_counter += INSTRUCTION_SIZE;
    }
}

fn load_into(x: usize, value: u8, console: &mut Console) {
    console.cpu.register.v[x] = value;
}

fn add_into(x: usize, value: u8, console: &mut Console) {
    console.cpu.register.v[x] = 
        console.cpu.register.v[x].wrapping_add(value);
}

fn load_register_into(registers: (usize,usize), console: &mut Console) {
    let (x,y) = registers;
    console.cpu.register.v[x] = console.cpu.register.v[y];
}

fn or_register_into(registers: (usize,usize), console: &mut Console) {
    let (x,y) = registers;
    console.cpu.register.v[x] |= console.cpu.register.v[y];
}

fn and_register_into(registers: (usize,usize), console: &mut Console) {
    let (x,y) = registers;
    console.cpu.register.v[x] &= console.cpu.register.v[y];
}

fn xor_register_into(registers: (usize,usize), console: &mut Console) {
    let (x,y) = registers;
    console.cpu.register.v[x] ^= console.cpu.register.v[y];
}

fn add_register_into(registers: (usize,usize), console: &mut Console) {
    let (x,y) = registers;
    let result = 
        console.cpu.register.v[x]
            .overflowing_add(
                console.cpu.register.v[y]);
    console.cpu.register.v[x] = result.0;
    console.cpu.register.v[0xF] = result.1 as u8;

}

fn subtract_register_into(registers: (usize,usize), console: &mut Console) {
    let (x,y) = registers;
    console.cpu.register.v[0xF] = (
        console.cpu.register.v[y] <= console.cpu.register.v[x]
    ) as u8;
    console.cpu.register.v[x] = 
        console.cpu.register.v[x].wrapping_sub(console.cpu.register.v[y]);
}

fn reverse_subtract_register_into(registers: (usize,usize), console: &mut Console) {
    let (x,y) = registers;
    console.cpu.register.v[0xF] = (
        console.cpu.register.v[x] <= console.cpu.register.v[y]
    ) as u8;
    console.cpu.register.v[x] = 
        console.cpu.register.v[y].wrapping_sub(console.cpu.register.v[x]);
}

fn left_shift(x: usize, console: &mut Console) {
    console.cpu.register.v[0xF] = 
        has_bit_at(
            LEAST_SIGNIFICANT_BIT, 
            console.cpu.register.v[x]
        ) as u8;
    console.cpu.register.v[x] = console.cpu.register.v[x] << 1;
}

fn right_shift(x: usize, console: &mut Console) {
    console.cpu.register.v[0xF] = 
        has_bit_at(
            MOST_SIGNIFICANT_BIT, 
            console.cpu.register.v[x]
        ) as u8;
    console.cpu.register.v[x] = console.cpu.register.v[x] >> 1;
}

fn random_into(x: usize, value: u8, console: &mut Console) {
    let random: u8 = rand::random();
    load_into(x, random & value, console);
}

fn load_into_i(address: usize, console: &mut Console) {
    console.cpu.register.i = address;
}

fn add_register_into_i(x: usize, console: &mut Console) {
    console.cpu.register.i += console.cpu.register.v[x] as usize;
}

fn load_digit_sprite_into_i(x: usize, console: &mut Console) {
    console.cpu.register.i = 
        (console.cpu.register.v[x] as usize)*DIGIT_SPRITE_SIZE;
}

fn load_register_into_delay_timer(x: usize, console: &mut Console) {
    console.cpu.timer.delay = console.cpu.register.v[x];
}

fn load_register_into_sound_timer(x: usize, console: &mut Console) {
    console.cpu.timer.sound = console.cpu.register.v[x];
}

fn load_delay_timer_into(x: usize, console: &mut Console) {
    console.cpu.register.v[x] = console.cpu.timer.delay;
}

fn load_decimal_digits_into_memory(x: usize, console: &mut Console) {
    let value = console.cpu.register.v[x];
    let decimal = [
        (value/100)%10, 
        (value/10) %10, 
        (value)    %10
    ];
    console.memory.set_range(console.cpu.register.i, &decimal);
}

fn fill_registers_from_memory(x: usize, console: &mut Console) {
    for n in 0..=x {
        console.cpu.register.v[n] = 
            console.memory.get(
                console.cpu.register.i + n
            );
    }
}

fn dump_registers_into_memory(x: usize, console: &mut Console) {
    for n in 0..=x {
        console.memory.set(
            console.cpu.register.i + n,
            console.cpu.register.v[n]
        );
    }
}

fn wait_for_key_press(x: usize, console: &mut Console) {
    let input = console.io.input.map(Io::nibble_from).flatten();
    if input.is_none() {
        console.cpu.program_counter -= INSTRUCTION_SIZE;
    } else {
        console.cpu.register.v[x] = input.unwrap();
        console.io.input = None;
    }
}

fn draw_sprite_from_memory(registers: (usize,usize), rows: usize, console: &mut Console) {
    let (x,y) = registers;
    let sprite = console.memory.get_range(console.cpu.register.i, rows);
    console.cpu.register.v[0xF] =
        console.io.draw(
            sprite, 
            console.cpu.register.v[x] as usize, 
            console.cpu.register.v[y] as usize
        ) as u8;
}
