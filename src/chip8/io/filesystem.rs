use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::env::current_dir;
use crate::args::ProgramArg;

const PROGRAM_SIZE_MAX: usize = 3584;

pub struct Program {
    pub name: String,
    pub rom: [u8; PROGRAM_SIZE_MAX],
    pub size: usize,
}

pub struct Filesystem {
    root: PathBuf,
}

impl Filesystem {
    pub fn new(root: PathBuf) -> Self {
        Filesystem{ root }
    }

    pub fn at_current_dir() -> Result<Self, String> {
        let root = current_dir()
            .map_err(|e| e.to_string())?;

        debug!("Current directory: {:?}", root);

        Ok(Self::new(root))
    }

    pub fn load_program(&self, arg: ProgramArg) -> Result<Program, String> {
        let program = match arg {
            ProgramArg::BuiltInProgram(name) => self.load_built_in_program(&name),
            ProgramArg::ProgramFile(path)    => self.load_program_file(&path),
        }?;

        debug!("Loaded program: name={}, size={}B", program.name, program.size);

        Ok(program)
    }

    pub fn load_built_in_program(&self, name: &str) -> Result<Program, String> {
        let data = match name {
            "15puzzle" => &include_bytes!("../../../programs/15puzzle")[..],
            "blinky"   => &include_bytes!("../../../programs/blinky")[..],
            "blitz"    => &include_bytes!("../../../programs/blitz")[..],
            "brix"     => &include_bytes!("../../../programs/brix")[..],
            "connect4" => &include_bytes!("../../../programs/connect4")[..],
            "guess"    => &include_bytes!("../../../programs/guess")[..],
            "hidden"   => &include_bytes!("../../../programs/hidden")[..],
            "invaders" => &include_bytes!("../../../programs/invaders")[..],
            "kaleid"   => &include_bytes!("../../../programs/kaleid")[..],
            "maze"     => &include_bytes!("../../../programs/maze")[..],
            "merlin"   => &include_bytes!("../../../programs/merlin")[..],
            "missile"  => &include_bytes!("../../../programs/missile")[..],
            "pong"     => &include_bytes!("../../../programs/pong")[..],
            "pong2"    => &include_bytes!("../../../programs/pong2")[..],
            "puzzle"   => &include_bytes!("../../../programs/puzzle")[..],
            "syzygy"   => &include_bytes!("../../../programs/syzygy")[..],
            "tank"     => &include_bytes!("../../../programs/tank")[..],
            "tetris"   => &include_bytes!("../../../programs/tetris")[..],
            "tictac"   => &include_bytes!("../../../programs/tictac")[..],
            "ufo"      => &include_bytes!("../../../programs/ufo")[..],
            "vbrix"    => &include_bytes!("../../../programs/vbrix")[..],
            "vers"     => &include_bytes!("../../../programs/vers")[..],
            "wipeoff"  => &include_bytes!("../../../programs/wipeoff")[..],
            _          => return Err(format!("Unknown program: {}.", name)),
        };
        let size = data.len();

        let mut rom = [0u8; PROGRAM_SIZE_MAX];
        rom[0..size].copy_from_slice(data);

        debug!("Loaded built-in program: name={}, size={}", name, size);

        Ok(Program {
            name: name.to_owned(),
            rom,
            size,
        })
    }

    pub fn load_program_file(&self, path: &str) -> Result<Program, String> {
        let path = self.root.join(path);
        let name = path.file_stem()
            .and_then(|stem| stem.to_str())
            .ok_or("Cannot extract program name from file path")?;

        let mut file = File::open(&path)
            .map_err(|e| e.to_string())?;

        let mut rom = [0u8; PROGRAM_SIZE_MAX];
        let size = file.read(&mut rom)
            .map_err(|e| e.to_string())?;

        debug!("Loaded program from file: name={}, size={}, path={:?}", name, size, path);

        Ok(Program {
            name: name.to_owned(),
            rom,
            size,
        })
    }
}

#[cfg(test)]
#[path = "./filesystem_test.rs"]
mod filesystem_test;
