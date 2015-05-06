// Game of Life implementation in Rust by Derecho.

extern crate rslife;

use std::io;
use std::io::Write;
use std::fmt::Display;
use std::str::FromStr;

fn prompt<T: Display + FromStr, U: Into<T>>(question: &str, default: U) -> T {
    let mut input = String::new();
    let default = default.into();

    print!("{} [{}]: ", question, default);
    io::stdout().flush().ok();

    io::stdin().read_line(&mut input).ok().expect("Failed to read stdin");
    input.trim().parse().unwrap_or(default)
}

fn main() {
    let interval = prompt("Interval", 0.2);

    let filename: String = prompt("Filename (empty is a random grid)", "");

    let mut game = if filename == "" {
        let width = prompt("Width", 32);
        let height = prompt("Height", width);
        rslife::Game::random_game(width, height)
    } else {
        rslife::Game::file_game(&*filename).ok().expect("Failed to read file")
    };

    let draw_method: String = prompt("Draw method (braille/block/ansi)", "braille");
    match &*draw_method {
        "ansi"         => game.run_ansi(interval),
        "block"        => game.run_block(interval),
        "" | "braille" => game.run_braille(interval),
        _              => panic!("Invalid draw method"),
    };
}
