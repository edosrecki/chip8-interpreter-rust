use super::chip8::io::display::Display;
use super::chip8::io::keypad::Keypad;
use super::chip8::io::sound::Sound;
use super::chip8::io::filesystem::{Filesystem, Program};
use super::chip8::cpu::Processor;
use super::chip8::system::System;
use super::args::Args;

pub fn run() -> Result<(), String> {
    let Args { window_scale, program } = Args::parse()?;

    let Program { name, rom, .. } =  Filesystem::at_current_dir()?.load_program(program)?;

    let sdl = sdl2::init()?;
    let display = Display::new(&sdl, name, window_scale)?;
    let keypad = Keypad::new(&sdl)?;
    let sound = Sound::new(&sdl)?;

    let mut processor = Processor::new();
    processor.load_program(&rom);

    let mut system = System::new(display, keypad, sound, processor);
    system.run_loop();

    Ok(())
}
