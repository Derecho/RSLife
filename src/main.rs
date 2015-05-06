// Game of Life implementation in Rust by Derecho.

extern crate rslife;

use std::io;
use std::io::Write;

fn main() {
    let default_interval = 0.2;
    let default_width = 32;
    let mut reader = io::stdin();
    let mut input = String::new();

    print!("Interval [{}]: ", default_interval);
    io::stdout().flush().ok();
    reader.read_line(&mut input).ok().expect("Failed to read interval");
    let interval: f32 = input.trim().parse().unwrap_or(default_interval);

    print!("Filename (empty is a random grid) []: ");
    io::stdout().flush().ok();
    input.clear();
    reader.read_line(&mut input).ok().expect("Failed to read filename");
    let filename = input.trim().to_owned();
    let filename_str = &*filename;

    let mut width = 0;
    let mut height = 0;
    if filename_str == "" {
        print!("Width [{}]: ", default_width);
        io::stdout().flush().ok();
        input.clear();
        reader.read_line(&mut input).ok().expect("Failed to read width");
        width = input.trim().parse().unwrap_or(default_width);

        print!("Height [{}]: ", width);
        io::stdout().flush().ok();
        input.clear();
        reader.read_line(&mut input).ok().expect("Failed to read width");
        height = input.trim().parse().unwrap_or(width);
    }

    let mut game = match filename_str {
        "" => rslife::Game::random_game(width, height),
        _  => rslife::Game::file_game(filename_str).ok().expect("Failed to read file")
    };

    print!("Draw method: (braille/block/ansi) [braille]: ");
    io::stdout().flush().ok();
    input.clear();
    reader.read_line(&mut input).ok().expect("Failed to read draw method");
    let draw_method = input.trim();

    match draw_method {
        "ansi"         => game.run_ansi(interval),
        "block"        => game.run_block(interval),
        "" | "braille" => game.run_braille(interval),
        _              => panic!("Invalid draw method"),
    };
}
