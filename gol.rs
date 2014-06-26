// Game of Life implementation in Rust by Derecho.

fn update_screen() {
    // TODO
}

fn run(interval: f32) {
    println!("Running Game of Life with {} fps", 1.0/interval);
    loop {
        // TODO Sleep interval
        update_screen();
    }
}

fn main() {
    // Get these from input
    let interval = 0.2;

    run(interval);
}
