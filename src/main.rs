use std::env;
use inquire::Select;
use crate::core::{io, AocClient};
use crate::util::handle_inquire_res;
use crate::years::Years;

mod core;
mod util;
mod years;
mod y2024;

#[tokio::main]
async fn main() {
    let Ok(session_cookie) = env::var("AOC_SESSION_COOKIE") else {
        io::print_error("AOC_SESSION_COOKIE environment variable must be set");
        return;
    };
    let client = AocClient::new(session_cookie);

    loop {
        let year = Years::get_year();
        let Ok(year) = handle_inquire_res(year) else {
            return;
        };

        year_loop(year, &client).await;
    }
}

async fn year_loop(year: Years, client: &AocClient) {
    loop {
        let day = Select::new("Which task do you want to run?", year.available_days())
            .prompt();
        let Ok(day) = handle_inquire_res(day) else {
            return;
        };
        year.run_day(day, client).await;
    }
}