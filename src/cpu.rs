use std::sync::Arc;

use crate::{display::Display, keyboard::Keyboard};

pub struct CPU {
    pub registers: [u8; 16],    // 16 general-purpose 8-bit registers
    pub i_register: u16,        // 16-bit I register
    pub delay_timer: u8,        // delay timer register
    pub sound_timer: u8,        // sound timer register
    pub program_counter: usize, // program counter (aka location in memory)
    pub heap: [u8; 4096],       // 4KB heap
    pub stack: [u16; 16],       // 16-entry stack
    pub stack_pointer: usize,   // stack pointer
    pub keyboard: Arc<Keyboard>,
    pub display: Display,
}

impl CPU {
    pub fn new(keyboard: Arc<Keyboard>) -> Self {
        let mut cpu = CPU {
            registers: [0; 16],
            i_register: 0,
            delay_timer: 0,
            sound_timer: 0,
            heap: [0; 4096],
            program_counter: 0x200,
            stack: [0; 16],
            stack_pointer: 0,
            keyboard: keyboard.clone(),
            display: Display::new(keyboard),
        };

        // Load built-in hex sprites into interpreter memory area (0x000-0x1FF)
        let hex_sprites: [u8; 80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];

        // Load sprites into interpreter memory area (0x000-0x1FF)
        for (i, &byte) in hex_sprites.iter().enumerate() {
            cpu.heap[i] = byte;
        }

        cpu
    }

    pub fn tick(&mut self) {
        let op_byte1 = self.heap[self.program_counter] as u16;
        let op_byte2 = self.heap[self.program_counter + 1] as u16;
        let opcode: u16 = op_byte1 << 8 | op_byte2;

        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;
        let n = (opcode & 0x000F) as u8;
        let kk = (opcode & 0x00FF) as u8;
        let op_minor = (opcode & 0x000F) as u8;
        let addr = opcode & 0x0FFF;

        // println!("opcode: {:04x}", opcode);
        // println!("heap: {:?}", self.heap);

        self.program_counter += 2;

        // println!("program_counter: {}", self.program_counter);

        match opcode {
            0x0000 => return,                    // Shut down the entire process
            0x00E0 => self.cls(),                // CLS - Clear the Display
            0x00EE => self.ret(),                // Return from a subroutine
            0x0000..=0x0FFF => self.sys(addr),   // SYS addr
            0x1000..=0x1FFF => self.jmp(addr),   // Jump to location nnn
            0x2000..=0x2FFF => self.call(addr),  // Call subroutine at nnn
            0x3000..=0x3FFF => self.se(x, kk),   // Skip next instruction if Vx == kk
            0x4000..=0x4FFF => self.sne(x, kk),  // Skip next instruction if Vx != kk
            0x5000..=0x5FF0 => self.se_xy(x, y), // Skip next instruction if Vx == Vy
            0x6000..=0x6FFF => self.ld(x, kk),   // LD Vx, byte
            0x7000..=0x7FFF => self.add(x, kk),  // ADD Vx, byte
            0x8000..=0x8FFF => match op_minor {
                0 => self.ld(x, self.registers[y as usize]), // LD Vx, Vy
                1 => self.or_xy(x, y),                       // OR Vx, Vy
                2 => self.and_xy(x, y),                      // AND Vx, Vy
                3 => self.xor_xy(x, y),                      // XOR Vx, Vy
                4 => self.add_xy(x, y),                      // ADD Vx, Vy
                5 => self.sub_xy(x, y),                      // SUB Vx, Vy
                6 => self.shr_xy(x),                         // SHR Vx
                7 => self.subn_xy(x, y),                     // SUBN Vx, Vy
                0xE => self.shl_xy(x),                       // SHL Vx
                _ => panic!("invalid opcode: {:04x}", opcode),
            },
            0x9000..=0x9FF0 => self.sne(x, y), // Skip next instruction if Vx != Vy
            0xA000..=0xAFFF => self.ld_i(addr), // LD I, addr
            0xB000..=0xBFFF => self.jmp(addr + self.registers[0] as u16), // JP V0, addr
            0xC000..=0xCFFF => self.rnd(x, kk), // RND Vx, byte
            0xD000..=0xDFFF => self.drw(x, y, n), // DRW Vx, Vy, nibble
            op if (op & 0xF0FF) == 0xE09E => self.skp(x), // SKP Vx
            op if (op & 0xF0FF) == 0xE0A1 => self.sknp(x), // SKNP Vx
            op if (op & 0xF0FF) == 0xF007 => self.ld_vx(x), // LD Vx, DT
            op if (op & 0xF0FF) == 0xF00A => self.ld_k(x), // LD Vx, K
            op if (op & 0xF0FF) == 0xF015 => self.ld_dt(x), // LD DT, Vx
            op if (op & 0xF0FF) == 0xF018 => self.ld_st(x), // LD ST, Vx
            op if (op & 0xF0FF) == 0xF01E => self.add_i(x), // ADD I, Vx
            op if (op & 0xF0FF) == 0xF029 => self.ld_f(x), // LD F, Vx
            op if (op & 0xF0FF) == 0xF033 => self.ld_b(x), // LD B, Vx
            op if (op & 0xF0FF) == 0xF055 => self.ld_i_vx(x), // LD [I], Vx
            op if (op & 0xF0FF) == 0xF065 => self.ld_vx_i(x), // LD Vx, [I]
            _ => panic!("invalid opcode: {:04x}", opcode),
        }

        // Update timers
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    /// (0nnn) SYS addr
    /// This instruction is only used on the old computers on which Chip-8 was originally implemented. It is ignored by modern interpreters.
    fn sys(&mut self, _addr: u16) {
        // this instruction is ignored by modern interpreters
    }

    // 0x00E0: Clear the display
    fn cls(&mut self) {
        self.display.clear();
    }

    /// (6xkk) LD sets the value `kk` into register `vx`
    fn ld(&mut self, vx: u8, kk: u8) {
        self.registers[vx as usize] = kk;
    }

    /// (Axnn) LD I, addr
    fn ld_i(&mut self, addr: u16) {
        self.i_register = addr;
    }

    /// (7xkk) Add sets the value `kk` into register `vx`
    fn add(&mut self, vx: u8, kk: u8) {
        self.registers[vx as usize] += kk;
    }

    /// (3xkk) Skip if equal
    fn se(&mut self, vx: u8, kk: u8) {
        if self.registers[vx as usize] == kk {
            self.program_counter += 2;
        }
    }

    /// (4xkk) Skip if not equal
    fn sne(&mut self, vx: u8, kk: u8) {
        if self.registers[vx as usize] != kk {
            self.program_counter += 2;
        }
    }

    /// (1nnn) JUMP to `addr`
    fn jmp(&mut self, addr: u16) {
        self.program_counter = addr as usize;
    }

    /// (2nnn) CALL sub-routine at `addr`
    fn call(&mut self, addr: u16) {
        let sp = self.stack_pointer;
        let stack = &mut self.stack;

        if sp >= stack.len() {
            panic!("Stack overflow!")
        }

        stack[sp] = self.program_counter as u16;
        self.stack_pointer += 1;
        self.program_counter = addr as usize;
    }

    /// RET return from the current sub-routine
    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack underflow");
        }

