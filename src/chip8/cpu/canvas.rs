use crate::chip8::constants::{CHIP8_WIDTH, CHIP8_HEIGHT};

pub struct Canvas {
    pub pixels: [[bool; CHIP8_WIDTH]; CHIP8_HEIGHT],
    pub have_pixels_changed: bool,
}

impl Canvas {
    pub fn new() -> Self {
        Canvas {
            pixels: [[false; CHIP8_WIDTH]; CHIP8_HEIGHT],
            have_pixels_changed: false,
        }
    }

    pub fn clean(&mut self) {
        for y in 0..CHIP8_HEIGHT {
            for x in 0..CHIP8_WIDTH {
                let old_pixel = self.pixels[y][x];
                self.pixels[y][x] = false;

                if old_pixel != self.pixels[y][x] {
                    self.have_pixels_changed = true;
                }
            }
        }
    }

    pub fn draw_sprite(&mut self, x_left: usize, y_top: usize, sprite: &[u8]) -> bool {
        let mut any_pixel_erased = false;

        for (j, &byte) in sprite.iter().enumerate() {
            for i in 0..8 {
                let x = (x_left + i) % CHIP8_WIDTH;
                let y = (y_top + j) % CHIP8_HEIGHT;
                
                let pixel_mask = {
                    let bit = (byte >> (7 - i)) & 0b1;
                    bit == 1
                };
                
                let old_pixel = self.pixels[y][x];
                self.pixels[y][x] ^= pixel_mask;

                if old_pixel && !self.pixels[y][x] {
                    any_pixel_erased = true;
                }

                if old_pixel != self.pixels[y][x] {
                    self.have_pixels_changed = true;
                }
            }
        }

        any_pixel_erased
    }

    pub fn reset_pixels_changed(&mut self) {
        self.have_pixels_changed = false;
    }
}
