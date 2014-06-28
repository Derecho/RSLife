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
        self.cells.get(((y + (self.size as int))%(self.size as int)) as uint)
            .get(((x + (self.size as int))%(self.size as int)) as uint)
    }

    fn get_mut<'r>(&'r mut self, x: int, y: int) -> &'r mut Cell {
        self.cells.get_mut(((y + (self.size as int))%(self.size as int)) as uint)
            .get_mut(((x + (self.size as int))%(self.size as int)) as uint)
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
    fn tick(&mut self) {
        let mut newgrid = Grid::empty_grid(self.grid.size);
        for y in range(0, self.grid.size) {
            for x in range(0, self.grid.size) {
                let mut neighbours = 0;
                neighbours += (self.grid.get((x as int)-1, (y as int)-1).alive == true) as uint;
                neighbours += (self.grid.get((x as int)  , (y as int)-1).alive == true) as uint;
                neighbours += (self.grid.get((x as int)+1, (y as int)-1).alive == true) as uint;
                neighbours += (self.grid.get((x as int)-1, (y as int)  ).alive == true) as uint;
                neighbours += (self.grid.get((x as int)+1, (y as int)  ).alive == true) as uint;
                neighbours += (self.grid.get((x as int)-1, (y as int)+1).alive == true) as uint;
                neighbours += (self.grid.get((x as int)  , (y as int)+1).alive == true) as uint;
                neighbours += (self.grid.get((x as int)+1, (y as int)+1).alive == true) as uint;

                let cell = newgrid.get_mut(x as int, y as int);
                cell.alive = match neighbours {
                    3 if !cell.alive => true,
                    2 | 3 if cell.alive => true,
                    _ => false
                }
            }
        }
        self.grid = newgrid;
    }

    fn run(&mut self, interval: f32) {
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

    let mut game = Game { grid: Grid::random_grid(size)};
    game.run(interval);
}
