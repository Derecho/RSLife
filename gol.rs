// Game of Life implementation in Rust by Derecho.

use std::io;
use std::io::{timer, File, BufferedReader, IoError};
use std::rand::random;
use std::mem;

struct Cell {
    alive: bool
}

enum FileGridError {
    FileError(IoError),
    InvalidGrid
}

struct Grid {
    cells: Vec<Vec<Cell>>,
    width: uint,
    height: uint
}
impl Grid {
    fn get<'r>(&'r self, x: int, y: int) -> &'r Cell {
        self.cells.get(((y + (self.height as int))%(self.height as int)) as uint)
            .get(((x + (self.width as int))%(self.width as int)) as uint)
    }

    fn get_mut<'r>(&'r mut self, x: int, y: int) -> &'r mut Cell {
        self.cells.get_mut(((y + (self.height as int))%(self.height as int)) as uint)
            .get_mut(((x + (self.width as int))%(self.width as int)) as uint)
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

    fn empty_grid(width: uint, height: uint) -> Grid {
        let mut grid = Vec::new();
        for _ in range(0, height) {
            let mut row = Vec::new();
            for _ in range(0, width) {
                row.push(Cell { alive: false });
            }
            grid.push(row);
        };
        Grid { cells: grid, width: width, height: height }
    }

    fn random_grid(width: uint, height: uint) -> Grid {
        let mut grid = Vec::new();
        for _ in range(0, height) {
            let mut row = Vec::new();
            for _ in range(0, width) {
                row.push(Cell { alive: random::<bool>() });
            }
            grid.push(row);
        };
        Grid { cells: grid, width: width, height: height }
    }
    
    fn file_grid(filename: &str) -> Result<Grid, FileGridError> {
        let file = try!(File::open(&Path::new(filename)).map_err(FileError));
        let mut reader = BufferedReader::new(file);
        let mut grid = Vec::new();

        for line_result in reader.lines() {
            let mut row = Vec::new();
            let line = try!(line_result.map_err(FileError));
            for character in line.as_slice().chars() {
                match character {
                    ' ' => row.push(Cell { alive: false }),
                    'X' => row.push(Cell { alive: true }),
                    '\n' => (),
                    _ => return Err(InvalidGrid)
                };
            }
            grid.push(row);
        }

        if grid.len() == 0 {
            return Err(InvalidGrid);
        }

        let mut found_width = 0;
        for row in grid.iter() {
            if found_width == 0 {
                found_width = row.len();
            }
            else if found_width != row.len() {
                return Err(InvalidGrid);
            }
        }

        let width = grid.get(0).len();
        let height = grid.len();
        Ok(Grid { cells: grid, width: width, height: height })
    }
}

struct Game {
    current_grid: Grid,
    new_grid: Grid,
}
impl Game {
    fn tick(&mut self) {
        for y in range(0, self.current_grid.height) {
            for x in range(0, self.current_grid.width) {
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

    fn random_game(width: uint, height: uint) -> Game {
        Game { current_grid: Grid::random_grid(width, height),
               new_grid:     Grid::empty_grid(width, height) }
    }

    fn file_game(filename: &str) -> Result<Game, FileGridError>  {
        let current_grid = try!(Grid::file_grid(filename));
        let new_grid = Grid::empty_grid(current_grid.width, current_grid.height);
        Ok(Game { current_grid: current_grid,
               new_grid:     new_grid })
    }
}


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
        "" => Game::random_game(width, height),
        _  => Game::file_game(filename).ok().expect("Failed to read file")
    };
    game.run(interval);
}
