// Game of Life implementation in Rust by Derecho.

extern crate rslife;

use std::io;
use std::io::Write;
use std::fmt::Display;
use std::str::FromStr;

fn prompt<T: Display + FromStr>(question: &str, default: T) -> T {
    let mut input = String::new();

    print!("{} [{}]: ", question, default);
    io::stdout().flush().ok();

    io::stdin().read_line(&mut input).ok().expect("Failed to read stdin");
    let answer: T = input.trim().parse().unwrap_or(default);
    answer
}

fn main() {
    let interval = prompt::<f32>("Interval", 0.2);

    let filename = prompt::<String>("Filename (empty is a random grid)", String::new());

    let mut width = 0;
    let mut height = 0;
    if filename == "" {
        width = prompt::<usize>("Width", 32);
        height = prompt::<usize>("Height", width);
    }

    let mut game = match &*filename {
        "" => rslife::Game::random_game(width, height),
        _  => rslife::Game::file_game(&*filename).ok().expect("Failed to read file")
    };

    let draw_method = prompt::<String>("Draw method (braille/block/ansi)", "braille".to_owned());
    match &*draw_method {
        "ansi"         => game.run_ansi(interval),
        "block"        => game.run_block(interval),
        "" | "braille" => game.run_braille(interval),
        _              => panic!("Invalid draw method"),
    };
}
