use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Benchmark the performance of the day's solutions
    Benchmark
}

impl Cli {
    pub fn parse() -> Self {
        Parser::parse()
    }
}