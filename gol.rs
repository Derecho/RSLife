// Game of Life implementation in Rust by Derecho.

use std::io::timer;
use std::rand::random;

struct Cell {
    alive: bool
}

struct Grid {
    cells: Vec<Vec<Cell>>,
    size: uint
}
impl Grid {
    fn get<'r>(&'r self, x: int, y: int) -> &'r Cell {
        self.cells.get((y % self.size as int) as uint)
            .get((x % self.size as int) as uint)
    }

    fn draw(&self) {
        for row in self.cells.iter() {
            for cell in row.iter() {
                match cell.alive {
                    false => print!(" "),
                    true  => print!("X")
                }
            }
            println!("");
        }
    }

    #[allow(dead_code)]
    fn empty_grid(size: uint) -> Grid {
        let mut grid = Vec::new();
        for _ in range(0, size) {
            let mut row = Vec::new();
            for _ in range(0, size) {
                row.push(Cell { alive: false });
            }
            grid.push(row);
        };
        Grid { cells: grid, size: size }
    }

    fn random_grid(size: uint) -> Grid {
        let mut grid = Vec::new();
        for _ in range(0, size) {
            let mut row = Vec::new();
            for _ in range(0, size) {
                row.push(Cell { alive: random::<bool>() });
            }
            grid.push(row);
        };
        Grid { cells: grid, size: size }
    }
}

struct Game {
    grid: Grid
}
impl Game {
    fn tick(&self) {
        // TODO
    }

    fn run(&self, interval: f32) {
        let mut generation = 0;
        loop {
            print!("\x1B[2J");  // Clear screen
            println!("Running Game of Life with {} fps", 1.0/interval);
            println!("Generation: {}", generation);
            self.grid.draw();

            timer::sleep((interval * 1000.0) as u64);
            self.tick();
            generation += 1;
        }
    }
}


fn main() {
    // Get these from input
    let interval = 0.2;
    let size = 10;

    let game = Game { grid: Grid::random_grid(size)};
    game.run(interval);
}
