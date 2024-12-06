# CHIP-8 Emulator

This is a simple CHIP-8 emulator written in Rust. I built it following the spec at [Cowgod's CHIP-8 Technical Reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM).

Also huge shoutout to the [Rust In Action](https://www.manning.com/books/rust-in-action) book for the baseline knowledge of Rust and CPUs to finish this project.

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
