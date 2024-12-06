# CHIP-8 Emulator

This is a simple CHIP-8 emulator written in Rust. I built it following the spec at [Cowgod's CHIP-8 Technical Reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM).

It implements the CHIP-8 instruction set (opcodes), and follows other specs, including:
- 4KB of memory (heap)
- 16 general-purpose 8-bit registers
- 16-entry stack
- 64x32 pixel display
- 16-key hexadecimal keypad

It uses the minifb library for the display and keyboard input handling. The only thing **not** implemented is the sound.

Transparently, it's pretty untested and probably has a lot of bugs, but it mostly works for the programs I've tested it with. This project was meant to be a fun way to learn Rust and CPUs/VMs, so I wasn't very "perfectionist" about this.


## Usage

```
cargo run <path_to_rom>
```

## Keyboard Mapping

```
CHIP-8 Key   Keyboard
---------    ---------
1 2 3 C      1 2 3 4
4 5 6 D      Q W E R
7 8 9 E      A S D F
A 0 B F      Z X C V
```

## Notes

- The `calculator.ch8` program is a simple example of a CHIP-8 program that adds two numbers together, (but the custom `+` and `=` sprites are not rendering for some reason).
- The `coffee.ch8` program is a simple example of a CHIP-8 program that displays the word "COFFEE".

Also huge shoutout to the [Rust In Action](https://www.manning.com/books/rust-in-action) book for the baseline knowledge of Rust and CPUs to finish this project.

## Contributing

If you spot any issues, or want to generally roast my code, feel free to open an issue or PR, or shout me out on [X](https://x.com/jacobdcastro)!

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
