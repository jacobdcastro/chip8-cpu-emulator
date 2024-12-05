use crate::keyboard::Keyboard;
use minifb::{Key, Window, WindowOptions};
use std::{convert::TryInto, sync::Arc};

pub struct Display {
    window: Window,
    buffer: Vec<u32>,
    width: usize,
    height: usize,
    keyboard: Arc<Keyboard>,
}

pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;

impl Display {
    pub fn new(keyboard: Arc<Keyboard>) -> Self {
        let window = Window::new(
            "CHIP-8 Emulator",
            DISPLAY_WIDTH,
            DISPLAY_HEIGHT,
            WindowOptions {
                scale: minifb::Scale::X16,
                ..WindowOptions::default()
            },
        )
        .expect("failed to create window");

        let display = Display {
            window,
            buffer: vec![0; DISPLAY_WIDTH * DISPLAY_HEIGHT],
            width: DISPLAY_WIDTH,
            height: DISPLAY_HEIGHT,
            keyboard,
        };

        // // Test pattern - draw a simple rectangle
        // for i in 10..20 {
        //     for j in 10..20 {
        //         display.buffer[i * DISPLAY_WIDTH + j] = 0xFFFFFF;
        //     }
        // }

        display
    }

    // pub fn get_buffer(&self) -> &Vec<u32> {
    //     &self.buffer
    // }

    // clear the display
    pub fn clear(&mut self) {
        self.buffer.fill(0);
    }

    // draw a sprite at position (x, y) with data from memory
    pub fn draw(&mut self, x: u8, y: u8, sprite: &[u8]) -> bool {
        let mut collision = false;

        for (row, sprite_byte) in sprite.iter().enumerate() {
            for bit in 0..8 {
                let px = (x as usize + bit) % self.width;
                let py = (y as usize + row) % self.height;
                let pixel = (sprite_byte >> (7 - bit)) & 1;

                if pixel == 1 {
                    let index = py * self.width + px;
                    if self.buffer[index] == 0xFFFFFF {
                        collision = true;
                    }
                    self.buffer[index] ^= 0xFFFFFF;
                }
            }
        }

        collision
    }

    // update the window with current buffer contents
    // pub fn update(&mut self) -> bool {
    //     if !self.window.is_open() {
    //         return false;
    //     }

    //     self.window
    //         .update_with_buffer(&self.buffer, self.width, self.height)
    //         .unwrap();

    //     true
    // }

    // // get the current state of a key
    // pub fn is_key_pressed(&self, key: Key) -> bool {
    //     self.window.is_key_down(key)
    // }

    // pub fn handle_input(&mut self) -> bool {
    //     if !self.window.is_open() {
    //         return false;
    //     }

    //     // Get all pressed keys
    //     self.window.get_keys().iter().for_each(|key| match key {
    //         Key::Key1 => self.keyboard.set_key(0x1, true),
    //         Key::Key2 => self.keyboard.set_key(0x2, true),
    //         Key::Key3 => self.keyboard.set_key(0x3, true),
    //         Key::Key4 => self.keyboard.set_key(0xC, true),
    //         Key::Q => self.keyboard.set_key(0x4, true),
    //         Key::W => self.keyboard.set_key(0x5, true),
    //         Key::E => self.keyboard.set_key(0x6, true),
    //         Key::R => self.keyboard.set_key(0xD, true),
    //         Key::A => self.keyboard.set_key(0x7, true),
    //         Key::S => self.keyboard.set_key(0x8, true),
    //         Key::D => self.keyboard.set_key(0x9, true),
    //         Key::F => self.keyboard.set_key(0xE, true),
    //         Key::Z => self.keyboard.set_key(0xA, true),
    //         Key::X => self.keyboard.set_key(0x0, true),
    //         Key::C => self.keyboard.set_key(0xB, true),
    //         Key::V => self.keyboard.set_key(0xF, true),
    //         _ => {}
    //     });

    //     // Clear keys that are no longer pressed
    //     let pressed_keys = self.window.get_keys();
    //     [
    //         (Key::Key1, 0x1),
    //         (Key::Key2, 0x2),
    //         (Key::Key3, 0x3),
    //         (Key::Key4, 0xC),
    //         (Key::Q, 0x4),
    //         (Key::W, 0x5),
    //         (Key::E, 0x6),
    //         (Key::R, 0xD),
    //         (Key::A, 0x7),
    //         (Key::S, 0x8),
    //         (Key::D, 0x9),
    //         (Key::F, 0xE),
    //         (Key::Z, 0xA),
    //         (Key::X, 0x0),
    //         (Key::C, 0xB),
    //         (Key::V, 0xF),
    //     ]
    //     .iter()
    //     .for_each(|(key, chip8_key)| {
    //         if !pressed_keys.contains(key) {
    //             self.keyboard.set_key(*chip8_key, false);
    //         }
    //     });

    //     true
    // }

    // // add this method to get the window reference
    // pub fn window(&mut self) -> &mut Window {
    //     &mut self.window
    // }

    // add this method to check if window is open
    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }

    pub fn update(&mut self) -> Result<(), minifb::Error> {
        // update keyboard state
        for (chip8_key, pc_key) in Keyboard::KEYMAP.iter().enumerate() {
            self.keyboard
                .set_key(chip8_key as u8, self.window.is_key_down(*pc_key));
        }

        self.window
            .update_with_buffer(&self.buffer, self.width, self.height)
    }
}
