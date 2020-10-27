use std::env::current_dir;

use super::chip8::io::display::Display;
use super::chip8::io::keypad::Keypad;
use super::chip8::io::sound::Sound;
use super::chip8::io::filesystem::Filesystem;
use super::chip8::cpu::Processor;
use super::chip8::runner::Runner;
use super::args::{Args, ProgramArg};

pub fn run() {
    let Args { window_scale, program } = Args::parse()
        .expect("Cannot parse command line arguments.");

    let sdl = sdl2::init()
        .expect("Cannot initialize SDL2 context");

    let display = Display::new(&sdl, window_scale)
        .expect("Cannot initialize Display");

    let keypad = Keypad::new(&sdl)
        .expect("Cannot initialize Keypad");

    let sound = Sound::new(&sdl)
        .expect("Cannot initialize Sound");

    let root = current_dir()
        .expect("Cannot get current directory");

    let filesystem = Filesystem::new(root);

    let program_rom = match program {
        ProgramArg::BuiltInProgram(name) => filesystem.load_built_in_program(&name),
        ProgramArg::ProgramFile(path)    => filesystem.load_program_file(&path),
    }.expect("Cannot load program");

    let mut processor = Processor::new();
    processor.load_program(&program_rom);

    let mut runner = Runner::new(display, keypad, sound, processor);
    runner.run_loop();
}
