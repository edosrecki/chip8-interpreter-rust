#![allow(non_snake_case)]

use std::collections::BTreeSet;
use super::font::FONT;
use super::canvas::Canvas;
use super::pc::ProgramCounter;
use crate::chip8::interface::{Input, Output};

const RAM_BYTES: usize = 4096;
const V_SIZE: usize = 16;
const STACK_SIZE: usize = 16;
const PROGRAM_ADDRESS_START: usize = 0x200;

pub struct Processor {
    /// 4096B of RAM. The first 512B are reserved for the interpreter (0x000 to 0x1FF).
    /// Programs start at 0x200.
    memory: [u8; RAM_BYTES],

    /// 16 general purpose 8b registers.
    V: [u8; V_SIZE],

    /// 12b register to store memory addresses. 12 least significant bits are used.
    I: usize,

    /// Program counter: stores the address of the currently executing instruction.
    /// 12 least significant bits are used.
    pc: ProgramCounter,

    /// 8b stack pointer. Points to the next free spot on the stack.
    sp: usize,

    /// 16 items, each 16b long. Stores addresses that the interpreter must return to
    /// after finishing a subroutine.
    stack: [usize; 16],

    /// Timer subtracts 1 from its value when it is greater than 0, at a rate of 60Hz.
    delay_timer: u8,
    
    /// Timer subtracts 1 from its value when it is greater than 0, at a rate of 60Hz.
    /// While it is greater than 0, the buzzer produces a sound of a constant arbitrary
    /// frequency.
    sound_timer: u8,

    // While waiting for keypad press, CPU is not processig instructions.
    waiting_for_keypad: bool,
    // Store keycode into this register after waiting for keypad press.
    keycode_register: usize,

    // Read and write the state of each pixel (on/off)
    canvas: Canvas,
}

impl Processor {
    pub fn new() -> Self {
        let mut cpu = Processor {
            memory: [0; RAM_BYTES],
            V: [0; V_SIZE],
            I: 0,
            pc: ProgramCounter::new(PROGRAM_ADDRESS_START),
            sp: 0,
            stack: [0; STACK_SIZE],
            delay_timer: 0,
            sound_timer: 0,
            waiting_for_keypad: false,
            keycode_register: 0,
            canvas: Canvas::new(),
        };

        cpu.load(0x0, &FONT);
        cpu
    }

    pub fn load_program(&mut self, data: &[u8]) {
        self.load(PROGRAM_ADDRESS_START, data);
    }

    pub fn execute_instruction(&mut self, input: Input) -> Output {
        let pressed_keycodes = input.pressed_keycodes;
        self.canvas.reset_pixels_changed();

        if self.waiting_for_keypad {
            self.wait_for_keypad(pressed_keycodes);
        } else {
            let opcode = self.get_opcode();
            self.execute_opcode(opcode, pressed_keycodes);
        }

        Output {
            is_sound_on: self.sound_timer > 0,
            pixels: self.canvas.pixels,
            have_pixels_changed: self.canvas.have_pixels_changed,
        }
    }

    pub fn update_timers(&mut self) {
        if !self.waiting_for_keypad {
            self.delay_timer = self.delay_timer.saturating_sub(1);
            self.sound_timer = self.sound_timer.saturating_sub(1);
        }
    }
    
    fn load(&mut self, address: usize, data: &[u8]) {
        let addresses = address..;
        let bytes = data.iter();

        for (address, &byte) in addresses.zip(bytes) {
            if address >= RAM_BYTES {
                break;
            }

            self.memory[address] = byte;
        }
    }

    fn wait_for_keypad(&mut self, pressed_keycodes: &BTreeSet<u8>) {
        if let Some(&keycode) = pressed_keycodes.first() {
            self.V[self.keycode_register] = keycode;
            self.waiting_for_keypad = false;
        }
    }

    fn get_opcode(&self) -> u16 {
        let pc: usize = self.pc.get_current();

        let msb = self.memory[pc] as u16;
        let lsb = self.memory[pc + 1] as u16;

        (msb << 8) | lsb
    }

