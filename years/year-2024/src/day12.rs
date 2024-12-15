use aoc_utils::matrix::{Direction, Matrix};
use aoc_utils::one_off::OneOff;
use aoc_utils::rustc_hash::FxHashSet;
use crate::prelude::*;

create_solution!(12);

#[derive(Debug)]
pub struct PuzzleInput {
    garden_plots: Matrix<char>,
}

create_puzzle_result!(PuzzleResultPart1, "Final fence cost: {}", cost: usize);
create_puzzle_result!(PuzzleResultPart2, "Final alternative fence cost: {}", cost: usize);

create_solution_part1!((input: PuzzleInput) -> PuzzleResultPart1 {
    let mut visited = FxHashSet::default();
    let cost = input.garden_plots.entry_iter()
        .filter_map(|entry| {
            if visited.contains(&entry.position()) {
                return None;
            }

            let mut area = 0;
            let mut perimiter = 0;

            for plot in entry.find_all_connected(|a,b| a.get() == b.get(), false) {
                visited.insert(plot.position());

                area += 1;
                // Get all the adjacent plots that are not the same as the current plot or are out of bounds
                perimiter += Direction::iter(false)
                    .filter(|dir| {
                        if let Some(adj) = plot.adjacent(*dir) {
                            adj.get() != plot.get()
                        } else {
                            true
                        }
                    })
                    .count();
            }

            return Some((area, perimiter));
        })
        .fold(0, |acc, (area, perimeter)| {
            acc + area * perimeter
        });

    Ok(PuzzleResultPart1 { cost })
});

create_solution_part2!((input: PuzzleInput) -> PuzzleResultPart2 {
    let mut visited = FxHashSet::default();
    let cost = input.garden_plots.entry_iter()
        .filter_map(|entry| {
            if visited.contains(&entry.position()) {
                return None;
            }

            let mut area = 0;
            let mut sides = 0;

            for plot in entry.find_all_connected(|a,b| a.get() == b.get(), false) {
                visited.insert(plot.position());

                area += 1;
                // Get all the adjacent plots that are not the same as the current plot or are out of bounds
                // and only count them if they are the last border (clockwise)
                sides += Direction::iter(false)
                    .filter(|dir| { // Check if this is a border
                        if let Some(adj) = plot.adjacent(*dir) {
                            adj.get() != plot.get()
                        } else {
                            true
                        }
                    })
                    .filter(|&dir| { // Check if the side continues clockwise
                        let dir_next = dir.rotate_right(false);

                        // Check if the next plot on the outside clockwise is the same as the current plot
                        // Eg: (* current plot, o plot in question, x known different plot)
                        // x o
                        // *
                        // If the plot in question is the same as the current plot, then the side continues upwards, and we should count it
                        // If the plot in question is different, then the side continues rightwards, and we should not count it
                        let same_outside = plot.adjacent(dir)
                            .and_then(|adj| adj.adjacent(dir_next))
                            .map(|adj| adj.get() == plot.get())
                            .unwrap_or(false);

                        if same_outside {
                            return true;
                        }

                        // Check if the next plot on the inside clockwise is the same as the current plot
                        // Eg: (* current plot, o plot in question, x known different plot)
                        // x
                        // * o
                        // If the plot in question is the same as the current plot, then the side continues rightwards, and we should not count it
                        // If the plot in question is different, then the side continues downwards, and we should count it
                        let same_inside = plot.adjacent(dir_next)
                            .map(|adj| adj.get() == plot.get())
                            .unwrap_or(false);

                        if !same_inside {
                            return true;
                        }

                        false
                    })
                    .count();
            }

            return Some((area, sides));
        })
        .fold(0, |acc, (area, sides)| {
            acc + area * sides
        });

    Ok(PuzzleResultPart2 { cost })
});

impl aoc_lib::PuzzleInput for PuzzleInput {
    fn from_input(input: &str) -> Result<Self> {
        let garden_plots = Matrix::try_from_string_chars(input.trim(), |c| {
            match c {
                'A'..='Z' => Ok(c),
                _ => Err(OneOff::new(format!("Invalid character: {}", c))),
            }
        }).context("Failed to parse garden plots")?;
        Ok(Self { garden_plots })
    }
}