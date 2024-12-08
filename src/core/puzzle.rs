use std::error::Error;
use std::hint::black_box;
use crate::core::{io, AocClient, Input, PuzzleResult};
use crate::util::DisplayDuration;

pub trait Puzzle {
    type Input: Input + Send + Sync + 'static;

    type SolveError: Error + Send + Sync + 'static;

    type ResultPart1: PuzzleResult + Send + Sync + 'static;

    const YEAR: u32;
    const DAY: u32;

    /// Solve the puzzle.
    fn solve_part1(input: Self::Input) -> Result<Self::ResultPart1, Self::SolveError>;

    /// Run the puzzle.
    ///
    /// This is implemented by default, if you need to override it, you can do so.
    async fn run_part1(client: &AocClient) {
        let input = match client.get_challenge(Self::YEAR, Self::DAY).await {
            Ok(input) => input,
            Err(e) => {
                io::print_error(format_args!("Failed to fetch input: {}", e));
                return;
            }
        };

        match Self::solve_part1(input) {
            Ok(res) => res.display(),
            Err(e) => {
                io::print_error(format_args!("Failed to solve puzzle: {}", e));
            }
        }
    }

    /// Benchmark the puzzle.
    async fn benchmark_part1(client: &AocClient, iterations: u32) {
        let raw_input = match client.get_challenge_raw(Self::YEAR, Self::DAY).await {
            Ok(input) => input,
            Err(e) => {
                io::print_error(format_args!("Failed to fetch input: {}", e));
                return;
            }
        };

        fn inner<P: Puzzle + ?Sized>(input: String) -> bool {
            let input = match Input::from_input(input) {
                Ok(input) => input,
                Err(_) => return false,
            };
            match P::solve_part1(input) {
                Ok(_) => true,
                Err(_) => false,
            }
        }

        let now = std::time::Instant::now();
        for _ in 0..iterations {
            black_box(inner::<Self>(raw_input.clone()));
        }
        let elapsed = now.elapsed();
        let dur = elapsed / iterations;

        println!("Part 1: Average time: {}", DisplayDuration(dur));
    }
}

pub trait PuzzlePart2: Puzzle {
    type ResultPart2: PuzzleResult + Send + Sync + 'static;

    /// Solve part 2 of the puzzle.
    fn solve_part2(input: Self::Input) -> Result<Self::ResultPart2, Self::SolveError>;

    /// Run part 2 of the puzzle.
    ///
    /// This is implemented by default, if you need to override it, you can do so.
    async fn run_part2(client: &AocClient) {
        let input = match client.get_challenge(Self::YEAR, Self::DAY).await {
            Ok(input) => input,
            Err(e) => {
                io::print_error(format_args!("Failed to fetch input: {}", e));
                return;
            }
        };

        match Self::solve_part2(input) {
            Ok(res) => res.display(),
            Err(e) => {
                io::print_error(format_args!("Failed to solve part 2 of the puzzle: {}", e));
            }
        }
    }

    /// Benchmark part 2 of the puzzle.
    async fn benchmark_part2(client: &AocClient, iterations: u32) {
        let raw_input = match client.get_challenge_raw(Self::YEAR, Self::DAY).await {
            Ok(input) => input,
            Err(e) => {
                io::print_error(format_args!("Failed to fetch input: {}", e));
                return;
            }
        };

        fn inner<P: PuzzlePart2 + ?Sized>(input: String) -> bool {
            let input = match Input::from_input(input) {
                Ok(input) => input,
                Err(_) => return false,
            };
            match P::solve_part2(input) {
                Ok(_) => true,
                Err(_) => false,
            }
        }

        let now = std::time::Instant::now();
        for _ in 0..iterations {
            black_box(inner::<Self>(raw_input.clone()));
        }
        let elapsed = now.elapsed();
        let dur = elapsed / iterations;

        println!("Part 2: Average time: {}", DisplayDuration(dur));
    }
}
