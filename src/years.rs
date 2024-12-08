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
    pub fn get_year(benchmark: bool) -> Result<Self, InquireError> {
        let default = {
            let d = util::get_aoc_season();
            match d {
                Year2024::YEAR => Some(Self::Year2024),
                _ => None,
            }
        };

        if let Some(year) = default {
            let msg = if benchmark {
                format!("Benchmark tasks for year {}?", year.year())
            } else {
                format!("Run tasks for year {}?", year.year())
            };

            let ans = Confirm::new(&msg)
                .with_default(true)
                .prompt()?;

            if ans {
                return Ok(year);
            }
        }

        let msg = if benchmark {
            "Select year to benchmark tasks for"
        } else {
            "Select year to run tasks for"
        };

        Select::new(msg, Self::available_years())
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

    pub async fn benchmark_day(&self, day: u8, client: &crate::core::AocClient) {
        match self {
            Self::Year2024 => Year2024::benchmark_day(day, client).await,
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