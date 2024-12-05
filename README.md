# CHIP-8 Emulator

This is a simple CHIP-8 emulator written in Rust. It is built using the minifb library for the display and keyboard.

## Usage

```
cargo run --release <path_to_rom>
```

## Controls

- 1, 2, 3, C: Change color palette
- Arrow keys: Move cursor
- Space: Toggle tracing
- R: Reset
- Escape: Quit

```
CHIP-8 Key   Keyboard
---------    ---------
1 2 3 C      1 2 3 4
4 5 6 D      Q W E R
7 8 9 E      A S D F
A 0 B F      Z X C V
```
