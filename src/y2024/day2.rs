use std::ops::RangeInclusive;
use crate::core::{AocClient, Input, Puzzle, PuzzlePart2};
use crate::core::aoc_client::AocClientError;
use crate::util::StringError;

/// https://adventofcode.com/2024/day/2
pub struct PuzzleSolution;

pub struct PuzzleInput {
    reports: Vec<Report>,
}

pub struct Report {
    levels: Vec<u32>,
}

enum ReportState {
    Initial,
    First {
        last: u32,
    },
    Decreasing {
        last: u32,
    },
    Increasing {
        last: u32,
    },
}

enum Difference {
    Positive(u32),
    Negative(u32),
}

const DIFFERENCE_RANGE: RangeInclusive<u32> = 1..=3;

impl Puzzle for PuzzleSolution {
    type Input = PuzzleInput;
    type FetchError = AocClientError<PuzzleInput>;
    type SolveError = StringError;

    async fn fetch_input(client: &AocClient) -> Result<Self::Input, Self::FetchError> {
        client.get_challenge(2024, 2).await
    }

    async fn solve(input: Self::Input) -> Result<(), Self::SolveError> {
        let valid_reports = input.reports.iter()
            .filter(|report| report.is_valid())
            .count();

        println!("Number of valid reports: {}", valid_reports);

        Ok(())
    }
}

impl PuzzlePart2 for PuzzleSolution {
    async fn solve_part2(input: Self::Input) -> Result<(), Self::SolveError> {
        let valid_reports = input.reports.iter()
            .filter(|report| {
                report.remove_one_iter()
                    .any(|report| report.is_valid()) // Check if any of the reports created by removing one level is valid
            })
            .count();

        println!("Number of valid reports: {}", valid_reports);

        Ok(())
    }
}

impl Input for PuzzleInput {
    type ParseError = StringError;

    async fn from_input(input: String) -> Result<Self, Self::ParseError> {
        let mut reports = Vec::new();

        for line in input.lines() {
            let levels = line.split_whitespace()
                .map(|s| s.parse::<u32>());

            let levels = match levels.collect::<Result<Vec<_>, _>>() {
                Ok(levels) => levels,
                Err(_) => return Err(StringError::new("AOC returned invalid input")),
            };

            reports.push(Report { levels });
        }

        Ok(PuzzleInput { reports })
    }
}

impl Report {
    fn is_valid(&self) -> bool {
        self.levels.iter()
            .copied()
            .try_fold(ReportState::Initial, |state, level| {
                match state {
                    ReportState::Initial => Ok(ReportState::First { last: level }),
                    ReportState::First { last } => {
                        match Difference::from_values(last, level) {
                            Difference::Positive(diff) if DIFFERENCE_RANGE.contains(&diff) => {
                                Ok(ReportState::Increasing { last: level })
                            },
                            Difference::Negative(diff) if DIFFERENCE_RANGE.contains(&diff) => {
                                Ok(ReportState::Decreasing { last: level })
                            },
                            _ => Err(()) // difference is not in range -> invalid report
                        }
                    },
                    ReportState::Decreasing { last } => {
                        match Difference::from_values(last, level) {
                            Difference::Negative(diff) if DIFFERENCE_RANGE.contains(&diff) => {
                                Ok(ReportState::Decreasing { last: level })
                            },
                            _ => Err(()) // we are either not decreasing or the difference is not in range -> invalid report
                        }
                    },
                    ReportState::Increasing { last } => {
                        match Difference::from_values(last, level) {
                            Difference::Positive(diff) if DIFFERENCE_RANGE.contains(&diff) => {
                                Ok(ReportState::Increasing { last: level })
                            },
                            _ => Err(()) // we are either not increasing or the difference is not in range -> invalid report
                        }
                    },
                }
            })
            .is_ok()
    }

    /// Yields all possible reports that can be created by removing one level from the report.
    fn remove_one_iter(&self) -> impl Iterator<Item=Report> + '_ {
        self.levels.iter()
            .copied()
            .enumerate()
            .map(move |(i, _)| {
                let mut levels = self.levels.clone();
                levels.remove(i);
                Report { levels }
            })
    }
}

impl Difference {
    fn from_values(initial: u32, next: u32) -> Self {
        if next > initial {
            Difference::Positive(next - initial)
        } else {
            Difference::Negative(initial - next)
        }
    }
}
