use crate::day::{Day, PartialDay};
use crate::day::solved::SolvedDayBox;
use super::day_map::DayMap;

#[derive(Debug)]
pub struct Year {
    year: u16,
    days: DayMap,
}

impl Year {
    /// Create a new year with the given year number.
    pub fn new(year: u16) -> Self {
        Self {
            year,
            days: DayMap::new(year),
        }
    }

    /// Add a task day to the year.
    ///
    /// # Panics
    /// This function will panic if the day's year does not match the year of this year.
    pub fn add_day(&mut self, day: Day) {
        assert_eq!(day.year(), self.year);
        self.days.set(day.day(), day);
    }

    /// Add multiple task days to the year.
    ///
    /// # Panics
    /// This function will panic if any of the days' year does not match the year of this year.
    pub fn add_days(&mut self, days: impl IntoIterator<Item = Day>) {
        for day in days {
            self.add_day(day);
        }
    }

    /// Get the day with the given number.
    pub fn get_day(&self, day: u8) -> Option<&Day> {
        self.days.get(day)
    }

    pub fn get_partial(&self) -> impl Iterator<Item = PartialDay> + '_ {
        self.days.days()
            .filter_map(Day::try_into_partial)
    }

    pub fn get_solved(&self) -> impl Iterator<Item = SolvedDayBox> + '_ {
        self.days.days()
            .filter_map(Day::try_into_solved)
    }

    pub fn year(&self) -> u16 {
        self.year
    }
}