    fn execute_opcode(&mut self, opcode: u16, pressed_keycodes: &BTreeSet<u8>) {
        let nibbles = (
            (opcode & 0xF000) >> 12,
            (opcode & 0x0F00) >> 8,
            (opcode & 0x00F0) >> 4,
            (opcode & 0x000F)
        );

        let nnn = (opcode & 0x0FFF) as usize;
        let n   = nibbles.3 as usize;
        let x   = nibbles.1 as usize;
        let y   = nibbles.2 as usize;
        let kk  = (opcode & 0x00FF) as u8;

        match nibbles {
            (0x0, 0x0, 0xE, 0x0) => self.op_00E0(),
            (0x0, 0x0, 0xE, 0xE) => self.op_00EE(),
            (0x1, _  , _  , _  ) => self.op_1nnn(nnn),
            (0x2, _  , _  , _  ) => self.op_2nnn(nnn),
            (0x3, _  , _  , _  ) => self.op_3xkk(x, kk),
            (0x4, _  , _  , _  ) => self.op_4xkk(x, kk),
            (0x5, _  , _  , 0x0) => self.op_5xy0(x, y),
            (0x6, _  , _  , _  ) => self.op_6xkk(x, kk),
            (0x7, _  , _  , _  ) => self.op_7xkk(x, kk),
            (0x8, _  , _  , 0x0) => self.op_8xy0(x, y),
            (0x8, _  , _  , 0x1) => self.op_8xy1(x, y),
            (0x8, _  , _  , 0x2) => self.op_8xy2(x, y),
            (0x8, _  , _  , 0x3) => self.op_8xy3(x, y),
            (0x8, _  , _  , 0x4) => self.op_8xy4(x, y),
            (0x8, _  , _  , 0x5) => self.op_8xy5(x, y),
            (0x8, _  , _  , 0x6) => self.op_8xy6(x),
            (0x8, _  , _  , 0x7) => self.op_8xy7(x, y),
            (0x8, _  , _  , 0xE) => self.op_8xyE(x),
            (0x9, _  , _  , 0x0) => self.op_9xy0(x, y),
            (0xA, _  , _  , _  ) => self.op_Annn(nnn),
            (0xB, _  , _  , _  ) => self.op_Bnnn(nnn),
            (0xC, _  , _  , _  ) => self.op_Cxkk(x, kk),
            (0xD, _  , _  , _  ) => self.op_Dxyn(x, y, n),
            (0xE, _  , 0x9, 0xE) => self.op_Ex9E(x, pressed_keycodes),
            (0xE, _  , 0xA, 0x1) => self.op_ExA1(x, pressed_keycodes),
            (0xF, _  , 0x0, 0x7) => self.op_Fx07(x),
            (0xF, _  , 0x0, 0xA) => self.op_Fx0A(x),
            (0xF, _  , 0x1, 0x5) => self.op_Fx15(x),
            (0xF, _  , 0x1, 0x8) => self.op_Fx18(x),
            (0xF, _  , 0x1, 0xE) => self.op_Fx1E(x),
            (0xF, _  , 0x2, 0x9) => self.op_Fx29(x),
            (0xF, _  , 0x3, 0x3) => self.op_Fx33(x),
            (0xF, _  , 0x5, 0x5) => self.op_Fx55(x),
            (0xF, _  , 0x6, 0x5) => self.op_Fx65(x),
            _                    => {},
        }
    }

    // Clean screen.
    fn op_00E0(&mut self) {
        self.canvas.clean();

        self.pc.goto_next();
    }

    // Return from a subroutine.
    fn op_00EE(&mut self) {
        self.sp -= 1;

        self.pc.jump(self.stack[self.sp]);
    }

    // Jump to address at `nnn`.
    fn op_1nnn(&mut self, nnn: usize) {
        self.pc.jump(nnn);
    }

    // Call subroutine at `nnn`.
    fn op_2nnn(&mut self, nnn: usize) {
        self.stack[self.sp] = self.pc.get_next();
        self.sp += 1;

        self.pc.jump(nnn);
    }

