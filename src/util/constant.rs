pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;
pub const BG: u32 = rgb(0, 0, 30);
pub const FG: u32 = rgb(180, 180, 180);

const fn rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}