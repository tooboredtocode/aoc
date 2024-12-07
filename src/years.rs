use std::fmt;
use inquire::{Confirm, InquireError, Select};
use crate::core::AocYear;
use crate::core::year::AocYearExt;
use crate::util;
use crate::y2024::Year2024;

pub enum Years {
    Year2024,
}

impl Years {
    pub fn get_year() -> Result<Self, InquireError> {
        let default = {
            let d = util::get_aoc_season();
            match d {
                Year2024::YEAR => Some(Self::Year2024),
                _ => None,
            }
        };

        if let Some(year) = default {
            let msg = format!("Run tasks for year {}?", year.year());
            let ans = Confirm::new(&msg)
                .with_default(true)
                .prompt()?;

            if ans {
                return Ok(year);
            }
        }

        Select::new("Select year to run tasks for", Self::available_years())
            .prompt()
    }

    pub fn available_days(&self) -> Vec<u8> {
        match self {
            Self::Year2024 => Year2024::available_days(),
        }
    }

    pub async fn run_day(&self, day: u8, client: &crate::core::AocClient) {
        match self {
            Self::Year2024 => Year2024::run_day(day, client).await,
        }
    }

    fn available_years() -> Vec<Self> {
        vec![Self::Year2024]
    }

    fn year(&self) -> u16 {
        match self {
            Self::Year2024 => Year2024::YEAR,
        }
    }
}

impl fmt::Display for Years {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Year2024 => write!(f, "{}", Year2024::YEAR),
        }
    }
}