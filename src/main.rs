#![feature(map_first_last)]

mod args;
mod chip8;
mod constants;
mod run;
mod util;

fn main() {
    run::run();
}
