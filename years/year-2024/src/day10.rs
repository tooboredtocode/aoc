use aoc_utils::itertools::Itertools;
use aoc_utils::matrix::{Matrix, MatrixEntry};
use aoc_utils::matrix::ext::Yield;
use aoc_utils::one_off::OneOff;
use crate::prelude::*;

create_solution!(10);

#[derive(Debug)]
pub struct PuzzleInput {
    // Note: while a u8 would be sufficient for the input, using a u16 is more performant
    elevations: Matrix<u16>
}

create_puzzle_result!(PuzzleResultPart1, "Sum of trailhead scores by peaks: {}", score: u64);
create_puzzle_result!(PuzzleResultPart2, "Sum of trailhead scores by trails: {}", score: u64);

fn trail_predicate(c: &MatrixEntry<u16>, n: &MatrixEntry<u16>) -> Yield {
    if c.get().abs_diff(*n.get()) != 1 {
        return Yield::Cancel;
    }
    if *c.get() > *n.get() {
        return Yield::Cancel;
    }
    if *n.get() >= 9 {
        return Yield::Stop;
    }

    Yield::Continue
}

create_solution_part1!((input: PuzzleInput) -> PuzzleResultPart1 {
    let score = input.elevations.entry_iter()
        .filter(|e| *e.get() == 0) // Find all trailheads
        .map(|e| { // For each trailhead, find the potential trails
            e.try_yield_last(trail_predicate, false)
                .unique_by(|e| e.position())
                .count()
        })
        .sum::<usize>();

    Ok(PuzzleResultPart1 { score: score as u64 })
});

create_solution_part2!((input: PuzzleInput) -> PuzzleResultPart2 {
    let score = input.elevations.entry_iter()
        .filter(|e| *e.get() == 0) // Find all trailheads
        .map(|e| { // For each trailhead, find the potential trails
            e.try_yield_last(trail_predicate, false)
                .count()
        })
        .sum::<usize>();

    Ok(PuzzleResultPart2 { score: score as u64 })
});

impl aoc_lib::PuzzleInput for PuzzleInput {
    fn from_input(input: &str) -> Result<Self> {
        let elevations = Matrix::try_from_string_chars(input.trim(), |c| {
            match c {
                '0'..='9' => c.to_digit(10)
                    .map(|d| Ok(d as u16))
                    .expect("Digit should always be valid"),
                _ => Err(OneOff::new("Invalid character in input"))
            }
        }).context("Failed to parse map")?;

        Ok(PuzzleInput { elevations })
    }
}
