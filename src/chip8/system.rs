use fps_clock::FpsClock;

use super::io::display::Display;
use super::io::keypad::{Keypad, KeypadState};
use super::io::sound::Sound;
use super::cpu::Processor;
use super::interface::{Input, Output};

const FREQUENCY: u32 = 60;
const CPU_SPEED_FACTOR: u32 = 9; // 540 Hz

pub struct System {
    clock: FpsClock,
    display: Display,
    keypad: Keypad,
    sound: Sound,
    processor: Processor,
}

impl System {
    pub fn new(display: Display, keypad: Keypad, sound: Sound, processor: Processor) -> Self {
        System {
            clock: FpsClock::new(FREQUENCY),
            display,
            keypad,
            sound,
            processor,
        }
    }

    pub fn run_loop(&mut self) {
        'main: loop {
            for _ in 0..CPU_SPEED_FACTOR {
                let pressed_keycodes = &(match self.keypad.state() {
                    KeypadState::PressedEscape       => break 'main,
                    KeypadState::PressedKeycodes(pk) => pk
                });

                let input = Input { pressed_keycodes };
                let Output { is_sound_on, pixels, have_pixels_changed } = self.processor.execute_instruction(input);

                self.sound.set(is_sound_on);

                if have_pixels_changed {
                    self.display.render(&pixels).expect("Cannot render pixels on display");
                }
            }

            self.processor.update_timers();
            self.clock.tick();
        }
    }
}
