use seq_macro::seq;

use crate::core::AocClient;
use crate::core::io;
use self::day::AocDay;
pub use self::day::{UnsolvedDay, SolvedDay, SolvedDayPart1};

mod day;

seq!(N in 1..=25 {
    pub trait AocYear {
        const YEAR: u16;

        #(
            type Day~N: AocDay;
        )*
    }
});

pub trait AocYearExt: AocYear + sealed::Sealed {
    fn available_days() -> Vec<u8>;
    async fn run_day(day: u8, client: &AocClient);
    async fn benchmark_day(day: u8, client: &AocClient);
}

seq!(N in 1..=25 {
    impl<T: AocYear> AocYearExt for T {
        fn available_days() -> Vec<u8> {
            let mut days = Vec::new();
            #(
                if Self::Day~N::SOLVED {
                    days.push(N);
                }
            )*

            days
        }

        async fn run_day(day: u8, client: &AocClient) {
            match day {
                #(
                    N => Self::Day~N::run_day(day, client).await,
                )*
                _ => io::print_error(format_args!("Day {} is not part of Advent of Code", day)),
            }
        }

        async fn benchmark_day(day: u8, client: &AocClient) {
            match day {
                #(
                    N => Self::Day~N::benchmark_day(day, client).await,
                )*
                _ => io::print_error(format_args!("Day {} is not part of Advent of Code", day)),
            }
        }
    }
});

impl<T: AocYear> sealed::Sealed for T {}

mod sealed {
    pub trait Sealed {}
}
