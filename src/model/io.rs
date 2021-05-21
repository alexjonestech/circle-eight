use minifb::{Window, WindowOptions, Key};
use crate::util::constant::{WIDTH, HEIGHT, REFRESH_NANOS, BG, FG};
use crate::util::bit::has_bit_at;

pub struct Io {
    pub window: Window,
    pub buffer: [u32; WIDTH*HEIGHT],
    pub input: Option<Key>
}

impl Io {
    pub fn init() -> Io {
        let mut window = 
            Window::new(
                "Circle8", WIDTH, HEIGHT, 
                WindowOptions {
                    resize: true, 
                    scale: minifb::Scale::X32, 
                    ..WindowOptions::default()})
                .unwrap();
        window.limit_update_rate(
            Some(std::time::Duration::from_nanos(REFRESH_NANOS)));
        return Io { 
            window: window, 
            buffer: [BG; WIDTH*HEIGHT],
            input: None
        };
    }

    pub fn clear_buffer(&mut self) {
        self.buffer = [BG; WIDTH*HEIGHT];
    }

    pub fn draw(&mut self, sprite: &[u8], x: usize, y: usize) -> bool {
        let mut collision = false;
        for row in 0..sprite.len() {
            for n in 0..8usize {
                if has_bit_at(n, sprite[row]) {
                    collision |= 
                        self.draw_pixel(n+x%WIDTH, row+y%HEIGHT);
                }
            }
        }
        return collision;
    }

    fn draw_pixel(&mut self, x: usize, y: usize) -> bool {
        if (x >= WIDTH) | (y >= HEIGHT) { return false };
        let i = x + y*WIDTH;
        match self.buffer[i] {
            BG => {
                self.buffer[i] = FG;
                return false;
            }
            FG => {
                self.buffer[i] = BG;
                return true;
            }
            _  => panic!("Don't put other colors on the screen!")
        }
    }

    pub fn key_from(nibble: u8) -> Option<Key> {
        return match nibble {
            0x1 => Some(Key::Key1),
            0x2 => Some(Key::Key2),
            0x3 => Some(Key::Key3),
            0xC => Some(Key::Key4),
            0x4 => Some(Key::Q),
            0x5 => Some(Key::W),
            0x6 => Some(Key::E),
            0xD => Some(Key::R),
            0x7 => Some(Key::A),
            0x8 => Some(Key::S),
            0x9 => Some(Key::D),
            0xE => Some(Key::F),
            0xA => Some(Key::Z),
            0x0 => Some(Key::X),
            0xB => Some(Key::C),
            0xF => Some(Key::V),
            _   => None
        }
    }
    
    pub const fn nibble_from(key: Key) -> Option<u8> {
        return match key {
            Key::Key1 => Some(0x1),
            Key::Key2 => Some(0x2),
            Key::Key3 => Some(0x3),
            Key::Key4 => Some(0xC),
            Key::Q    => Some(0x4),
            Key::W    => Some(0x5),
            Key::E    => Some(0x6),
            Key::R    => Some(0xD),
            Key::A    => Some(0x7),
            Key::S    => Some(0x8),
            Key::D    => Some(0x9),
            Key::F    => Some(0xE),
            Key::Z    => Some(0xA),
            Key::X    => Some(0x0),
            Key::C    => Some(0xB),
            Key::V    => Some(0xF),
            _         => None
        }
    }
}