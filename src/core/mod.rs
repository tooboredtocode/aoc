mod puzzle;
pub mod aoc_client;
mod input;
pub mod io;
pub mod year;
mod result;

pub use puzzle::{Puzzle, PuzzlePart2};
pub use input::Input;
pub use aoc_client::AocClient;
pub use year::AocYear;
pub use result::PuzzleResult;
