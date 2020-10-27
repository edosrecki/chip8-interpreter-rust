use lazy_static::lazy_static;
use maplit::hashmap;
use std::collections::{HashMap, HashSet, BTreeSet};
use sdl2::Sdl;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;

type Chip8Keycode = u8;

// CHIP-8 can detect input from a sixteen key keypad, where each key corresponds to a
// hexadecimal digit. Original computers running CHIP-8 provided such a keypad. For
// modern keyboards, we need to map keys to simulate usage of the original keypad.
// 
// # Mapping
// ```
// QWERTY        |    Original
// ------------------------------------
// 1  2  3  4    |    1  2  3  C
// Q  W  E  R    |    4  5  6  D
// A  S  D  F    |    7  8  9  E
// Z  X  C  V    |    A  0  B  F
// ```
lazy_static! {
    static ref QWERTY_TO_CHIP8_KEYCODE: HashMap<Keycode, Chip8Keycode> = hashmap!{
        Keycode::Num1 => 0x1,
        Keycode::Num2 => 0x2,
        Keycode::Num3 => 0x3,
        Keycode::Num4 => 0xC,
    
        Keycode::Q => 0x4,
        Keycode::W => 0x5,
        Keycode::E => 0x6,
        Keycode::R => 0xD,
    
        Keycode::A => 0x7,
        Keycode::S => 0x8,
        Keycode::D => 0x9,
        Keycode::F => 0xE,
    
        Keycode::Z => 0xA,
        Keycode::X => 0x0,
        Keycode::C => 0xB,
        Keycode::V => 0xF,
    };
}

pub enum KeypadState {
    PressedEscape,
    PressedKeycodes(BTreeSet<Chip8Keycode>)
}

pub struct Keypad {
    events: EventPump
}

impl Keypad {
    pub fn new(sdl: &Sdl) -> Result<Self, String> {
        let events = sdl.event_pump()?;
        
        Ok(Keypad { events })
    }

    pub fn state(&mut self) -> KeypadState {
        self.events.pump_events();

        let pressed_keycodes: HashSet<Keycode> = self.events
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        if pressed_keycodes.contains(&Keycode::Escape) {
            KeypadState::PressedEscape
        } else {
            let pressed_chip8_keycodes: BTreeSet<Chip8Keycode> = pressed_keycodes
                .iter()
                .filter_map(|keycode| QWERTY_TO_CHIP8_KEYCODE.get(&keycode))
                .cloned()
                .collect();

            KeypadState::PressedKeycodes(pressed_chip8_keycodes)
        }
    }
}
