use aoc_lib::day::PartialDay;
use aoc_lib::year::{Year, Years};
use aoc_lib::AocClient;
use inquire::{Confirm, InquireError, Select};
use std::sync::Arc;
use itertools::Itertools;

mod util;
mod select;

use util::handle_inquire_res;
use select::{DisplayPartialDay, DisplayYear, Part};

pub async fn run_tui(years: &Years, client: &AocClient, benchmark: bool) {
    loop {
        let year = prompt_year(years, benchmark);
        let Ok(year) = handle_inquire_res(year) else {
            return;
        };

        year_loop(year, client, benchmark).await;
    }
}

fn prompt_year(years: &Years, benchmark: bool) -> Result<Arc<Year>, InquireError> {
    let msg = if benchmark {
        "Select year to benchmark tasks for"
    } else {
        "Select year to run tasks for"
    };

    let default = years.current();

    let msg_default = if benchmark {
        format!("Benchmark tasks for year {}?", default.year())
    } else {
        format!("Run tasks for year {}?", default.year())
    };

    let ans = Confirm::new(&msg_default)
        .with_default(true)
        .prompt()?;

    if ans {
        return Ok(default.clone());
    }

    let available_years = years.get_years()
        .sorted_by(|a, b| Ord::cmp(&b.year(), &a.year()))
        .map(|year| DisplayYear::new(year))
        .collect::<Vec<_>>();

    Select::new(msg, available_years)
        .prompt()
        .map(|year| year.year().clone())
}

async fn year_loop(year: Arc<Year>, client: &AocClient, benchmark: bool) {
    let message = if benchmark {
        "Which day do you want to benchmark?"
    } else {
        "Which day do you want to run?"
    };

    loop {
        let days = year.get_partial()
            .map(|day| DisplayPartialDay::new(day))
            .collect::<Vec<_>>();

        let day = Select::new(message, days)
            .prompt()
            .map(|day| day.day());
        let Ok(day) = handle_inquire_res(day) else {
            return;
        };

        day_prompt(day, client, benchmark).await;
    }
}

async fn day_prompt(day: PartialDay, client: &AocClient, benchmark: bool) {
    let message = if benchmark {
        "Which part do you want to benchmark?"
    } else {
        "Which part do you want to run?"
    };

    let day = match day {
        PartialDay::Partial(day) => {
            if benchmark {
                day.bench_part1(client, 50).await;
            } else {
                day.run_part1(client).await;
            }
            return;
        }
        PartialDay::Solved(day) => day
    };

    let part = Select::new(message, Part::vec())
        .prompt();
    let Ok(part) = handle_inquire_res(part) else {
        return;
    };

    match (part, benchmark) {
        (Part::Part1, true) => day.bench_part1(client, 50).await,
        (Part::Part1, false) => day.run_part1(client).await,
        (Part::Part2, true) => day.bench_part2(client, 50).await,
        (Part::Part2, false) => day.run_part2(client).await,
    }
}
