use std::sync::Arc;
use aoc_lib::day::PartialDay;
use aoc_lib::year::Year;



pub struct DisplayYear<'a> {
    year: &'a Arc<Year>,
}

impl<'a> DisplayYear<'a> {
    pub fn new(year: &'a Arc<Year>) -> Self {
        Self { year }
    }

    pub fn year(self) -> &'a Arc<Year> {
        self.year
    }
}

impl<'a> std::fmt::Display for DisplayYear<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.year.year())
    }
}

pub struct DisplayPartialDay {
    day: PartialDay,
}

impl DisplayPartialDay {
    pub fn new(day: PartialDay) -> Self {
        Self { day }
    }

    pub fn day(self) -> PartialDay {
        self.day
    }
}

impl std::fmt::Display for DisplayPartialDay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Day {}", self.day.day())
    }
}

pub struct DisplayAlternatives {
    alternative: &'static str,
    day: PartialDay,
}

impl DisplayAlternatives {
    pub fn new(alternative: &'static str, day: PartialDay) -> Self {
        Self { alternative, day }
    }

    pub fn alternative(&self) -> &'static str {
        self.alternative
    }

    pub fn day(self) -> PartialDay {
        self.day
    }
}

impl std::fmt::Display for DisplayAlternatives {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.alternative.fmt(f)
    }
}

pub enum Part {
    Part1,
    Part2,
}

impl Part {
    pub fn vec() -> Vec<Self> {
        vec![Self::Part1, Self::Part2]
    }
}

impl std::fmt::Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Part1 => write!(f, "Part 1"),
            Self::Part2 => write!(f, "Part 2"),
        }
    }
}
