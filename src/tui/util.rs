use inquire::InquireError;
use aoc_lib::io;

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
