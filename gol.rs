// Game of Life implementation in Rust by Derecho.

use std::io::timer;

struct Cell {
    alive: bool
}

fn draw_grid() {
    // TODO
}

fn run(interval: f32) {
    let mut generation = 0;
    loop {
        print!("\x1B[2J");  // Clear screen
        println!("Running Game of Life with {} fps", 1.0/interval);
        println!("Generation: {}", generation);

        draw_grid();
        
        generation += 1;
        timer::sleep((interval * 1000.0) as u64);
    }
}

fn main() {
    // Get these from input
    let interval = 0.2;

    run(interval);
}
