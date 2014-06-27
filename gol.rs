// Game of Life implementation in Rust by Derecho.
extern crate debug;

use std::io::timer;
use std::rand::random;

struct Cell {
    alive: bool
}

struct Grid {
    cells: Vec<Vec<Cell>>
}

struct Game {
    grid: Grid
}
impl Game {
    fn draw_grid(&self) {
        for row in self.grid.cells.iter() {
            println!("[D] row: {:?}", row);
            for cell in row.iter() {
                println!("[D] cell: {:?}", cell);
                match cell.alive {
                    false => print!(" "),
                    true  => print!("X")
                }
            }
            println!("");
        }
    }

    fn run(&self, interval: f32) {
        let mut generation = 0;
        loop {
            print!("\x1B[2J");  // Clear screen
            println!("Running Game of Life with {} fps", 1.0/interval);
            println!("Generation: {}", generation);

            self.draw_grid();

            generation += 1;
            timer::sleep((interval * 1000.0) as u64);
        }
    }

    fn empty_grid(size: int) -> Grid {
        let mut grid = vec![];
        let mut i = 0;
        let mut j = 0;
        while i < size {
            let mut row = vec![];
            while j < size {
                row.push(Cell { alive: false });
                j += 1;
            }
            grid.push(row);
            i += 1;
        };
        Grid { cells: grid }
    }

    fn random_grid(size: int) -> Grid {
        let mut grid = vec![];
        let mut i = 0;
        let mut j = 0;
        while i < size {
            let mut row = vec![];
            while j < size {
                row.push(Cell { alive: random::<bool>() });
                j += 1;
            }
            grid.push(row);
            i += 1;
        };
        Grid { cells: grid }
    }
}


fn main() {
    // Get these from input
    let interval = 0.2;
    let size = 10;

    let game = Game { grid: Game::random_grid(size) };
    game.run(interval);
}