    // Skip next instruction if Vx == kk.
    fn op_3xkk(&mut self, x: usize, kk: u8) {
        if self.V[x] == kk {
            self.pc.skip_next();
        } else {
            self.pc.goto_next();
        }
    }

    // Skip next instruction if Vx != kk.
    fn op_4xkk(&mut self, x: usize, kk: u8) {
        if self.V[x] != kk {
            self.pc.skip_next();
        } else {
            self.pc.goto_next();
        }
    }

    // Skip next instruction if Vx == Vy.
    fn op_5xy0(&mut self, x: usize, y: usize) {
        if self.V[x] == self.V[y] {
            self.pc.skip_next();
        } else {
            self.pc.goto_next();
        }
    }

    // Set Vx = kk.
    fn op_6xkk(&mut self, x: usize, kk: u8) {
        self.V[x] = kk;

        self.pc.goto_next();
    }

    // Set Vx = Vx + kk.
    fn op_7xkk(&mut self, x: usize, kk: u8) {
        let a = self.V[x] as u16;
        let b = kk as u16;

        self.V[x] = (a + b) as u8;

        self.pc.goto_next();
    }

    // Set Vx = Vy.
    fn op_8xy0(&mut self, x: usize, y: usize) {
        self.V[x] = self.V[y];

        self.pc.goto_next();
    }

    // Set Vx = Vx | Vy.
    fn op_8xy1(&mut self, x: usize, y: usize) {
        self.V[x] |= self.V[y];

        self.pc.goto_next();
    }

    // Set Vx = Vx & Vy.
    fn op_8xy2(&mut self, x: usize, y: usize) {
        self.V[x] &= self.V[y];

        self.pc.goto_next();
    }

    // Set Vx = Vx ^ Vy.
    fn op_8xy3(&mut self, x: usize, y: usize) {
        self.V[x] ^= self.V[y];

        self.pc.goto_next();
    }

    // Set Vx = Vx + Vy. Set VF = carry.
    fn op_8xy4(&mut self, x: usize, y: usize) {
        let a = self.V[x] as u16;
        let b = self.V[y] as u16;
        
        let res = a + b;
        self.V[x] = res as u8;
        self.V[0xF] = if res > 0xFF { 1 } else { 0 };

        self.pc.goto_next();
    }

    // Set Vx = Vx - Vy. Set VF = 1 if Vx > Vy, otherwise set to 0.
    fn op_8xy5(&mut self, x: usize, y: usize) {
        self.V[0xF] = if self.V[x] > self.V[y] { 1 } else { 0 };
        self.V[x] = self.V[x].wrapping_sub(self.V[y]);

        self.pc.goto_next();
    }
    
    // Set VF to 1 if least-significant bit of Vx == 1, otherwise set to 0.
    // Divide Vx by 2.
    fn op_8xy6(&mut self, x: usize) {
        self.V[0xF] = self.V[x] & 0b1;
        self.V[x] >>= 1;

        self.pc.goto_next();
    }

    // Set Vx = Vy - Vx. Set VF = 1 if Vy > Vx, otherwise set to 0. 
    fn op_8xy7(&mut self, x: usize, y: usize) {
        self.V[0xF] = if self.V[y] > self.V[x] { 1 } else { 0 };
        self.V[x] = self.V[y].wrapping_sub(self.V[x]);
        
        self.pc.goto_next();
    }

    // Set VF to 1 if most-significant bit of Vx == 1, otherwise set to 0.
    // Multiply V[x] by 2.
    fn op_8xyE(&mut self, x: usize) {
        self.V[0xF] = (self.V[x] & 0b10000000) >> 7;
        self.V[x] <<= 1;

        self.pc.goto_next();
    }

    // Skip next instruction if Vx != Vy.
    fn op_9xy0(&mut self, x: usize, y: usize) {
        if self.V[x] != self.V[y] {
            self.pc.skip_next();
        } else {
            self.pc.goto_next();
        }
    }

    // Set register I = nnn.
    fn op_Annn(&mut self, nnn: usize) {
        self.I = nnn;

        self.pc.goto_next();
    }

    // Jump to address nnn + V0.
    fn op_Bnnn(&mut self, nnn: usize) {
        self.pc.jump((self.V[0x0] as usize) + nnn);
    }

