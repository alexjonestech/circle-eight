use minifb::{Window, WindowOptions};
use crate::util::constant::{WIDTH, HEIGHT, BG};

pub struct Graphics {
    pub window: Window,
    pub buffer: [u32; WIDTH*HEIGHT]
}

impl Graphics {
    pub fn init() -> Graphics {
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
        return Graphics { 
            window: window, 
            buffer: [BG; WIDTH*HEIGHT] 
        };
    }
}