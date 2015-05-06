extern crate rand;
extern crate drawille; 

use std::io::{BufRead, BufReader, Error};
use std::fs::File;

use self::drawille::{block, braille};

pub struct Cell {
    pub alive: bool
}

pub enum FileGridError {
    FileError(Error),
    InvalidGrid
}

pub struct Grid {
    cells: Vec<Vec<Cell>>,
    pub width: usize,
    pub height: usize
}
impl Grid {
    pub fn get<'r>(&'r self, x: i32, y: i32) -> &'r Cell {
        &self.cells[((y + (self.height as i32))%(self.height as i32)) as usize]
            [((x + (self.width as i32))%(self.width as i32)) as usize]
    }

    pub fn get_mut<'r>(&'r mut self, x: i32, y: i32) -> &'r mut Cell {
        &mut self.cells[((y + (self.height as i32))%(self.height as i32)) as usize]
            [((x + (self.width as i32))%(self.width as i32)) as usize]
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

    pub fn draw_block(&self, canvas: &mut block::Canvas) {
        for y in 0..self.height {
            for x in 0..self.width {
                match self.get(x as i32, y as i32).alive {
                    false => canvas.set(x, y, block::Color::Black),
                    true  => canvas.set(x, y, block::Color::White)
                }
            }
        }
    }

    pub fn draw_braille(&self, canvas: &mut braille::Canvas) {
        for y in 0..self.height {
            for x in 0..self.width {
                match self.get(x as i32, y as i32).alive {
                    false => canvas.unset(x, y),
                    true  => canvas.set(x, y)
                }
            }
        }
    }

    pub fn empty_grid(width: usize, height: usize) -> Grid {
        let mut grid = Vec::new();
        for _ in 0..height {
            let mut row = Vec::new();
            for _ in 0..width {
                row.push(Cell { alive: false });
            }
            grid.push(row);
        };
        Grid { cells: grid, width: width, height: height }
    }

    pub fn random_grid(width: usize, height: usize) -> Grid {
        let mut grid = Vec::new();
        for _ in 0..height {
            let mut row = Vec::new();
            for _ in 0..width {
                row.push(Cell { alive: rand::random() });
            }
            grid.push(row);
        };
        Grid { cells: grid, width: width, height: height }
    }
    
    pub fn file_grid(filename: &str) -> Result<Grid, FileGridError> {
        let file = try!(File::open(filename).map_err(FileGridError::FileError));
        let reader = BufReader::new(file);
        let mut grid = Vec::new();

        for line_result in reader.lines() {
            let mut row = Vec::new();
            let line = try!(line_result.map_err(FileGridError::FileError));
            for character in line.chars() {
                match character {
                    ' ' => row.push(Cell { alive: false }),
                    'X' => row.push(Cell { alive: true }),
                    '\n' => (),
                    _ => return Err(FileGridError::InvalidGrid)
                };
            }
            grid.push(row);
        }

        if grid.len() == 0 {
            return Err(FileGridError::InvalidGrid);
        }

        let mut found_width = 0;
        for row in grid.iter() {
            if found_width == 0 {
                found_width = row.len();
            }
            else if found_width != row.len() {
                return Err(FileGridError::InvalidGrid);
            }
        }

        let width = grid[0].len();
        let height = grid.len();
        Ok(Grid { cells: grid, width: width, height: height })
    }
}
