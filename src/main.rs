mod cpu;
mod display;
mod keyboard;
mod rom_loader;

extern crate minifb;
extern crate rand;
use cpu::CPU;

use crate::keyboard::Keyboard;
use crate::rom_loader::RomLoader;
use std::path::Path;
use std::sync::Arc;

fn main() {
    let keyboard = Arc::new(Keyboard::new());

    let mut cpu = CPU::new(keyboard);

    // Load ROM file
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <ROM file>", args[0]);
        return;
    }

    let rom_path = &args[1];
    // load ROM file (will handle both binary and text assembly)
    let rom_data = match RomLoader::load(Path::new(rom_path)) {
        Ok(data) => data,
        Err(e) => {
            println!("error loading ROM: {}", e);
            return;
        }
    };

    println!("loaded ROM: {} bytes", rom_data.len());

    // ensure ROM isn't too large for memory
    if rom_data.len() > 0xFFF - 0x200 {
        println!("ROM is too large to fit in memory!");
        return;
    }

    // load ROM data into memory starting at 0x200
    for (i, &byte) in rom_data.iter().enumerate() {
        cpu.heap[0x200 + i] = byte;
    }

    println!("ROM loaded into memory at 0x200");

    // main emulation loop
    while cpu.display.is_open() {
        // run one cpu cycle
        cpu.tick();

        // update display
        if let Err(e) = cpu.display.update() {
            println!("failed to update display: {}", e);
            break;
        }

        // sleep for 1/60th of a second
        // std::thread::sleep(std::time::Duration::from_secs_f32(0.4));
    }
}
