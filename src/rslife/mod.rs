#![crate_id = "rslife#0.0"]
#![crate_type = "lib"]

#![feature(globs)]

pub use grid::*;
pub use game::*;

pub mod grid;
pub mod game;
