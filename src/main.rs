use std::env;
use inquire::Select;

use crate::core::{io, AocClient};
use crate::util::handle_inquire_res;
use crate::years::Years;
use crate::cli::{Cli, Commands};

mod core;
mod util;
mod years;
mod y2024;
mod cli;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let benchmark = matches!(cli.command, Some(Commands::Benchmark));

    let Ok(session_cookie) = env::var("AOC_SESSION_COOKIE") else {
        io::print_error("AOC_SESSION_COOKIE environment variable must be set");
        return;
    };
    let client = AocClient::new(session_cookie);

    loop {
        let year = Years::get_year(benchmark);
        let Ok(year) = handle_inquire_res(year) else {
            return;
        };

        year_loop(year, &client, benchmark).await;
    }
}

async fn year_loop(year: Years, client: &AocClient, benchmark: bool) {
    let message = if benchmark {
        "Which day do you want to benchmark?"
    } else {
        "Which day do you want to run?"
    };

    loop {
        let day = Select::new(message, year.available_days())
            .prompt();
        let Ok(day) = handle_inquire_res(day) else {
            return;
        };
        if benchmark {
            year.benchmark_day(day, client).await;
        } else {
            year.run_day(day, client).await;
        }
    }
}