    // Set Vx = random byte & kk.
    fn op_Cxkk(&mut self, x: usize, kk: u8) {
        let rand_byte = rand::random::<u8>();
        self.V[x] = rand_byte & kk;

        self.pc.goto_next();
    }

    // Read n bytes from memory starting at address written in register I.
    // Draw those bytes starting at coordinate (Vx, Vy), treating each byte
    // as a row of pixels. If part of the sprite is outside of display
    // boundaries, it wraps around to the opposite side of the display.
    fn op_Dxyn(&mut self, x: usize, y: usize, n: usize) {
        let x_left = self.V[x] as usize;
        let y_top = self.V[y] as usize;
        let sprite = &self.memory[self.I..self.I + n];

        let any_pixel_erased = self.canvas.draw_sprite(x_left, y_top, sprite);
        self.V[0xF] = if any_pixel_erased { 1 } else { 0 };

        self.pc.goto_next();
    }

    // Skip next instruction if key with the value of Vx is pressed.
    fn op_Ex9E(&mut self, x: usize, pressed_keycodes: &BTreeSet<u8>) {
        if pressed_keycodes.contains(&self.V[x]) {
            self.pc.skip_next();
        } else {
            self.pc.goto_next();
        }
    }

    // Skip next instruction if key with the value of Vx is not pressed.
    fn op_ExA1(&mut self, x: usize, pressed_keycodes: &BTreeSet<u8>) {
        if pressed_keycodes.contains(&self.V[x]) {
            self.pc.goto_next();
        } else {
            self.pc.skip_next();
        }
    }

    // Set Vx = delay_timer.
    fn op_Fx07(&mut self, x: usize) {
        self.V[x] = self.delay_timer;

        self.pc.goto_next();
    }

    // Wait for a key press. Store keycode into register Vx.
    fn op_Fx0A(&mut self, x: usize) {
        self.waiting_for_keypad = true;
        self.keycode_register = x;

        self.pc.goto_next();
    }

    // Set delay_timer = Vx.
    fn op_Fx15(&mut self, x: usize) {
        self.delay_timer = self.V[x];

        self.pc.goto_next();
    }

    // Set sound_timer = Vx.
    fn op_Fx18(&mut self, x: usize) {
        self.sound_timer = self.V[x];

        self.pc.goto_next();
    }

    // Set I = I + Vx. 
    fn op_Fx1E(&mut self, x: usize) {
        self.I += self.V[x] as usize;

        // Most implementations do not set VF in case of an overflow
        // https://github.com/Chromatophore/HP48-Superchip/issues/2
        // self.V[0xF] = if self.I > 0xFFF { 1 } else { 0 };

        self.pc.goto_next();
    }

    // Set I to the location of sprite for digit Vx.
    fn op_Fx29(&mut self, x: usize) {
        // digit sprites start at memory 0x000 and each is 5B long
        self.I = (self.V[x] as usize) * 5;

        self.pc.goto_next();
    }

    // Take digits of V[x] and set:
    // * memory[I] = hundreds digit,
    // * memory[I + 1] = tens digit,
    // * memory[I + 2] = ones digit.
    fn op_Fx33(&mut self, x: usize) {
        self.memory[self.I]     = self.V[x] / 100;       // hundreds digit
        self.memory[self.I + 1] = (self.V[x] / 10) % 10; // tens digit
        self.memory[self.I + 2] = self.V[x] % 10;        // ones digit

        self.pc.goto_next();
    }

    // Store registers V0 through Vx in memory starting at address I.
    fn op_Fx55(&mut self, x: usize) {
        for offset in 0..=x {
            self.memory[self.I + offset] = self.V[offset];
        }

        self.pc.goto_next();
    }

    // Read into registers V0 through Vx from memory starting at location I.
    fn op_Fx65(&mut self, x: usize) {
        for offset in 0..=x {
            self.V[offset] = self.memory[self.I + offset];
        }

        self.pc.goto_next();
    }
}

#[cfg(test)]
#[path = "./processor_test.rs"]
mod processor_test;
