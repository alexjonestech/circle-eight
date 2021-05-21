mod model;
mod util;
use crate::model::console::Console;

fn main() {
    let rom = get_rom_path();
    Console::boot(rom);
}

fn get_rom_path() -> String {
    let mut args = std::env::args().collect::<Vec<String>>();
    args.remove(0);
    return args.pop()
        .ok_or_else(|| 
            panic!("Must provide path to ROM file"))
        .unwrap();
}