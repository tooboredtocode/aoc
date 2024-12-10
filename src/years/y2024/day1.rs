use aoc_lib::{impl_puzzle_result, PuzzleInput, SolutionPart1, SolutionPart2};
use crate::util::StringError;

create_solution!(1);

pub struct Input {
    pairs: Vec<(u32, u32)>,
}

pub struct ResultPart1 {
    sum_diff: u32,
}

impl_puzzle_result!(ResultPart1, "Sum of absolute differences: {}", sum_diff);

pub struct ResultPart2 {
    similarity_score: u64,
}

impl_puzzle_result!(ResultPart2, "Similarity score: {}", similarity_score);

impl SolutionPart1 for PuzzleSolution {
    type Input = Input;
    type SolveError = StringError;
    type Result = ResultPart1;

    fn solve(input: Self::Input) -> Result<Self::Result, Self::SolveError> {
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

impl SolutionPart2 for PuzzleSolution {
    type Input = Input;
    type SolveError = StringError;
    type Result = ResultPart2;

    fn solve(input: Self::Input) -> Result<Self::Result, Self::SolveError> {
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

impl PuzzleInput for Input {
    type ParseError = StringError;

    fn from_input(input: &str) -> Result<Self, Self::ParseError> {
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