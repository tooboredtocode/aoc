use std::fmt::Display;
use std::io;
use crossterm::execute;
use crossterm::style::{Color, Print, ResetColor, SetForegroundColor};

pub fn print_error<T: Display>(msg: T) {
    execute!(
        io::stdout(),
        SetForegroundColor(Color::Red),
        Print(msg),
        Print("\n"),
        ResetColor
    ).expect("Failed to print error message");
}

pub fn print_debug<T: Display>(msg: T) {
    execute!(
        io::stdout(),
        SetForegroundColor(Color::Grey),
        Print(msg),
        Print("\n"),
        ResetColor
    ).expect("Failed to print debug message");
}