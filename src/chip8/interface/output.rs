use crate::chip8::constants::{CHIP8_WIDTH, CHIP8_HEIGHT};

pub struct Output {
    pub pixels: [[bool; CHIP8_WIDTH]; CHIP8_HEIGHT],
    pub have_pixels_changed: bool,
    pub is_sound_on: bool,
}
