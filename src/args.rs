use clap::{App, Arg};

use super::constants::{BUILT_IN_PROGRAMS, WINDOW_SCALE_DEFAULT};

pub enum ProgramArg {
    BuiltInProgram(String),
    ProgramFile(String),
}

pub struct Args {
    pub window_scale: u32,
    pub program: ProgramArg,
}

impl Args {
    pub fn parse() -> Result<Self, String> {
        let app = App::new("chip8-interpreter")
            .version("0.1.0")
            .author("Dinko Osrecki")
            .about("An interpreter (emulator) for the CHIP-8 programming language.")
            .arg(Arg::with_name("program")
                .long("program")
                .short("p")
                .value_name("PROGRAM")
                .possible_values(&BUILT_IN_PROGRAMS)
                .required_unless("program-file")
                .conflicts_with("program-file")
                .help("Name of a built-in program to load.")
            )
            .arg(Arg::with_name("program-file")
                .long("program-file")
                .short("f")
                .value_name("PROGRAM_FILE")
                .required_unless("program")
                .conflicts_with("program")
                .help("File path to the CHIP-8 program to load.")
            )
            .arg(Arg::with_name("window-scale")
                .long("window-scale")
                .short("s")
                .value_name("WINDOW_SCALE")
                .help("Number to multiply CHIP-8 original window resolution with.")
            )
            .get_matches();

        let window_scale = app.value_of("window-scale")
            .unwrap_or(WINDOW_SCALE_DEFAULT)
            .parse::<u32>()
            .map_err(|e| e.to_string())?;

        let built_in_program = app.value_of("program")
            .map(|p| ProgramArg::BuiltInProgram(p.to_owned()));

        let program_file = app.value_of("program-file")
            .map(|p| ProgramArg::ProgramFile(p.to_owned()));

        let program = built_in_program.unwrap_or_else(|| program_file.unwrap());

        Ok(Args {
            window_scale,
            program,

        })
    }
}
