extern crate drawille; 

use std::io::timer;
use std::mem;

use self::drawille::braille;

use super::grid::Grid;
use super::grid::FileGridError;

pub struct Game {
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

    pub fn run_ansi(&mut self, interval: f32) {
        let mut generation: uint = 0;
        loop {
            print!("\x1B[2J");  // Clear screen
            println!("Running Game of Life with {} fps", 1.0/interval);
            println!("Generation: {}", generation);
            self.current_grid.draw_ansi();

            timer::sleep((interval * 1000.0) as u64);
            self.tick();
            generation += 1;
        }
    }

    pub fn run_braille(&mut self, interval: f32) {
        let mut generation: uint = 0;
        let mut canvas = braille::Canvas::new(self.current_grid.width,
                                              self.current_grid.height);
        loop {
            print!("\x1B[2J");  // Clear screen
            println!("Running Game of Life with {} fps", 1.0/interval);
            println!("Generation: {}", generation);
            self.current_grid.draw_braille(&mut canvas);
            println!("{}", canvas.frame());

            timer::sleep((interval * 1000.0) as u64);
            self.tick();
            generation += 1;
        }
    }

    pub fn random_game(width: uint, height: uint) -> Game {
        Game { current_grid: Grid::random_grid(width, height),
               new_grid:     Grid::empty_grid(width, height) }
    }

    pub fn file_game(filename: &str) -> Result<Game, FileGridError>  {
        let current_grid = try!(Grid::file_grid(filename));
        let new_grid = Grid::empty_grid(current_grid.width, current_grid.height);
        Ok(Game { current_grid: current_grid,
               new_grid:     new_grid })
    }
}
