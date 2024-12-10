use aoc_lib::{SolutionPart1, SolutionPart2};
use crate::util::StringError;

create_solution!(7);

#[derive(Debug)]
pub struct PuzzleInput {
    equations: Vec<Equation>,
}

#[derive(Debug)]
pub struct Equation {
    result: u64,
    operands: Vec<u64>,
}

impl SolutionPart1 for PuzzleSolution {
    type Input = PuzzleInput;
    type SolveError = StringError;
    type Result = String;

    fn solve(input: Self::Input) -> Result<Self::Result, Self::SolveError> {
        let mut valid = 0u64;
        let result = Equation::valid_equations(&input.equations, false)
            .map(|equation| equation.result)
            .inspect(|_| valid += 1)
            .sum::<u64>();

        Ok(format!("Found {} valid equations which total to {}", valid, result))
    }
}

impl SolutionPart2 for PuzzleSolution {
    type Input = PuzzleInput;
    type SolveError = StringError;
    type Result = String;

    fn solve(input: Self::Input) -> Result<Self::Result, Self::SolveError> {
        let mut valid = 0u64;
        let result = Equation::valid_equations(&input.equations, true)
            .map(|equation| equation.result)
            .inspect(|_| valid += 1)
            .sum::<u64>();

        Ok(format!("Found {} extended valid equations which total to {}", valid, result))
    }
}

impl Equation {
    fn valid_equations<'a>(input: impl IntoIterator<Item=&'a Equation>, allow_concat: bool) -> impl Iterator<Item=&'a Equation> {
        input.into_iter()
            .filter(move |equation| equation.is_valid(allow_concat))
    }

    fn is_valid(&self, allow_concat: bool) -> bool {
        fn inner(result: u64, remaining: &[u64], allow_concat: bool) -> bool {
            if result == 0 {
                // Result is 0, either we have used all operands (valid outcome) or we have not (invalid outcome)
                return remaining.is_empty();
            }
            if remaining.is_empty() {
                // We have a result but no operands to use
                return false;
            }

            let operand = remaining[remaining.len() - 1];
            let remaining = &remaining[..remaining.len() - 1];

            if allow_concat {
                let result_str = result.to_string();
                let operand_str = operand.to_string();

                if result_str.ends_with(&operand_str) {
                    let mut new_result = result - operand; // Zero out the digits the operand contributed
                    new_result = new_result / 10u64.pow(operand_str.len() as u32); // Remove the digits the operand contributed
                    if inner(new_result, remaining, allow_concat) {
                        return true;
                    }
                }
            }

            if result < operand {
                // We can't subtract or divide a number from a smaller number
                false
            } else {
                let new_result = result - operand;
                if inner(new_result, remaining, allow_concat) {
                    return true;
                }

                // Result from subtraction was invalid, try division
                if result % operand != 0 {
                    // We can't divide the result by the operand
                    return false;
                }
                let new_result = result / operand;
                if inner(new_result, remaining, allow_concat) {
                    return true;
                }

                // We can't find a valid equation
                false
            }
        }

        inner(self.result, &self.operands, allow_concat)
    }
}

impl aoc_lib::PuzzleInput for PuzzleInput {
    type ParseError = StringError;

    fn from_input(input: &str) -> Result<Self, Self::ParseError> {
        input.lines()
            .map(|line| {
                let (result, operands) = line.split_once(":")
                    .ok_or_else(|| StringError::new("Expected lines in the format 'result: operand1 operand2'"))?;

                let result = result.trim()
                    .parse()
                    .map_err(|err| StringError::with_cause("Expected a number: ", err))?;
                let operands = operands.split_whitespace()
                    .map(|operand|
                        operand.parse()
                            .map_err(|err| StringError::with_cause("Expected a number: ", err))
                    )
                    .collect::<Result<Vec<u64>, _>>()?;

                Ok(Equation { result, operands })
            })
            .collect::<Result<Vec<Equation>, _>>()
            .map(|equations| PuzzleInput { equations })
    }
}