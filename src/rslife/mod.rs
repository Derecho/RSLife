#![crate_name = "rslife"]
#![crate_type = "lib"]

#![feature(globs)]

pub use grid::*;
pub use game::*;

pub mod grid;
pub mod game;
