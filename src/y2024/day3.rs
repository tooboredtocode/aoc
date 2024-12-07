use crate::core::aoc_client::AocClientError;
use crate::core::{Input, Puzzle, PuzzlePart2};
use crate::util::StringError;
use lazy_regex::{lazy_regex, Lazy};
use regex::Regex;

/// https://adventofcode.com/2024/day/3
pub struct PuzzleSolution;

pub struct PuzzleInput {
    jumbled_instructions: String,
}

static INSTRUCTION_REGEX: Lazy<Regex> = lazy_regex!(r#"(?x)
      (?: mul\( ( \d{1,3} ), ( \d{1,3} ) \) )
    | (?: do\(\)                            )
    | (?: don't\(\)                         )
"#);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Instruction {
    Mul(u32, u32),
    Do,
    Dont,
}

impl Puzzle for PuzzleSolution {
    type Input = PuzzleInput;
    type FetchError = AocClientError<PuzzleInput>;
    type SolveError = StringError;

    async fn fetch_input(client: &crate::core::AocClient) -> Result<Self::Input, Self::FetchError> {
        client.get_challenge(2024, 3).await
    }

    async fn solve(input: Self::Input) -> Result<(), Self::SolveError> {
        let result = Instruction::parse_jumbled(&input.jumbled_instructions)?
            .into_iter()
            .filter_map(|instruction| match instruction { // Filter out non-mul instructions
                Instruction::Mul(a, b) => Some((a, b)),
                _ => None,
            })
            .map(|(a, b)| a * b)
            .sum::<u32>();

        println!("Result of the jumbled instructions: {}", result);

        Ok(())
    }
}

impl PuzzlePart2 for PuzzleSolution {
    async fn solve_part2(input: Self::Input) -> Result<(), Self::SolveError> {
        let result = Instruction::parse_jumbled(&input.jumbled_instructions)?
            .into_iter()
            .fold((0, true), |(acc, active), instruction| {
                match instruction {
                    Instruction::Mul(a, b) if active => (acc + a * b, active),
                    Instruction::Do => (acc, true),
                    Instruction::Dont => (acc, false),
                    _ => (acc, active),
                }
            }).0;

        println!("Result of the extended jumbled instructions: {}", result);

        Ok(())
    }
}

impl Input for PuzzleInput {
    type ParseError = StringError;

    async fn from_input(input: String) -> Result<Self, Self::ParseError> {
        Ok(Self {
            jumbled_instructions: input,
        })
    }
}

impl Instruction {
    fn parse_jumbled(jumbled: &str) -> Result<Vec<Self>, StringError> {
        let valid_instructions = INSTRUCTION_REGEX
            .captures_iter(jumbled)
            .map(|captures| {
                if captures[0].starts_with("do(") {
                    return Instruction::Do;
                }
                if captures[0].starts_with("don't(") {
                    return Instruction::Dont;
                }
                if captures[0].starts_with("mul(") {
                    let a: u32 = captures[1].parse().expect("Digits should be parseable");
                    let b: u32 = captures[2].parse().expect("Digits should be parseable");

                    return Instruction::Mul(a, b);
                }

                unreachable!("The regex should only match valid instructions");
            });

        Ok(valid_instructions.collect()) // Collect, so we don't need the haystack any more
    }
}
