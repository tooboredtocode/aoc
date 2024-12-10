use aoc_lib::{SolutionPart1, SolutionPart2};
use crate::util::matrix::{Direction, Matrix};
use crate::util::StringError;

create_solution!(4);

pub struct PuzzleInput {
    word_search: Matrix<char>,
}

impl SolutionPart1 for PuzzleSolution {
    type Input = PuzzleInput;
    type SolveError = StringError;
    type Result = String;

    /// Find all xmases in the word search
    fn solve(input: Self::Input) -> Result<Self::Result, Self::SolveError> {
        let result = input.word_search
            .entry_iter()
            .filter(|item| item.get() == &'X')
            .map(|item| {
                Direction::iter()
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
    type SolveError = StringError;
    type Result = String;

    /// Find all x-mas-es (aka two mas oriented in an x) in the word search
    fn solve(input: Self::Input) -> Result<Self::Result, Self::SolveError> {
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
    type ParseError = StringError;

    fn from_input(input: &str) -> Result<Self, Self::ParseError> {
        let line_length = input.lines().next().map_or(0, |line| line.chars().count());
        if line_length == 0 {
            return Err(StringError::new("No lines in the input"));
        }
        if input.lines().any(|line| line.chars().count() != line_length) {
            return Err(StringError::new("Lines in the input have different lengths"));
        }
        let row_height = input.lines().count();
        if row_height == 0 {
            return Err(StringError::new("No rows in the input"));
        }

        let text = input.lines().flat_map(|line| line.chars()).collect::<Vec<_>>();

        let matrix = Matrix::from_vec(line_length, row_height, text);

        Ok(Self {
            word_search: matrix,
        })
    }
}