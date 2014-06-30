// Game of Life implementation in Rust by Derecho.

use std::io;
use std::io::{timer, File, BufferedReader};
use std::rand::random;
use std::mem;

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
    
    fn file_grid(filename: &str) -> Grid {
        let file = File::open(&Path::new(filename)).ok().expect("Failed to read file");
        let mut reader = BufferedReader::new(file);
        let mut grid = Vec::new();

        for line_result in reader.lines() {
            let mut row = Vec::new();
            let line = line_result.ok().expect("Failed to read line");
            for character in line.as_slice().chars() {
                match character {
                    ' ' => row.push(Cell { alive: false }),
                    'X' => row.push(Cell { alive: true }),
                    '\n' => (),
                    _ => fail!("Invalid character in file")
                };
            }
            grid.push(row);
        }

        let size = grid.len();  // TODO Enforce square input
        Grid { cells: grid, size: size }
    }
}

struct Game {
    current_grid: Grid,
    new_grid: Grid,
}
impl Game {
    fn tick(&mut self) {
        for y in range(0, self.current_grid.size) {
            for x in range(0, self.current_grid.size) {
                let mut neighbours = 0;
                neighbours += (self.current_grid.get((x as int)-1, (y as int)-1).alive == true) as uint;
                neighbours += (self.current_grid.get((x as int)  , (y as int)-1).alive == true) as uint;
                neighbours += (self.current_grid.get((x as int)+1, (y as int)-1).alive == true) as uint;
                neighbours += (self.current_grid.get((x as int)-1, (y as int)  ).alive == true) as uint;
                neighbours += (self.current_grid.get((x as int)+1, (y as int)  ).alive == true) as uint;
                neighbours += (self.current_grid.get((x as int)-1, (y as int)+1).alive == true) as uint;
                neighbours += (self.current_grid.get((x as int)  , (y as int)+1).alive == true) as uint;
                neighbours += (self.current_grid.get((x as int)+1, (y as int)+1).alive == true) as uint;

                let current_cell = self.current_grid.get(x as int, y as int);
                let new_cell = self.new_grid.get_mut(x as int, y as int);
                new_cell.alive = match neighbours {
                    2 if current_cell.alive => true,
                    3 => true,
                    _ => false
                }
            }
        }
        mem::swap(&mut self.current_grid, &mut self.new_grid);
    }

    fn run(&mut self, interval: f32) {
        let mut generation: uint = 0;
        loop {
            print!("\x1B[2J");  // Clear screen
            println!("Running Game of Life with {} fps", 1.0/interval);
            println!("Generation: {}", generation);
            self.current_grid.draw();

            timer::sleep((interval * 1000.0) as u64);
            self.tick();
            generation += 1;
        }
    }
}


fn main() {
    let default_interval = 0.2;
    let default_size = 32;
    let mut reader = io::stdin();

    print!("Interval [{}]: ", default_interval);
    let input = reader.read_line().ok().expect("Failed to read interval");
    let interval = from_str::<f32>(input.as_slice().trim()).unwrap_or(default_interval);

    print!("Filename (empty is a random grid) []: ");
    let input = reader.read_line().ok().expect("Failed to read filename");
    let filename = input.as_slice().trim();

    let mut size = 0;
    if filename == "" {
        print!("Size [{}]: ", default_size);
        let input = reader.read_line().ok().expect("Failed to read size");
        size = from_str::<uint>(input.as_slice().trim()).unwrap_or(default_size);
    }

    let mut game = match filename {
        "" => Game { current_grid: Grid::random_grid(size), new_grid: Grid::empty_grid(size) },
        _  => Game { current_grid: Grid::file_grid(filename), new_grid: Grid::file_grid(filename) }
    };
    game.run(interval);
}