        self.stack_pointer -= 1;
        self.program_counter = self.stack[self.stack_pointer] as usize;
    }

    /// (8xy4) ADD Vx, Vy
    /// set Vx = Vx + Vy, set VF = carry
    fn add_xy(&mut self, x: u8, y: u8) {
        let x_val = self.registers[x as usize] as u16;
        let y_val = self.registers[y as usize] as u16;
        let sum = x_val + y_val;
        if sum > 255 {
            // set the carry flag
            self.registers[0xF] = 1;
        } else {
            // clear the carry flag
            self.registers[0xF] = 0;
        }
        self.registers[x as usize] = sum as u8; // store only the lower 8 bits
    }

    /// (8xy2) AND Vx, Vy
    /// set Vx = Vx AND Vy
    fn and_xy(&mut self, x: u8, y: u8) {
        let x_ = self.registers[x as usize];
        let y_ = self.registers[y as usize];

        self.registers[x as usize] = x_ & y_;
    }

    /// (8xy1) OR Vx, Vy
    /// set Vx = Vx OR Vy
    fn or_xy(&mut self, x: u8, y: u8) {
        let x_ = self.registers[x as usize];
        let y_ = self.registers[y as usize];

        self.registers[x as usize] = x_ | y_;
    }

    /// (8xy3) XOR Vx, Vy
    /// set Vx = Vx XOR Vy
    fn xor_xy(&mut self, x: u8, y: u8) {
        let x_ = self.registers[x as usize];
        let y_ = self.registers[y as usize];

        self.registers[x as usize] = x_ ^ y_;
    }

    /// (8xy5) SUB Vx, Vy
    /// set Vx = Vx - Vy, set VF = NOT borrow
    fn sub_xy(&mut self, x: u8, y: u8) {
        let x_val = self.registers[x as usize];
        let y_val = self.registers[y as usize];
        if x_val > y_val {
            // set the borrow flag
            self.registers[0xF] = 1;
        } else {
            // clear the borrow flag
            self.registers[0xF] = 0;
        }
        self.registers[x as usize] = x_val - y_val;
    }

    /// (8xy6) SHR Vx {, Vy}
    /// set Vx = Vx SHR 1
    /// if the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0
    /// then Vx is divided by 2
    fn shr_xy(&mut self, x: u8) {
        // set VF to 1 if the least significant bit is 1
        if self.registers[x as usize] & 0x1 == 1 {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
        // shift right by 1
        self.registers[x as usize] >>= 1;
    }

    /// (8xy7) SUBN Vx, Vy
    /// set Vx = Vy - Vx, set VF = NOT borrow
    fn subn_xy(&mut self, x: u8, y: u8) {
        let x_val = self.registers[x as usize];
        let y_val = self.registers[y as usize];
        if y_val > x_val {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
        self.registers[x as usize] = y_val - x_val;
    }

    /// (8xyE) SHL Vx {, Vy}
    /// set Vx = Vx SHL 1
    /// if the most-significant bit of Vx is 1, then VF is set to 1, otherwise 0
    /// then Vx is multiplied by 2
    fn shl_xy(&mut self, x: u8) {
        // set VF to 1 if the most significant bit is 1
        if self.registers[x as usize] & 0x80 == 0x80 {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
        // shift left by 1
        self.registers[x as usize] <<= 1;
    }

    /// (Cxkk) RND Vx, byte
    /// set Vx = random byte AND kk
    fn rnd(&mut self, x: u8, kk: u8) {
        self.registers[x as usize] = rand::random::<u8>() & kk;
    }

    /// (Dxyn) DRW Vx, Vy, nibble
    /// display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision
    fn drw(&mut self, x: u8, y: u8, n: u8) {
        let x_coord = self.registers[x as usize];
        let y_coord = self.registers[y as usize];
        let sprite = &self.heap[self.i_register as usize..(self.i_register + n as u16) as usize];

        let collision = self.display.draw(x_coord, y_coord, sprite);
        self.registers[0xF] = if collision { 1 } else { 0 };
    }

    /// (Ex9E) SKP Vx
    /// skip next instruction if key with the value of Vx is pressed
    fn skp(&mut self, x: u8) {
        let key = self.registers[x as usize];
        if self.keyboard.is_key_pressed(key) {
            self.program_counter += 2;
        }
    }

    /// (ExA1) SKNP Vx
    /// skip next instruction if key with the value of Vx is not pressed
    fn sknp(&mut self, x: u8) {
        let key = self.registers[x as usize];
        if !self.keyboard.is_key_pressed(key) {
            self.program_counter += 2;
        }
    }

    /// (Fx07) LD Vx, DT
    /// set Vx = delay timer value
    fn ld_vx(&mut self, vx: u8) {
        self.registers[vx as usize] = self.delay_timer;
    }

    /// (Fx0A) LD Vx, K
    /// wait for a key press, store the value of the key in Vx
    fn ld_k(&mut self, vx: u8) {
        if let Some(key) = self.keyboard.wait_for_key_press() {
            self.registers[vx as usize] = key;
        } else {
            // if no key is pressed, decrease PC to repeat this instruction
            self.program_counter -= 2;
        }
    }

    /// (Fx15) LD DT, Vx
    /// Delay timer is set to the value of Vx
    fn ld_dt(&mut self, vx: u8) {
        self.delay_timer = self.registers[vx as usize];
    }

    /// (Fx18) LD ST, Vx
    /// Sound timer is set to the value of Vx
    fn ld_st(&mut self, vx: u8) {
        self.sound_timer = self.registers[vx as usize];
    }

    /// (Fx1E) ADD I, Vx
    /// I is added to Vx
    fn add_i(&mut self, vx: u8) {
        self.i_register += self.registers[vx as usize] as u16;
    }

    /// (Fx29) LD F, Vx
    /// I is set to the location of the sprite for digit Vx
    fn ld_f(&mut self, vx: u8) {
        let digit = self.registers[vx as usize] & 0xF; // Ensure we only use the lowest 4 bits
        self.i_register = (digit * 5) as u16; // Each sprite is 5 bytes tall
    }

    /// (Fx33) LD B, Vx
    /// The BCD representation of Vx is stored in memory at location in I, I+1, and I+2.
    /// The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2.
    fn ld_b(&mut self, vx: u8) {
        let vx_value = self.registers[vx as usize];
        let hundreds = vx_value / 100;
        let tens = (vx_value % 100) / 10;
        let ones = vx_value % 10;

        self.heap[self.i_register as usize] = hundreds;
        self.heap[self.i_register as usize + 1] = tens;
        self.heap[self.i_register as usize + 2] = ones;
    }

    /// (Fx55) LD [I], Vx
    /// The interpreter copies the values of registers V0 through Vx into memory, starting at location I.
    fn ld_i_vx(&mut self, vx: u8) {
        for i in 0..=vx {
            self.heap[self.i_register as usize + i as usize] = self.registers[i as usize];
        }
    }

    /// (Fx65) LD Vx, [I]
    /// The interpreter reads values from memory starting at location I into registers V0 through Vx.
    fn ld_vx_i(&mut self, vx: u8) {
        for i in 0..=vx {
            self.registers[i as usize] = self.heap[self.i_register as usize + i as usize];
        }
    }

    /// (5xy0) Skip if registers equal
    fn se_xy(&mut self, x: u8, y: u8) {
        if self.registers[x as usize] == self.registers[y as usize] {
            self.program_counter += 2;
        }
    }
}
