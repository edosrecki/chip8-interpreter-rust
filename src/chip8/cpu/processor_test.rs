#![cfg(test)]
use speculate::speculate;
use lazy_static::lazy_static;
use maplit::btreeset;

use crate::{assert_all_elems_eq, flatten};
use super::*;

speculate! {
    const ADDRESS_START: usize = 0x200;
    const ADDRESS_NEXT: usize = 0x202;
    const ADDRESS_ONE_AFTER_NEXT: usize = 0x204;

    describe "new" {
        test "initialise processor" {
            let processor = Processor::new();

            assert_eq!(processor.memory[..0x50], FONT[..]);
            assert_all_elems_eq!(processor.memory[0x50..], 0);
            assert_eq!(processor.V, [0; 16]);
            assert_eq!(processor.I, 0);
            assert_eq!(processor.pc.get_current(), ADDRESS_START);
            assert_eq!(processor.sp, 0);
            assert_eq!(processor.stack, [0; 16]);
            assert_eq!(processor.delay_timer, 0);
            assert_eq!(processor.sound_timer, 0);
            assert_eq!(processor.waiting_for_keypad, false);
            assert_eq!(processor.keycode_register, 0);
            assert_all_elems_eq!(flatten!(processor.canvas.pixels), false);
        }
    }

    describe "load_program" {
        test "load program from the program start address" {
            let rom = [0x01, 0x02, 0x03, 0xF1, 0x50];
            let mut processor = Processor::new();

            processor.load_program(&rom);

            assert_eq!(processor.memory[0x200..=0x204], rom);
        }
    }

    describe "update_timers" {
        test "decrement timers by 1" {
            let mut processor = Processor::new();
            processor.delay_timer = 100;
            processor.sound_timer = 80;

            processor.update_timers();

            assert_eq!(processor.delay_timer, 99);
            assert_eq!(processor.sound_timer, 79);
        }

        test "do not update timers if waiting for keypad" {
            let mut processor = Processor::new();
            processor.delay_timer = 100;
            processor.sound_timer = 80;
            processor.waiting_for_keypad = true;

            processor.update_timers();

            assert_eq!(processor.delay_timer, 100);
            assert_eq!(processor.sound_timer, 80);
        }

        test "do not update timers if they are equal 0" {
            let mut processor = Processor::new();
            processor.delay_timer = 0;
            processor.sound_timer = 0;

            processor.update_timers();

            assert_eq!(processor.delay_timer, 0);
            assert_eq!(processor.sound_timer, 0);
        }
    }

    describe "execute_opcode" {

        const PRESSED_KEYCODE: u8 = 0x1;
        lazy_static! {
            static ref PRESSED_KEYCODES: BTreeSet<u8> = btreeset! { PRESSED_KEYCODE };
        }

        test "00E0 - clean screen" {
            let mut processor = Processor::new();
            processor.canvas.draw_sprite(0, 0, &[0xF, 0xF, 0xF, 0xF, 0xF]);

            processor.execute_opcode(0x00E0, &PRESSED_KEYCODES);

            assert_all_elems_eq!(flatten!(processor.canvas.pixels), false);
            assert_eq!(processor.pc.get_current(), ADDRESS_NEXT);
        }

        test "00EE - return from subroutine" {
            let addr = 0x300;
            let mut processor = Processor::new();
            processor.stack[0] = addr;
            processor.sp += 1;

            processor.execute_opcode(0x00EE, &PRESSED_KEYCODES);

            assert_eq!(processor.pc.get_current(), addr);
            assert_eq!(processor.sp, 0);
        }

        test "1nnn - jump to address at nnn" {
            let mut processor = Processor::new();

            processor.execute_opcode(0x1ABC, &PRESSED_KEYCODES);

            assert_eq!(processor.pc.get_current(), 0xABC);
        }

        test "2nnn - call subroutine at nnn" {
            let mut processor = Processor::new();

            processor.execute_opcode(0x2ABC, &PRESSED_KEYCODES);

            assert_eq!(processor.pc.get_current(), 0xABC);
            assert_eq!(processor.sp, 1);
            assert_eq!(processor.stack[0], ADDRESS_NEXT);
        }

        test "3xkk - skip next instruction if Vx == kk" {
            let mut processor = Processor::new();
            processor.V[0x1] = 0xAA;

            processor.execute_opcode(0x31AA, &PRESSED_KEYCODES);

            assert_eq!(processor.pc.get_current(), ADDRESS_ONE_AFTER_NEXT);
        }

        test "3xkk - do not skip next instruction if Vx != kk" {
            let mut processor = Processor::new();
            processor.V[0x2] = 0xBB;

            processor.execute_opcode(0x32AA, &PRESSED_KEYCODES);

            assert_eq!(processor.pc.get_current(), ADDRESS_NEXT);
        }

        test "4xkk - skip next instruction if Vx != kk" {
            let mut processor = Processor::new();
            processor.V[0xC] = 0xBB;

            processor.execute_opcode(0x4CAA, &PRESSED_KEYCODES);

            assert_eq!(processor.pc.get_current(), ADDRESS_ONE_AFTER_NEXT);
        }

        test "4xkk - do not skip next instruction if Vx == kk" {
            let mut processor = Processor::new();
            processor.V[0x1] = 0xAA;

            processor.execute_opcode(0x41AA, &PRESSED_KEYCODES);

            assert_eq!(processor.pc.get_current(), ADDRESS_NEXT);
        }

        test "5xy0 - skip next instruction if Vx == Vy" {
            let mut processor = Processor::new();
            processor.V[0x1] = 0xAA;
            processor.V[0x2] = 0xAA;

            processor.execute_opcode(0x5120, &PRESSED_KEYCODES);

            assert_eq!(processor.pc.get_current(), ADDRESS_ONE_AFTER_NEXT);
        }

        test "5xy0 - do not skip next instruction if Vx != Vy" {
            let mut processor = Processor::new();
            processor.V[0x1] = 0xAA;
            processor.V[0x2] = 0xBB;

            processor.execute_opcode(0x5120, &PRESSED_KEYCODES);

            assert_eq!(processor.pc.get_current(), ADDRESS_NEXT);
        }

        test "6xkk - set Vx = kk" {
            let mut processor = Processor::new();

            processor.execute_opcode(0x6A10, &PRESSED_KEYCODES);

            assert_eq!(processor.V[0xA], 0x10);
            assert_eq!(processor.pc.get_current(), ADDRESS_NEXT);
        }

        test "7xkk - set Vx = Vx + kk" {
            let mut processor = Processor::new();
            processor.V[0xA] = 0x10;

            processor.execute_opcode(0x7A20, &PRESSED_KEYCODES);

            assert_eq!(processor.V[0xA], 0x30);
            assert_eq!(processor.pc.get_current(), ADDRESS_NEXT);
        }

        test "8xy0 - set Vx = Vy" {
            let mut processor = Processor::new();
            processor.V[0x2] = 0x10;

            processor.execute_opcode(0x8120, &PRESSED_KEYCODES);

            assert_eq!(processor.V[0x1], 0x10);
            assert_eq!(processor.pc.get_current(), ADDRESS_NEXT);
        }

        test "8xy1 - set Vx = Vx | Vy" {
            let mut processor = Processor::new();
            processor.V[0x1] = 0x10;
            processor.V[0x2] = 0x01;

            processor.execute_opcode(0x8121, &PRESSED_KEYCODES);

            assert_eq!(processor.V[0x1], 0x11);
            assert_eq!(processor.pc.get_current(), ADDRESS_NEXT);
        }

        test "8xy2 - set Vx = Vx & Vy" {
            let mut processor = Processor::new();
            processor.V[0x1] = 0x10;
            processor.V[0x2] = 0x11;

            processor.execute_opcode(0x8122, &PRESSED_KEYCODES);

            assert_eq!(processor.V[0x1], 0x10);
            assert_eq!(processor.pc.get_current(), ADDRESS_NEXT);
        }

        test "8xy3 - set Vx = Vx ^ Vy" {
            let mut processor = Processor::new();
            processor.V[0x1] = 0x10;
            processor.V[0x2] = 0x11;

            processor.execute_opcode(0x8123, &PRESSED_KEYCODES);

            assert_eq!(processor.V[0x1], 0x01);
            assert_eq!(processor.pc.get_current(), ADDRESS_NEXT);
        }

        test "8xy4 - set Vx = Vx + Vy. Set VF = 0 (not carry)" {
            let mut processor = Processor::new();
            processor.V[0xA] = 0xF0;
            processor.V[0xB] = 0x0F;
            processor.V[0xF] = 1;

            processor.execute_opcode(0x8AB4, &PRESSED_KEYCODES);

            assert_eq!(processor.V[0xA], 0xFF);
            assert_eq!(processor.V[0xF], 0);
            assert_eq!(processor.pc.get_current(), ADDRESS_NEXT);
        }

        test "8xy4 - set Vx = Vx + Vy, set VF = 1 (carry)" {
            let mut processor = Processor::new();
            processor.V[0xA] = 0xF0;
            processor.V[0xB] = 0x1F;

            processor.execute_opcode(0x8AB4, &PRESSED_KEYCODES);

            assert_eq!(processor.V[0xA], 0x0F);
            assert_eq!(processor.V[0xF], 1);
            assert_eq!(processor.pc.get_current(), ADDRESS_NEXT);
        }

        test "8xy5 - set Vx = Vx - Vy, set VF = 1 (not carry)" {
            let mut processor = Processor::new();
            processor.V[0xA] = 0xFF;
            processor.V[0xB] = 0x0F;

            processor.execute_opcode(0x8AB5, &PRESSED_KEYCODES);

            assert_eq!(processor.V[0xA], 0xF0);
            assert_eq!(processor.V[0xF], 1);
            assert_eq!(processor.pc.get_current(), ADDRESS_NEXT);
        }

        test "8xy5 - set Vx = Vx - Vy, set VF = 0 (carry)" {
            let mut processor = Processor::new();
            processor.V[0xA] = 0x0F;
            processor.V[0xB] = 0xFF;
            processor.V[0xF] = 1;

            processor.execute_opcode(0x8AB5, &PRESSED_KEYCODES);

            assert_eq!(processor.V[0xA], 0x10);
            assert_eq!(processor.V[0xF], 0);
            assert_eq!(processor.pc.get_current(), ADDRESS_NEXT);
        }

        test "8xy6 - set VF to 1 if least-significant bit of Vx == 1, divide Vx by 2" {
            let mut processor = Processor::new();
            processor.V[0x1] = 0x7;

            processor.execute_opcode(0x8106, &PRESSED_KEYCODES);

            assert_eq!(processor.V[0x1], 0x3);
            assert_eq!(processor.V[0xF], 1);
            assert_eq!(processor.pc.get_current(), ADDRESS_NEXT);
        }

        test "8xy6 - set VF to 0 if least-significant bit of Vx != 1, divide Vx by 2" {
            let mut processor = Processor::new();
            processor.V[0x1] = 0x2;
            processor.V[0xF] = 1;

            processor.execute_opcode(0x8106, &PRESSED_KEYCODES);

            assert_eq!(processor.V[0x1], 0x1);
            assert_eq!(processor.V[0xF], 0);
            assert_eq!(processor.pc.get_current(), ADDRESS_NEXT);
        }

        test "8xy7 - set Vx = Vy - Vx. Set VF = 1 (not carry)" {
            let mut processor = Processor::new();
            processor.V[0xA] = 0x0F;
            processor.V[0xB] = 0xFF;

            processor.execute_opcode(0x8AB7, &PRESSED_KEYCODES);

            assert_eq!(processor.V[0xA], 0xF0);
            assert_eq!(processor.V[0xF], 1);
            assert_eq!(processor.pc.get_current(), ADDRESS_NEXT);
        }

        test "8xy7 - set Vx = Vy - Vx, set VF = 0 (carry)" {
            let mut processor = Processor::new();
            processor.V[0xA] = 0xFF;
            processor.V[0xB] = 0x0F;
            processor.V[0xF] = 1;

            processor.execute_opcode(0x8AB7, &PRESSED_KEYCODES);

            assert_eq!(processor.V[0xA], 0x10);
            assert_eq!(processor.V[0xF], 0);
            assert_eq!(processor.pc.get_current(), ADDRESS_NEXT);
        }

        test "8xyE - set VF to 1 if most-significant bit of Vx == 1, multiply Vx by 2" {
            let mut processor = Processor::new();
            processor.V[0x1] = 0xC1;

            processor.execute_opcode(0x810E, &PRESSED_KEYCODES);

            assert_eq!(processor.V[0x1], 0x82);
            assert_eq!(processor.V[0xF], 1);
            assert_eq!(processor.pc.get_current(), ADDRESS_NEXT);
        }

        test "8xyE - set VF to 0 if most-significant bit of Vx != 1, multiply Vx by 2" {
            let mut processor = Processor::new();
            processor.V[0x1] = 0x41;
            processor.V[0xF] = 1;

            processor.execute_opcode(0x810E, &PRESSED_KEYCODES);

            assert_eq!(processor.V[0x1], 0x82);
            assert_eq!(processor.V[0xF], 0);
            assert_eq!(processor.pc.get_current(), ADDRESS_NEXT);
        }

        test "9xy0 - skip next instruction if Vx != Vy" {
            let mut processor = Processor::new();
            processor.V[0x1] = 0x01;
            processor.V[0x2] = 0x10;

            processor.execute_opcode(0x9120, &PRESSED_KEYCODES);

            assert_eq!(processor.pc.get_current(), ADDRESS_ONE_AFTER_NEXT);
        }

        test "9xy0 - do not skip next instruction if Vx == Vy" {
            let mut processor = Processor::new();
            processor.V[0x1] = 0x10;
            processor.V[0x2] = 0x10;

            processor.execute_opcode(0x9120, &PRESSED_KEYCODES);

            assert_eq!(processor.pc.get_current(), ADDRESS_NEXT);
        }

        test "Annn - set register I = nnn" {
            let mut processor = Processor::new();

            processor.execute_opcode(0xA123, &PRESSED_KEYCODES);

            assert_eq!(processor.I, 0x123);
            assert_eq!(processor.pc.get_current(), ADDRESS_NEXT);
        }

        test "Bnnn - jump to address nnn + V0" {
            let mut processor = Processor::new();
            processor.V[0x0] = 0x1;

            processor.execute_opcode(0xB123, &PRESSED_KEYCODES);

            assert_eq!(processor.pc.get_current(), 0x124);
        }

        test "Cxkk - set Vx = random byte & kk" {
            let mut processor = Processor::new();

            processor.execute_opcode(0xC1F0, &PRESSED_KEYCODES);

            assert_eq!(processor.V[0x1] & 0x0F, 0x00); // test AND operation
            assert_eq!(processor.pc.get_current(), ADDRESS_NEXT);
        }

        test "Dxyn - draw sprite on the screen at (Vx, Vy), reading n bytes starting at address from register I" {
            let mut processor = Processor::new();
            processor.V[0x1] = 10;
            processor.V[0x2] = 20;
            processor.canvas.draw_sprite(10, 20, &[0xFF]);
            processor.I = 0x300;
            processor.memory[0x300] = 0xFF; // erases existing pixels
            processor.memory[0x301] = 0xF2;
            processor.memory[0x302] = 0xF3;

            processor.execute_opcode(0xD123, &PRESSED_KEYCODES);

            assert_eq!(processor.V[0xF], 1); // existing pixels were erased
            assert_eq!(processor.pc.get_current(), ADDRESS_NEXT);
        }

        test "Ex9E - skip next instruction if key with the value of Vx is pressed" {
            let mut processor = Processor::new();
            processor.V[0x1] = PRESSED_KEYCODE;

            processor.execute_opcode(0xE19E, &PRESSED_KEYCODES);

            assert_eq!(processor.pc.get_current(), ADDRESS_ONE_AFTER_NEXT);
        }

        test "Ex9E - do not skip next instruction if key with the value of Vx is not pressed" {
            let mut processor = Processor::new();

            processor.execute_opcode(0xE19E, &PRESSED_KEYCODES);

            assert_eq!(processor.pc.get_current(), ADDRESS_NEXT);
        }

        test "ExA1 - skip next instruction if key with the value of Vx is not pressed" {
            let mut processor = Processor::new();

            processor.execute_opcode(0xE1A1, &PRESSED_KEYCODES);

            assert_eq!(processor.pc.get_current(), ADDRESS_ONE_AFTER_NEXT);
        }

        test "ExA1 - do not skip next instruction if key with the value of Vx is pressed" {
            let mut processor = Processor::new();
            processor.V[0x1] = PRESSED_KEYCODE;

            processor.execute_opcode(0xE1A1, &PRESSED_KEYCODES);

            assert_eq!(processor.pc.get_current(), ADDRESS_NEXT);
        }

        test "Fx07 - set Vx = delay_timer" {
            let mut processor = Processor::new();
            processor.delay_timer = 0xFF;

            processor.execute_opcode(0xF107, &PRESSED_KEYCODES);

            assert_eq!(processor.V[0x1], 0xFF);
            assert_eq!(processor.pc.get_current(), ADDRESS_NEXT);
        }

        test "Fx0A - wait for a key press" {
            let mut processor = Processor::new();

            processor.execute_opcode(0xF10A, &PRESSED_KEYCODES);

            assert_eq!(processor.waiting_for_keypad, true);
            assert_eq!(processor.keycode_register, 0x1);
            assert_eq!(processor.pc.get_current(), ADDRESS_NEXT);
        }

        test "Fx15 - set delay_timer = Vx" {
            let mut processor = Processor::new();
            processor.V[0xA] = 0xFF;

            processor.execute_opcode(0xFA15, &PRESSED_KEYCODES);

            assert_eq!(processor.delay_timer, 0xFF);
            assert_eq!(processor.pc.get_current(), ADDRESS_NEXT);
        }

        test "Fx18 - set sound_timer = Vx" {
            let mut processor = Processor::new();
            processor.V[0xA] = 0xFF;

            processor.execute_opcode(0xFA18, &PRESSED_KEYCODES);

            assert_eq!(processor.sound_timer, 0xFF);
            assert_eq!(processor.pc.get_current(), ADDRESS_NEXT);
        }

        test "Fx1E - set I = I + Vx" {
            let mut processor = Processor::new();
            processor.I = 0xF0;
            processor.V[0x1] = 0x0F;

            processor.execute_opcode(0xF11E, &PRESSED_KEYCODES);

            assert_eq!(processor.I, 0xFF);
            assert_eq!(processor.pc.get_current(), ADDRESS_NEXT);
        }

        test "Fx29 - set I to the location of sprite for digit Vx" {
            let mut processor = Processor::new();
            processor.V[0x1] = 0x7;

            processor.execute_opcode(0xF129, &PRESSED_KEYCODES);

            assert_eq!(processor.I, 0x23);
            assert_eq!(processor.pc.get_current(), ADDRESS_NEXT);
        }

        test "Fx33 - set RAM[I] through RAM[I+2] to hundreds, tens and ones digit of Vx" {
            let mut processor = Processor::new();
            processor.I = 0x300;
            processor.V[0x1] = 123;

            processor.execute_opcode(0xF133, &PRESSED_KEYCODES);

            assert_eq!(processor.memory[0x300], 0x1);
            assert_eq!(processor.memory[0x301], 0x2);
            assert_eq!(processor.memory[0x302], 0x3);
            assert_eq!(processor.pc.get_current(), ADDRESS_NEXT);
        }

        test "Fx55 - store registers V0 through Vx in memory starting at address I" {
            let mut processor = Processor::new();
            processor.V = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0xA, 0xB, 0xC, 0xD, 0xE, 0xF];
            processor.I = 0x300;

            processor.execute_opcode(0xF655, &PRESSED_KEYCODES);

            assert_eq!(processor.memory[0x300..=0x30F], [0, 1, 2, 3, 4, 5, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
            assert_eq!(processor.pc.get_current(), ADDRESS_NEXT);
        }

        test "Fx65 - read into registers V0 through Vx from memory starting at location I" {
            let mut processor = Processor::new();
            processor.I = 0x300;
            processor.memory[0x300] = 0xA;
            processor.memory[0x301] = 0xB;
            processor.memory[0x302] = 0xC;

            processor.execute_opcode(0xF165, &PRESSED_KEYCODES);

            assert_eq!(processor.V, [0xA, 0xB, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
            assert_eq!(processor.pc.get_current(), ADDRESS_NEXT);
        }
    }
}
