use aoc_lib::{SolutionPart1, SolutionPart2};
use aoc_utils::matrix::{Direction, Matrix};
use crate::prelude::*;

create_solution!(4);

pub struct PuzzleInput {
    word_search: Matrix<char>,
}

impl SolutionPart1 for PuzzleSolution {
    type Input = PuzzleInput;
    type Result = String;

    /// Find all xmases in the word search
    fn solve(input: Self::Input) -> Result<Self::Result> {
        let result = input.word_search
            .entry_iter()
            .filter(|item| item.get() == &'X')
            .map(|item| {
                Direction::iter_all()
                    .filter(|dir| {
                        let potential = item.adjacent_iter(*dir)
                            .map(|adjacent| *adjacent.get())
                            .take(3)
                            .collect::<Vec<_>>();

                        potential == vec!['M', 'A', 'S']
                    })
                    .count()
            })
            .sum::<usize>();

        Ok(format!("Found {} xmases", result))
    }
}

impl SolutionPart2 for PuzzleSolution {
    type Input = PuzzleInput;
    type Result = String;

    /// Find all x-mas-es (aka two mas oriented in an x) in the word search
    fn solve(input: Self::Input) -> Result<Self::Result> {
        let result = input.word_search
            .entry_iter()
            .filter(|item| item.get() == &'A') // Find all As so we can check for the other two letters
            .filter(|item| {
                let mut num_mas = 0;

                if item.adjacent(Direction::UpLeft).map_or(false, |adjacent| *adjacent.get() == 'M')
                    && item.adjacent(Direction::DownRight).map_or(false, |adjacent| *adjacent.get() == 'S') {
                    num_mas += 1;
                }
                if item.adjacent(Direction::UpRight).map_or(false, |adjacent| *adjacent.get() == 'M')
                    && item.adjacent(Direction::DownLeft).map_or(false, |adjacent| *adjacent.get() == 'S') {
                    num_mas += 1;
                }
                if item.adjacent(Direction::DownRight).map_or(false, |adjacent| *adjacent.get() == 'M')
                    && item.adjacent(Direction::UpLeft).map_or(false, |adjacent| *adjacent.get() == 'S') {
                    num_mas += 1;
                }
                if item.adjacent(Direction::DownLeft).map_or(false, |adjacent| *adjacent.get() == 'M')
                    && item.adjacent(Direction::UpRight).map_or(false, |adjacent| *adjacent.get() == 'S') {
                    num_mas += 1;
                }

                num_mas == 2
            })
            .count();

        Ok(format!("Found {} x-mas-es", result))
    }
}

impl aoc_lib::PuzzleInput for PuzzleInput {
    fn from_input(input: &str) -> Result<Self> {
        let matrix = Matrix::from_string_chars(input.trim(), |c| c)
            .context("Failed to parse input as a matrix")?;

        Ok(Self {
            word_search: matrix,
        })
    }
}