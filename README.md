# circle-eight
Chip8 emulator written in Rust.

To use, you will need to download one or more Chip8 ROM files. 

From the project root, you can build an executable with

`cargo build`

or build and run in one go with

`cargo run path/to/rom.ch8`

Control the games using the mapping below

```
Keyboard         Chip8 Contoller
1 2 3 4          1 2 3 C
Q W E R    ->    4 5 6 D
A S D F          7 8 9 E
Z X C V          A 0 B F
```

There is no sound, though the sound timer does work properly.