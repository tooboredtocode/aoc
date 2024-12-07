use std::error::Error;

use crate::core::{io, AocClient};

pub trait Puzzle {
    type Input;

    type FetchError: Error + Send + Sync + 'static;
    type SolveError: Error + Send + Sync + 'static;

    /// Fetch the input for the puzzle.
    async fn fetch_input(client: &AocClient) -> Result<Self::Input, Self::FetchError>;

    /// Solve the puzzle.
    async fn solve(input: Self::Input) -> Result<(), Self::SolveError>;

    /// Run the puzzle.
    ///
    /// This is implemented by default, if you need to override it, you can do so.
    async fn run(client: &AocClient) {
        let input = match Self::fetch_input(client).await {
            Ok(input) => input,
            Err(e) => {
                io::print_error(format_args!("Failed to fetch input: {}", e));
                return;
            }
        };

        match Self::solve(input).await {
            Ok(()) => {},
            Err(e) => {
                io::print_error(format_args!("Failed to solve puzzle: {}", e));
            }
        }
    }
}

pub trait PuzzlePart2: Puzzle {
    /// Solve part 2 of the puzzle.
    async fn solve_part2(input: Self::Input) -> Result<(), Self::SolveError>;

    /// Run part 2 of the puzzle.
    ///
    /// This is implemented by default, if you need to override it, you can do so.
    async fn run_part2(client: &AocClient) {
        let input = match Self::fetch_input(client).await {
            Ok(input) => input,
            Err(e) => {
                io::print_error(format_args!("Failed to fetch input: {}", e));
                return;
            }
        };

        match Self::solve_part2(input).await {
            Ok(()) => {},
            Err(e) => {
                io::print_error(format_args!("Failed to solve part 2 of the puzzle: {}", e));
            }
        }
    }
}
