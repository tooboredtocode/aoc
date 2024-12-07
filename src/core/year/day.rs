use std::fmt;
use inquire::Select;
use crate::core::{AocClient, Puzzle, PuzzlePart2};
use crate::core::io;
use crate::util::handle_inquire_res;

pub struct UnsolvedDay;
pub struct SolvedDayPart1<P: Puzzle>(std::marker::PhantomData<P>);
pub struct SolvedDay<P: PuzzlePart2>(std::marker::PhantomData<P>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Part {
    Part1,
    Part2,
}

impl Part {
    fn ask_for_part() -> Option<Part> {
        let part = Select::new("Which part do you want to run?", vec![Part::Part1, Part::Part2])
            .prompt();

        handle_inquire_res(part).ok()
    }
}

impl fmt::Display for Part {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Part::Part1 => write!(f, "Part 1"),
            Part::Part2 => write!(f, "Part 2"),
        }
    }
}

pub trait AocDay: sealed::Sealed {
    const SOLVED: bool;
    const SOLVED_PART2: bool;

    async fn run_day(day: u8, client: &AocClient);
}

impl AocDay for UnsolvedDay {
    const SOLVED: bool = false;
    const SOLVED_PART2: bool = false;

    async fn run_day(day: u8, _client: &AocClient) {
        io::print_error(format_args!("Day {} has not been started yet", day));
    }
}

impl<P: Puzzle> AocDay for SolvedDayPart1<P> {
    const SOLVED: bool = true;
    const SOLVED_PART2: bool = false;

    async fn run_day(_day: u8, client: &AocClient) {
        P::run(client).await;
    }
}

impl<P: PuzzlePart2> AocDay for SolvedDay<P> {
    const SOLVED: bool = true;
    const SOLVED_PART2: bool = true;

    async fn run_day(_day: u8, client: &AocClient) {
        let Some(part) = Part::ask_for_part() else {
            return;
        };

        match part {
            Part::Part1 => P::run(client).await,
            Part::Part2 => P::run_part2(client).await,
        }
    }
}

impl sealed::Sealed for UnsolvedDay {}
impl<P: Puzzle> sealed::Sealed for SolvedDayPart1<P> {}
impl<P: PuzzlePart2> sealed::Sealed for SolvedDay<P> {}

mod sealed {
    pub trait Sealed {}
}