use minifb::Key;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// represents the current state of all keys
pub struct Keyboard {
    pub keys: Arc<Mutex<HashMap<u8, bool>>>,
}

impl Keyboard {
    // CHIP-8 to PC keyboard mapping
    pub const KEYMAP: [Key; 16] = [
        Key::X,    // 0
        Key::Key1, // 1
        Key::Key2, // 2
        Key::Key3, // 3
        Key::Q,    // 4
        Key::W,    // 5
        Key::E,    // 6
        Key::A,    // 7
        Key::S,    // 8
        Key::D,    // 9
        Key::Z,    // A
        Key::C,    // B
        Key::Key4, // C
        Key::R,    // D
        Key::F,    // E
        Key::V,    // F
    ];

    pub fn new() -> Self {
        let mut keys = HashMap::new();
        // initialize all keys (0-F) as not pressed
        for key in 0..=0xF {
            keys.insert(key, false);
        }

        Keyboard {
            keys: Arc::new(Mutex::new(keys)),
        }
    }

    // set key state (pressed/released)
    pub fn set_key(&self, key: u8, pressed: bool) {
        if let Ok(mut keys) = self.keys.lock() {
            keys.insert(key, pressed);
        }
    }

    // check if a specific key is pressed
    pub fn is_key_pressed(&self, key: u8) -> bool {
        if let Ok(keys) = self.keys.lock() {
            *keys.get(&key).unwrap_or(&false)
        } else {
            false
        }
    }

    // wait for any key press and return its value
    pub fn wait_for_key_press(&self) -> Option<u8> {
        if let Ok(keys) = self.keys.lock() {
            for (key, pressed) in keys.iter() {
                if *pressed {
                    return Some(*key);
                }
            }
        }
        None
    }
}
