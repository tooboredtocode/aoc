use crate::core::{AocClient, Input, Puzzle, PuzzlePart2};
use crate::core::aoc_client::AocClientError;
use crate::util::StringError;

/// https://adventofcode.com/2024/day/1
pub struct PuzzleSolution;

pub struct PuzzleInput {
    pairs: Vec<(u32, u32)>,
}

impl Puzzle for PuzzleSolution {
    type Input = PuzzleInput;
    type FetchError = AocClientError<PuzzleInput>;
    type SolveError = StringError;

    async fn fetch_input(client: &AocClient) -> Result<Self::Input, Self::FetchError> {
        client.get_challenge(2024, 1).await
    }

    async fn solve(input: Self::Input) -> Result<(), Self::SolveError> {
        let mut list1 = input.pairs.iter().map(|(a, _)| *a).collect::<Vec<_>>();
        let mut list2 = input.pairs.iter().map(|(_, b)| *b).collect::<Vec<_>>();
        list1.sort();
        list2.sort();

        let mut sum_diff = 0;

        for (a, b) in list1.iter().copied().zip(list2.iter().copied()) {
            sum_diff += a.abs_diff(b);
        }

        println!("Sum of absolute differences: {}", sum_diff);

        Ok(())
    }
}

impl PuzzlePart2 for PuzzleSolution {
    async fn solve_part2(input: Self::Input) -> Result<(), Self::SolveError> {
        let list1 = input.pairs.iter().map(|(a, _)| *a).collect::<Vec<_>>();
        let list2 = input.pairs.iter().map(|(_, b)| *b).collect::<Vec<_>>();

        let mut similarity_score: u64 = 0;

        for a in list1.iter().copied() {
            let matches = list2.iter().copied().filter(|&b| b == a).count() as u64;
            similarity_score += (a as u64) * matches;
        }

        println!("Similarity score: {}", similarity_score);

        Ok(())
    }
}

impl Input for PuzzleInput {
    type ParseError = StringError;

    async fn from_input(input: String) -> Result<Self, Self::ParseError> {
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