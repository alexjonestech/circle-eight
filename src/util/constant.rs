pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;

pub const BG: u32 = 0x000022;
pub const FG: u32 = 0xBBBBBB;

pub const DIGIT_SPRITE_SIZE: usize = 5;

pub const INSTRUCTION_SIZE: usize = 2;
pub const REFRESH_NANOS: u64 = 16666668;
pub const CLOCK_NANOS: u64 = 1851852/2;
pub const CLOCKS_PER_REFRESH: u64 = REFRESH_NANOS/CLOCK_NANOS;

pub const LEAST_SIGNIFICANT_BIT: usize = 7;
pub const MOST_SIGNIFICANT_BIT: usize = 0;
pub const BYTES: usize = 4;