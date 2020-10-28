#![cfg(test)]
use speculate::speculate;
use tempfile::NamedTempFile;
use std::env::current_dir;
use std::io::Error;

use super::*;

speculate! {
    describe "load_built_in_program" {
        test "load each one of 23 programs" {
            let programs: [(&str, usize); 23] = [
                ("15puzzle", 384), ("blinky", 2356), ("blitz", 391), ("brix", 280), ("connect4", 194),
                ("guess", 148), ("hidden", 850), ("invaders", 1283), ("kaleid", 120), ("maze", 34),
                ("merlin", 345), ("missile", 180), ("pong", 246), ("pong2", 264), ("puzzle", 184),
                ("syzygy", 946), ("tank", 560), ("tetris", 494), ("tictac", 486), ("ufo", 224),
                ("vbrix", 507), ("vers", 230), ("wipeoff", 206)
            ];
            let filesystem = build_filesystem();

            for &(name, size) in programs.iter() {
                let program = filesystem.load_built_in_program(name).unwrap();

                assert_eq!(program.name, name);
                assert_eq!(program.size, size);
            }
        }
    }

    describe "load_program_file" {
        test "load a program from a file" {
            let rom = include_bytes!("../../../programs/15puzzle");
            let size = 384;
            let file = save_to_temp_file(&rom[..]).unwrap();
            let path = file.path().to_str().unwrap();

            let filesystem = build_filesystem();
            let program = filesystem.load_program_file(path).unwrap();

            assert_eq!(program.size, size);
        }
    }

    fn build_filesystem() -> Filesystem {
        let root = current_dir().unwrap();
        Filesystem::new(root)
    }

    fn save_to_temp_file(bytes: &[u8]) -> Result<NamedTempFile, Error> {
        let mut tmp_program_file = NamedTempFile::new()?;
        tmp_program_file.write_all(bytes)?;
        tmp_program_file.flush()?;

        Ok(tmp_program_file)
    }
}