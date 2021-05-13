use crate::model::console::Console;

pub fn cycle(console: &mut Console) {
    let opcode = read(console);
    match opcode { _ => return }; // TODO: Implement instruction set
}

fn read(console: &mut Console) -> u16 {
    return console.memory.access[
        console.cpu.program_counter
    ];
}