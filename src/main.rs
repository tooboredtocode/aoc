use std::env;
use aoc_lib::{io, AocClient};

use crate::cli::{Cli, Commands};

mod cli;
mod years;
mod tui;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let benchmark = matches!(cli.command, Some(Commands::Benchmark));

    let Ok(session_cookie) = env::var("AOC_SESSION_COOKIE") else {
        io::print_error("AOC_SESSION_COOKIE environment variable must be set");
        return;
    };
    let client = AocClient::new(session_cookie);
    let years = years::years();

    tui::run_tui(&years, &client, benchmark).await;
}