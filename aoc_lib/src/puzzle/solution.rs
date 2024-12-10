use std::error::Error;
use std::hint::black_box;
use crate::util::duration::DisplayDuration;
use super::{Puzzle, PuzzleInput, PuzzleResult};

pub trait SolutionPart1: Puzzle {
    type Input: PuzzleInput;
    type SolveError: Error;
    type Result: PuzzleResult;

    /// Solve the part1 of the puzzle.
    fn solve(input: Self::Input) -> Result<Self::Result, Self::SolveError>;
}

pub trait SolutionPart2: Puzzle {
    type Input: PuzzleInput;
    type SolveError: Error;
    type Result: PuzzleResult;

    /// Solve the part2 of the puzzle.
    fn solve(input: Self::Input) -> Result<Self::Result, Self::SolveError>;
}

async fn run_solution<F, I, Res, Err>(year: u16, day: u8, client: &crate::AocClient, f: F)
where
    F: FnOnce(I) -> Result<Res, Err>,
    I: PuzzleInput,
    Res: PuzzleResult,
    Err: Error,
{
    let input = match client.get_challenge(year, day).await {
        Ok(input) => input,
        Err(e) => {
            crate::io::print_error(format_args!("Failed to fetch input: {}", e));
            return;
        }
    };

    let input = match PuzzleInput::from_input_owned(input) {
        Ok(input) => input,
        Err(e) => {
            crate::io::print_error(format_args!("Failed to parse input: {}", e));
            return;
        }
    };

    match f(input) {
        Ok(res) => res.display(),
        Err(e) => {
            crate::io::print_error(format_args!("Failed to solve puzzle: {}", e));
        }
    }
}

fn benchmark_inner<F, I, Res, Err>(input: &str, f: F) -> bool
where
    F: FnOnce(I) -> Result<Res, Err>,
    I: PuzzleInput,
    Res: PuzzleResult,
    Err: Error,
{
    let input = match PuzzleInput::from_input(input) {
        Ok(input) => input,
        Err(_) => return false,
    };
    match f(input) {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn benchmark_inner_owned<F, I, Res, Err>(input: String, f: F) -> bool
where
    F: FnOnce(I) -> Result<Res, Err>,
    I: PuzzleInput,
    Res: PuzzleResult,
    Err: Error,
{
    let input = match PuzzleInput::from_input_owned(input) {
        Ok(input) => input,
        Err(_) => return false,
    };
    match f(input) {
        Ok(_) => true,
        Err(_) => false,
    }
}

async fn benchmark_solution<F, I, Res, Err, const PART2: bool>(year: u16, day: u8, client: &crate::AocClient, iterations: u32, mut f: F)
where
    F: FnMut(I) -> Result<Res, Err>,
    I: PuzzleInput,
    Res: PuzzleResult,
    Err: Error,
{
    let input = match client.get_challenge(year, day).await {
        Ok(input) => input,
        Err(e) => {
            if PART2 {
                crate::io::print_error(format_args!("Failed to fetch input for Part 2: {}", e));
            } else {
                crate::io::print_error(format_args!("Failed to fetch input for Part 1: {}", e));
            }
            return;
        }
    };

    if I::PREFERS_OWNED_INPUT {
        let input_clones = (0..iterations)
            .map(|_| input.clone())
            .collect::<Vec<_>>();

        let now = std::time::Instant::now();
        for input in input_clones {
            let res = black_box(benchmark_inner_owned(input, &mut f));
            if !res {
                if PART2 {
                    crate::io::print_error(format_args!("Failed to benchmark Part 2 of the puzzle, an error occurred"));
                } else {
                    crate::io::print_error(format_args!("Failed to benchmark Part 1 of the puzzle, an error occurred"));
                }
                return;
            }
        }
        let elapsed = now.elapsed();
        let dur = elapsed / iterations;

        if PART2 {
            println!("Part 2: Average time: {}", DisplayDuration(dur));
        } else {
            println!("Part 1: Average time: {}", DisplayDuration(dur));
        }
    } else {
        let now = std::time::Instant::now();
        for _ in 0..iterations {
            let res = black_box(benchmark_inner(&input, &mut f));
            if !res {
                if PART2 {
                    crate::io::print_error(format_args!("Failed to benchmark Part 2 of the puzzle, an error occurred"));
                } else {
                    crate::io::print_error(format_args!("Failed to benchmark Part 1 of the puzzle, an error occurred"));
                }
                return;
            }
        }
        let elapsed = now.elapsed();
        let dur = elapsed / iterations;

        if PART2 {
            println!("Part 2: Average time: {}", DisplayDuration(dur));
        } else {
            println!("Part 1: Average time: {}", DisplayDuration(dur));
        }
    }
}

pub trait SolutionPart1Ext: SolutionPart1 + sealed::SealedPart1 {
    /// Run the Part 1 of the puzzle.
    fn run_part1<'a>(client: &'a crate::AocClient) -> impl std::future::Future<Output = ()> + Send + Sync + 'a {
        run_solution(Self::YEAR, Self::DAY, client, Self::solve)
    }

    /// Benchmark the Part 1 of the puzzle.
    fn bench_part1<'a>(client: &'a crate::AocClient, iterations: u32) -> impl std::future::Future<Output = ()> + Send + Sync + 'a {
        benchmark_solution::<_,_,_,_,false>(Self::YEAR, Self::DAY, client, iterations, Self::solve)
    }
}

impl<T> SolutionPart1Ext for T where T: SolutionPart1 {}
impl<T> sealed::SealedPart1 for T where T: SolutionPart1 {}

pub trait SolutionPart2Ext: SolutionPart2 + sealed::SealedPart2 {
    /// Run the Part 2 of the puzzle.
    fn run_part2<'a>(client: &'a crate::AocClient) -> impl std::future::Future<Output = ()> + Send + Sync + 'a {
        run_solution(Self::YEAR, Self::DAY, client, Self::solve)
    }

    /// Benchmark the Part 2 of the puzzle.
    fn bench_part2<'a>(client: &'a crate::AocClient, iterations: u32) -> impl std::future::Future<Output = ()> + Send + Sync + 'a {
        benchmark_solution::<_,_,_,_,true>(Self::YEAR, Self::DAY, client, iterations, Self::solve)
    }
}

impl<T> SolutionPart2Ext for T where T: SolutionPart2 {}
impl<T> sealed::SealedPart2 for T where T: SolutionPart2 {}

mod sealed {
    pub trait SealedPart1 {}
    pub trait SealedPart2 {}
}
