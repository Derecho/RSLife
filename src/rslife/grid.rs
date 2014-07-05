use std::io::{BufferedReader, File, IoError};
use std::rand::random;

pub struct Cell {
    pub alive: bool
}

pub enum FileGridError {
    FileError(IoError),
    InvalidGrid
}

pub struct Grid {
    cells: Vec<Vec<Cell>>,
    pub width: uint,
    pub height: uint
}
impl Grid {
    pub fn get<'r>(&'r self, x: int, y: int) -> &'r Cell {
        self.cells.get(((y + (self.height as int))%(self.height as int)) as uint)
            .get(((x + (self.width as int))%(self.width as int)) as uint)
    }

    pub fn get_mut<'r>(&'r mut self, x: int, y: int) -> &'r mut Cell {
        self.cells.get_mut(((y + (self.height as int))%(self.height as int)) as uint)
            .get_mut(((x + (self.width as int))%(self.width as int)) as uint)
    }

    pub fn draw_ansi(&self) {
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

    pub fn empty_grid(width: uint, height: uint) -> Grid {
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

    pub fn random_grid(width: uint, height: uint) -> Grid {
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
    
    pub fn file_grid(filename: &str) -> Result<Grid, FileGridError> {
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
