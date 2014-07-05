// Game of Life implementation in Rust by Derecho.

extern crate rslife;

use std::io;

fn main() {
    let default_interval = 0.2;
    let default_width = 32;
    let mut reader = io::stdin();

    print!("Interval [{}]: ", default_interval);
    let input = reader.read_line().ok().expect("Failed to read interval");
    let interval = from_str::<f32>(input.as_slice().trim()).unwrap_or(default_interval);

    print!("Filename (empty is a random grid) []: ");
    let input = reader.read_line().ok().expect("Failed to read filename");
    let filename = input.as_slice().trim();

    let mut width = 0;
    let mut height = 0;
    if filename == "" {
        print!("Width [{}]: ", default_width);
        let input = reader.read_line().ok().expect("Failed to read width");
        width = from_str::<uint>(input.as_slice().trim()).unwrap_or(default_width);

        print!("Height [{}]: ", width);
        let input = reader.read_line().ok().expect("Failed to read width");
        height = from_str::<uint>(input.as_slice().trim()).unwrap_or(width);
    }

    let mut game = match filename {
        "" => rslife::Game::random_game(width, height),
        _  => rslife::Game::file_game(filename).ok().expect("Failed to read file")
    };
    game.run_ansi(interval);
}
