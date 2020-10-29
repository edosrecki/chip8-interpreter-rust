#![feature(map_first_last)]
#[macro_use] extern crate log;

mod args;
mod chip8;
mod constants;
mod run;
mod util;

fn main() {
    env_logger::init();

    run::run().unwrap_or_else(|e| error!("Error: {}", e));
}
