use aoc_lib::day::Day;
use aoc_lib::year::Year;

macro_rules! create_solution {
    ($day:literal) => {
        #[doc = concat!("https://adventofcode.com/2024/day/", $day)]
        pub struct PuzzleSolution;

        impl ::aoc_lib::puzzle::Puzzle for PuzzleSolution {
            const DAY: u8 = $day;
            const YEAR: u16 = 2024;
        }
    }
}

macro_rules! create_solution_part1 {
    (($input_i:ident: $input:ty) -> $result:ty { $($code:tt)* }) => {
        impl ::aoc_lib::SolutionPart1 for PuzzleSolution {
            type Input = $input;
            type SolveError = $crate::util::StringError;
            type Result = $result;

            fn solve($input_i: Self::Input) -> Result<Self::Result, Self::SolveError> {
                $($code)*
            }
        }
    }
}

macro_rules! create_solution_part2 {
    (($input_i:ident: $input:ty) -> $result:ty { $($code:tt)* }) => {
        impl ::aoc_lib::SolutionPart2 for PuzzleSolution {
            type Input = $input;
            type SolveError = $crate::util::StringError;
            type Result = $result;

            fn solve($input_i: Self::Input) -> Result<Self::Result, Self::SolveError> {
                $($code)*
            }
        }
    }
}

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;

pub fn year2024() -> Year {
    let mut year = Year::new(2024);
    year.add_days([
        Day::solved::<day1::PuzzleSolution>(),
        Day::solved::<day2::PuzzleSolution>(),
        Day::solved::<day3::PuzzleSolution>(),
        Day::solved::<day4::PuzzleSolution>(),
        Day::solved::<day5::PuzzleSolution>(),
        Day::solved::<day6::PuzzleSolution>(),
        Day::solved::<day7::PuzzleSolution>(),
        Day::solved::<day8::PuzzleSolution>(),
        Day::solved::<day9::PuzzleSolution>(),
        Day::solved::<day10::PuzzleSolution>(),
    ]);
    year
}