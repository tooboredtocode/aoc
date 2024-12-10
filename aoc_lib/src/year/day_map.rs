use seq_macro::seq;

use crate::day::Day;

seq!(N in 1..=25 {
    #[derive(Debug)]
    pub(super) struct DayMap {
        #( day~N: Day, )*
    }
});


seq!(N in 1..=25 {
    impl DayMap {
        /// Creates a new `DayMap` for the given year.
        pub(super) fn new(year: u16) -> Self {
            Self {
                #( day~N: Day::unsolved(year, N), )*
            }
        }

        /// Returns the day for the given number.
        pub(super) fn get(&self, day: u8) -> Option<&Day> {
            match day {
                #(
                    N => Some(&self.day~N),
                )*
                _ => None,
            }
        }

        /// Sets the day for the given number.
        ///
        /// Note: if the day already exists, it will be replaced.
        pub(super) fn set(&mut self, day: u8, value: Day) {
            match day {
                #(
                    N => self.day~N = value,
                )*
                _ => {}
            }
        }

        /// Returns an iterator over all days.
        pub(super) fn days(&self) -> impl ExactSizeIterator<Item = &Day> {
            [ #( &self.day~N,)* ].into_iter()
        }
    }
});
