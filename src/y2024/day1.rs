use crate::core::{Input, Puzzle, PuzzlePart2, PuzzleResult};
use crate::util::StringError;

/// https://adventofcode.com/2024/day/1
pub struct PuzzleSolution;

pub struct PuzzleInput {
    pairs: Vec<(u32, u32)>,
}

pub struct ResultPart1 {
    sum_diff: u32,
}

pub struct ResultPart2 {
    similarity_score: u64,
}

impl Puzzle for PuzzleSolution {
    type Input = PuzzleInput;
    type SolveError = StringError;
    type ResultPart1 = ResultPart1;

    const YEAR: u32 = 2024;
    const DAY: u32 = 1;

    fn solve_part1(input: Self::Input) -> Result<ResultPart1, Self::SolveError> {
        let mut list1 = input.pairs.iter().map(|(a, _)| *a).collect::<Vec<_>>();
        let mut list2 = input.pairs.iter().map(|(_, b)| *b).collect::<Vec<_>>();
        list1.sort();
        list2.sort();

        let mut sum_diff = 0;

        for (a, b) in list1.iter().copied().zip(list2.iter().copied()) {
            sum_diff += a.abs_diff(b);
        }

        Ok(ResultPart1 { sum_diff })
    }
}

impl PuzzlePart2 for PuzzleSolution {
    type ResultPart2 = ResultPart2;

    fn solve_part2(input: Self::Input) -> Result<Self::ResultPart2, Self::SolveError> {
        let list1 = input.pairs.iter().map(|(a, _)| *a).collect::<Vec<_>>();
        let list2 = input.pairs.iter().map(|(_, b)| *b).collect::<Vec<_>>();

        let mut similarity_score: u64 = 0;

        for a in list1.iter().copied() {
            let matches = list2.iter().copied().filter(|&b| b == a).count() as u64;
            similarity_score += (a as u64) * matches;
        }

        Ok(ResultPart2 { similarity_score })
    }
}

impl PuzzleResult for ResultPart1 {
    fn display(&self) {
        println!("Sum of absolute differences: {}", self.sum_diff);
    }
}

impl PuzzleResult for ResultPart2 {
    fn display(&self) {
        println!("Similarity score: {}", self.similarity_score);
    }
}

impl Input for PuzzleInput {
    type ParseError = StringError;

    fn from_input(input: String) -> Result<Self, Self::ParseError> {
        let mut pairs = Vec::new();

        for line in input.lines() {
            let mut nums = line.split_whitespace()
                .map(|s| s.parse::<u32>());

            if let (Some(Ok(num1)), Some(Ok(num2))) = (nums.next(), nums.next()) {
                pairs.push((num1, num2));
            } else {
                return Err(StringError::new("AOC returned invalid input"));
            }

            if nums.next().is_some() {
                return Err(StringError::new("AOC returned invalid input"));
            }
        }

        Ok(Self { pairs })
    }
}