#[cfg(test)]
use speculate::speculate;

use super::*;

speculate! {
    const ADDRESS_START: usize = 0x100;
    const ADDRESS_NEXT: usize = 0x102;
    const ADDRESS_ONE_AFTER_NEXT: usize = 0x104;

    describe "get_current" {
        test "return current address" {
            let pc = ProgramCounter::new(ADDRESS_START);

            assert_eq!(pc.get_current(), ADDRESS_START);
        }
    }

    describe "get_next" {
        test "return next address" {
            let pc = ProgramCounter::new(ADDRESS_START);

            assert_eq!(pc.get_next(), ADDRESS_NEXT);
        }
    }

    describe "goto_next" {
        test "set program counter to the next address" {
            let mut pc = ProgramCounter::new(ADDRESS_START);

            pc.goto_next();

            assert_eq!(pc.pc, ADDRESS_NEXT);
        }
    }

    describe "skip_next" {
        test "set program counter to one after the next address" {
            let mut pc = ProgramCounter::new(ADDRESS_START);

            pc.skip_next();

            assert_eq!(pc.pc, ADDRESS_ONE_AFTER_NEXT);
        }
    }

    describe "jump" {
        test "set program counter to the given address" {
            let addr = 0x200;
            let mut pc = ProgramCounter::new(ADDRESS_START);

            pc.jump(addr);

            assert_eq!(pc.pc, addr);
        }
    }
}
