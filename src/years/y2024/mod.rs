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

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

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
    ]);
    year
}