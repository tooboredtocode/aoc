use aoc_lib::{SolutionPart1, SolutionPart2};
use aoc_utils::lazy_regex::{lazy_regex, Lazy, Regex};
use aoc_utils::lazy_regex;
use crate::prelude::*;

create_solution!(3);

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

pub struct PuzzleResult {
    result: u32,
    extended: bool,
}

impl SolutionPart1 for PuzzleSolution {
    type Input = PuzzleInput;
    type Result = PuzzleResult;

    fn solve(input: Self::Input) -> Result<Self::Result> {
        let result = Instruction::parse_jumbled(&input.jumbled_instructions)
            .into_iter()
            .filter_map(|instruction| match instruction { // Filter out non-mul instructions
                Instruction::Mul(a, b) => Some((a, b)),
                _ => None,
            })
            .map(|(a, b)| a * b)
            .sum::<u32>();

        Ok(PuzzleResult {
            result,
            extended: false,
        })
    }
}

impl SolutionPart2 for PuzzleSolution {
    type Input = PuzzleInput;
    type Result = PuzzleResult;

    fn solve(input: Self::Input) -> Result<Self::Result> {
        let result = Instruction::parse_jumbled(&input.jumbled_instructions)
            .into_iter()
            .fold((0, true), |(acc, active), instruction| {
                match instruction {
                    Instruction::Mul(a, b) if active => (acc + a * b, active),
                    Instruction::Do => (acc, true),
                    Instruction::Dont => (acc, false),
                    _ => (acc, active),
                }
            }).0;

        Ok(PuzzleResult {
            result,
            extended: true,
        })
    }
}

impl aoc_lib::PuzzleResult for PuzzleResult {
    fn display(&self) -> () {
        if self.extended {
            println!("Result of the extended jumbled instructions: {}", self.result)
        } else {
            println!("Result of the jumbled instructions: {}", self.result)
        }
    }
}

impl aoc_lib::PuzzleInput for PuzzleInput {
    const PREFERS_OWNED_INPUT: bool = true;

    fn from_input(input: &str) -> Result<Self> {
        Self::from_input_owned(input.to_string())
    }

    fn from_input_owned(input: String) -> Result<Self> {
        Ok(Self {
            jumbled_instructions: input,
        })
    }
}

impl Instruction {
    fn parse_jumbled(jumbled: &str) -> Vec<Self> {
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

        valid_instructions.collect() // Collect, so we don't need the haystack any more
    }
}
