mod constants;
use crate::constants::*;
use minifb::{Key, Window, WindowOptions};

fn main() {
    run(init());
}

fn init() -> (Window, [u32; WIDTH*HEIGHT]) {
    let mut window = 
        Window::new(
            "Circle8", WIDTH, HEIGHT, 
            WindowOptions {
                resize: true, 
                scale: minifb::Scale::X32, 
                ..WindowOptions::default()})
            .unwrap();
    window.limit_update_rate(
        Some(std::time::Duration::from_millis(16)));
    return (window, [BG; WIDTH * HEIGHT]);
}

fn run(window_buffer: (Window, [u32; WIDTH*HEIGHT])) {
    let (mut window, mut buffer) = window_buffer;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}