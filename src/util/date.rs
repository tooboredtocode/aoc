use chrono::{Datelike, Utc};

/// Get the current year if it's after summer, otherwise the previous year
pub fn get_aoc_season() -> u16 {
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