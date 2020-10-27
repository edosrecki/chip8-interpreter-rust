use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

const PROGRAM_SIZE_MAX: usize = 3584;

type Program = [u8; PROGRAM_SIZE_MAX];

pub struct Filesystem {
    root: PathBuf,
}

impl Filesystem {
    pub fn new(root: PathBuf) -> Self {
        Filesystem{ root }
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

        let mut buffer = [0u8; PROGRAM_SIZE_MAX];
        buffer[0..size].copy_from_slice(data);

        Ok(buffer)
    }

    pub fn load_program_file(&self, path: &str) -> Result<Program, String> {
        let path = self.root.join(path);
        let mut file = File::open(&path)
            .map_err(|e| e.to_string())?;

        let mut buffer = [0u8; PROGRAM_SIZE_MAX];
        file.read(&mut buffer)
            .map_err(|e| e.to_string())?;

        Ok(buffer)
    }
}
