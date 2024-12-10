use std::fmt;
use crate::{SolutionPart1, SolutionPart2};

pub mod solved;
pub mod partial;
pub mod unsolved;

pub use solved::SolvedDay;
pub use partial::PartialSolvedDay;
pub use unsolved::UnsolvedDay;

#[derive(Clone)]
pub enum Day {
    Unsolved(UnsolvedDay),
    Partial(partial::PartialSolvedDayBox),
    Solved(solved::SolvedDayBox),
}

#[derive(Clone)]
pub enum PartialDay {
    Partial(partial::PartialSolvedDayBox),
    Solved(solved::SolvedDayBox),
}

impl Day {
    pub fn unsolved(year: u16, day: u8) -> Self {
        Self::Unsolved(UnsolvedDay::new(year, day))
    }

    pub fn partial<P: SolutionPart1>() -> Self {
        Self::Partial(PartialSolvedDay::<P>::boxed())
    }

    pub fn solved<P: SolutionPart1 + SolutionPart2>() -> Self {
        Self::Solved(SolvedDay::<P>::boxed())
    }

    pub fn is_partial(&self) -> bool {
        match self {
            Self::Unsolved(_) => false,
            Self::Partial(_) => true,
            Self::Solved(_) => true,
        }
    }

    pub fn is_solved(&self) -> bool {
        match self {
            Self::Unsolved(_) => false,
            Self::Partial(_) => false,
            Self::Solved(_) => true,
        }
    }

    pub fn try_into_partial(&self) -> Option<PartialDay> {
        match self {
            Self::Partial(day) => Some(PartialDay::Partial(day.clone())),
            Self::Solved(day) => Some(PartialDay::Solved(day.clone())),
            _ => None,
        }
    }

    pub fn try_into_solved(&self) -> Option<solved::SolvedDayBox> {
        match self {
            Self::Solved(day) => Some(day.clone()),
            _ => None,
        }
    }

    pub fn year(&self) -> u16 {
        match self {
            Self::Unsolved(day) => day.year(),
            Self::Partial(day) => day.year(),
            Self::Solved(day) => day.year(),
        }
    }

    pub fn day(&self) -> u8 {
        match self {
            Self::Unsolved(day) => day.day(),
            Self::Partial(day) => day.day(),
            Self::Solved(day) => day.day(),
        }
    }
}

impl PartialDay {
    pub fn year(&self) -> u16 {
        match self {
            Self::Partial(day) => day.year(),
            Self::Solved(day) => day.year(),
        }
    }

    pub fn day(&self) -> u8 {
        match self {
            Self::Partial(day) => day.day(),
            Self::Solved(day) => day.day(),
        }
    }
}

impl fmt::Debug for Day {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = f.debug_struct("Day");
        s.field("year", &self.year());
        s.field("day", &self.day());
        match self {
            Self::Unsolved(_) => s.field("status", &"Unsolved"),
            Self::Partial(_) => s.field("status", &"Partial"),
            Self::Solved(_) => s.field("status", &"Solved"),
        };
        s.finish()
    }
}
