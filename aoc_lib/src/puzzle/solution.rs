use std::hint::black_box;
use anyhow::{Context, Result};
use crate::util::duration::DisplayDuration;
use super::{Puzzle, PuzzleInput, PuzzleResult};

pub trait SolutionPart1: Puzzle {
    type Input: PuzzleInput;
    type Result: PuzzleResult;

    /// Solve the part1 of the puzzle.
    fn solve(input: Self::Input) -> Result<Self::Result>;
}

pub trait SolutionPart2: Puzzle {
    type Input: PuzzleInput;
    type Result: PuzzleResult;

    /// Solve the part2 of the puzzle.
    fn solve(input: Self::Input) -> Result<Self::Result>;
}

async fn run_solution<F, I, Res>(year: u16, day: u8, client: &crate::AocClient, f: F) -> Result<()>
where
    F: FnOnce(I) -> Result<Res>,
    I: PuzzleInput,
    Res: PuzzleResult,
{
    let input = client.get_challenge(year, day)
        .await
        .context("Failed to fetch input")?;

    let input = PuzzleInput::from_input_owned(input)
        .context("Failed to parse input")?;

    let res = f(input)
        .context("Puzzle solution failed")?;

    res.display();

    Ok(())
}

fn benchmark_inner<F, I, Res>(input: &str, f: F) -> Result<()>
where
    F: FnOnce(I) -> Result<Res>,
    I: PuzzleInput,
    Res: PuzzleResult,
{
    let input = PuzzleInput::from_input(input)
        .context("Failed to parse input")?;

    let _ = f(input)
        .context("Puzzle solution failed")?;

    Ok(())
}

fn benchmark_inner_owned<F, I, Res>(input: String, f: F) -> Result<()>
where
    F: FnOnce(I) -> Result<Res>,
    I: PuzzleInput,
    Res: PuzzleResult,
{
    let input = PuzzleInput::from_input_owned(input)
        .context("Failed to parse input")?;

    let _ = f(input)
        .context("Puzzle solution failed")?;

    Ok(())
}

async fn benchmark_solution<F, I, Res, const PART2: bool>(year: u16, day: u8, client: &crate::AocClient, iterations: u32, mut f: F) -> Result<()>
where
    F: FnMut(I) -> Result<Res>,
    I: PuzzleInput,
    Res: PuzzleResult,
{
    let input = client.get_challenge(year, day).await
        .context("Failed to fetch input")?;

    if I::PREFERS_OWNED_INPUT {
        let input_clones = (0..iterations)
            .map(|_| input.clone())
            .collect::<Vec<_>>();

        let now = std::time::Instant::now();
        for (i, input) in input_clones.into_iter().enumerate() {
            black_box(benchmark_inner_owned(input, &mut f))
                .with_context(|| format!("Benchmark failed on the {}. iteration", i))?;
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
        for i in 0..iterations {
            black_box(benchmark_inner(&input, &mut f))
                .with_context(|| format!("Benchmark failed on the {}. iteration", i))?;
        }
        let elapsed = now.elapsed();
        let dur = elapsed / iterations;

        if PART2 {
            println!("Part 2: Average time: {}", DisplayDuration(dur));
        } else {
            println!("Part 1: Average time: {}", DisplayDuration(dur));
        }
    }

    Ok(())
}

pub trait SolutionPart1Ext: SolutionPart1 + sealed::SealedPart1 {
    /// Run the Part 1 of the puzzle.
    fn run_part1<'a>(client: &'a crate::AocClient) -> impl std::future::Future<Output = Result<()>> + Send + Sync + 'a {
        run_solution(Self::YEAR, Self::DAY, client, Self::solve)
    }

    /// Benchmark the Part 1 of the puzzle.
    fn bench_part1<'a>(client: &'a crate::AocClient, iterations: u32) -> impl std::future::Future<Output = Result<()>> + Send + Sync + 'a {
        benchmark_solution::<_,_,_,false>(Self::YEAR, Self::DAY, client, iterations, Self::solve)
    }
}

impl<T> SolutionPart1Ext for T where T: SolutionPart1 {}
impl<T> sealed::SealedPart1 for T where T: SolutionPart1 {}

pub trait SolutionPart2Ext: SolutionPart2 + sealed::SealedPart2 {
    /// Run the Part 2 of the puzzle.
    fn run_part2<'a>(client: &'a crate::AocClient) -> impl std::future::Future<Output = Result<()>> + Send + Sync + 'a {
        run_solution(Self::YEAR, Self::DAY, client, Self::solve)
    }

    /// Benchmark the Part 2 of the puzzle.
    fn bench_part2<'a>(client: &'a crate::AocClient, iterations: u32) -> impl std::future::Future<Output = Result<()>> + Send + Sync + 'a {
        benchmark_solution::<_,_,_,true>(Self::YEAR, Self::DAY, client, iterations, Self::solve)
    }
}

impl<T> SolutionPart2Ext for T where T: SolutionPart2 {}
impl<T> sealed::SealedPart2 for T where T: SolutionPart2 {}

mod sealed {
    pub trait SealedPart1 {}
    pub trait SealedPart2 {}
}
