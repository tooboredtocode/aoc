use std::collections::HashMap;
use std::sync::Arc;
use chrono::{Datelike, Utc};
use crate::year::Year;

pub struct Years {
    years: HashMap<u16, Arc<Year>>,
    current: Arc<Year>,
}

/// Get the current year if it's after summer, otherwise the previous year
pub fn current_aoc_season() -> u16 {
    let now = Utc::now();
    let year = now.year();
    let month = now.month();
    let aoc_year = if month <= 6 {
        year - 1 // We want the year of the previous season at the beginning of the year
    } else {
        year // We want the year of the current season after summer
    };

    // Advent of Code started in 2015
    if aoc_year < 2015 {
        2015
    } else {
        aoc_year as u16
    }
}

impl Years {
    pub fn new() -> Self {
        let current = current_aoc_season();

        let mut years = HashMap::new();

        let current = Arc::new(Year::new(current));
        years.insert(current.year(), current.clone());

        Self { years, current }
    }

    pub fn add_year(&mut self, year: Year) {
        let new = Arc::new(year);

        if self.current.year() == new.year() {
            self.current = new.clone();
        }
        self.years.insert(new.year(), new);
    }

    pub fn current(&self) -> &Arc<Year> {
        &self.current
    }

    pub fn get_year(&self, year: u16) -> Option<&Arc<Year>> {
        self.years.get(&year)
    }

    pub fn get_years(&self) -> impl Iterator<Item = &Arc<Year>> {
        self.years.values()
    }
}