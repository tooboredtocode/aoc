use crate::core::{Input, Puzzle, PuzzlePart2};
use crate::core::aoc_client::AocClientError;
use crate::util::matrix::{Direction, Matrix};
use crate::util::StringError;

/// https://adventofcode.com/2024/day/4
pub struct PuzzleSolution;

pub struct PuzzleInput {
    word_search: Matrix<char>,
}

impl Puzzle for PuzzleSolution {
    type Input = PuzzleInput;
    type FetchError = AocClientError<PuzzleInput>;
    type SolveError = StringError;

    async fn fetch_input(client: &crate::core::AocClient) -> Result<Self::Input, Self::FetchError> {
        client.get_challenge(2024, 4).await
    }

    /// Find all xmases in the word search
    async fn solve(input: Self::Input) -> Result<(), Self::SolveError> {
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

        println!("Found {} xmases", result);

        Ok(())
    }
}

impl PuzzlePart2 for PuzzleSolution {

    /// Find all x-mas-es (aka two mas oriented in an x) in the word search
    async fn solve_part2(input: Self::Input) -> Result<(), Self::SolveError> {
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

        println!("Found {} x-mas-es", result);

        Ok(())
    }
}

impl Input for PuzzleInput {
    type ParseError = StringError;

    async fn from_input(input: String) -> Result<Self, Self::ParseError> {
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