use std::ops::RangeInclusive;
use aoc_lib::{SolutionPart1, SolutionPart2};
use crate::prelude::*;

create_solution!(2);

pub struct Input {
    reports: Vec<Report>,
}

pub struct PuzzleResult {
    valid_reports: usize,
}

impl_puzzle_result!(PuzzleResult, "Number of valid reports {}", valid_reports);

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

impl SolutionPart1 for PuzzleSolution {
    type Input = Input;
    type Result = PuzzleResult;

    fn solve(input: Self::Input) -> Result<Self::Result> {
        let valid_reports = input.reports.iter()
            .filter(|report| report.is_valid())
            .count();

        Ok(PuzzleResult { valid_reports })
    }
}

impl SolutionPart2 for PuzzleSolution {
    type Input = Input;
    type Result = PuzzleResult;

    fn solve(input: Self::Input) -> Result<Self::Result> {
        let valid_reports = input.reports.iter()
            .filter(|report| {
                report.remove_one_iter()
                    .any(|report| report.is_valid()) // Check if any of the reports created by removing one level is valid
            })
            .count();

        Ok(PuzzleResult { valid_reports })
    }
}

impl PuzzleInput for Input {
    fn from_input(input: &str) -> Result<Self> {
        let mut reports = Vec::new();

        for line in input.lines() {
            let levels = line.split_whitespace()
                .map(|s|
                    s.parse::<u32>()
                        .context("Failed to parse level")
                );

            let levels = levels.collect::<Result<Vec<_>, _>>()?;

            reports.push(Report { levels });
        }

        Ok(Self { reports })
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
