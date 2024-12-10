pub mod aoc_client;
pub mod io;
pub mod puzzle;
mod util;
pub mod day;
pub mod year;

pub use aoc_client::AocClient;
pub use puzzle::{
    PuzzleInput,
    PuzzleResult,
    Puzzle,
    SolutionPart1,
    SolutionPart2,
};
