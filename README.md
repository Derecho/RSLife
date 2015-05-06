# RSLife
Game of Life implementation in Rust

This particular piece of code serves as a way for me to lean Rust and is my
first project in this language.

## Compiling
Use the standard approach of issuing `cargo build` to build the application.
Afterwards, run the `rslife` binary in the `target` directory.

## Usage
You will be promted to choose an interval and a file to read a grid from. By
not supplying a filename a random grid will be generated for you at a size you
choose.

Currently, after having exited the program, your cursor will be invisible.
For now, you'll have to issue the following command to get it back:

```
setterm -cursor on
```

This will be fixed in a future commit.
