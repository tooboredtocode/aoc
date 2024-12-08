use inquire::InquireError;

use crate::core::io;

mod date;
mod err;
pub mod matrix;
mod duration;

pub use date::get_aoc_season;
pub use err::StringError;
pub use duration::DisplayDuration;

pub struct Break;

pub fn handle_inquire_res<T>(res: Result<T, InquireError>) -> Result<T, Break> {
    match res {
        Ok(val) => Ok(val),
        Err(InquireError::OperationCanceled)
        | Err(InquireError::OperationInterrupted) => {
            // Quietly exit on cancel or interrupt
            Err(Break)
        }
        Err(e) => {
            io::print_error(format_args!("Something went wrong: {}", e));
            std::process::exit(1);
        }
    }
}
