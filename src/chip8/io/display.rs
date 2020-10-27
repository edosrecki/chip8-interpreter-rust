use sdl2::Sdl;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;
use crate::chip8::constants::{CHIP8_WIDTH, CHIP8_HEIGHT, WINDOW_TITLE, BG_COLOR, FG_COLOR};

pub struct Display {
    canvas: Canvas<Window>,
    scale: u32,
}

impl Display {
    pub fn new(sdl: &Sdl, scale: u32) -> Result<Self, String> {
        let width = (CHIP8_WIDTH as u32) * scale;
        let height = (CHIP8_HEIGHT as u32) * scale;

        let video = sdl.video()?;
        let window = video
            .window(WINDOW_TITLE, width, height)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;
        let canvas = window
            .into_canvas()
            .build()
            .map_err(|e| e.to_string())?;

        let mut display = Display {
            canvas,
            scale,
        };
        display.clear();

        Ok(display)
    }

    fn clear(&mut self) {
        self.canvas.set_draw_color(BG_COLOR);
        self.canvas.clear();
        self.canvas.present();
    }

    pub fn render(&mut self, pixels: &[[bool; CHIP8_WIDTH]; CHIP8_HEIGHT]) -> Result<(), String> {
        for (y, &row) in pixels.iter().enumerate() {
            for (x, &pixel) in row.iter().enumerate() {
                let x = (x as u32) * self.scale;
                let y = (y as u32) * self.scale;
                let color = if pixel { FG_COLOR } else { BG_COLOR };

                let pixel_scaled = Rect::new(x as i32, y as i32, self.scale, self.scale);

                self.canvas.set_draw_color(color);
                self.canvas.fill_rect(pixel_scaled)?;
            }
        }

        self.canvas.present();
        Ok(())
    }
}
