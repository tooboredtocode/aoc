use std::collections::HashMap;
use seq_macro::seq;

use crate::day::Day;

seq!(N in 1..=25 {
    #[derive(Debug)]
    pub(super) struct DayMap {
        #( day~N: DayMapValue, )*
    }
});

#[derive(Debug)]
pub(super) struct DayMapValue {
    pub(super) default_day: Day,
    pub(super) alternatives: Option<HashMap<&'static str, Day>>,
}

seq!(N in 1..=25 {
    impl DayMap {
        /// Creates a new `DayMap` for the given year.
        pub(super) fn new(year: u16) -> Self {
            Self {
                #( day~N: DayMapValue::new(year, N), )*
            }
        }

        /// Returns the day for the given number.
        ///
        /// Panics if the day is not a valid aoc day (1-25).
        pub(super) fn get(&self, day: u8) -> &DayMapValue {
            match day {
                #(
                    N => &self.day~N,
                )*
                _ => panic!("Invalid day number: {}", day),
            }
        }

        /// Sets the day for the given number.
        ///
        /// Note: if the day already exists, it will be replaced.
        pub(super) fn set(&mut self, value: Day) {
            match value.day() {
                #(
                    N => self.day~N.insert(value),
                )*
                _ => {}
            }
        }

        /// Returns an iterator over all days.
        pub(super) fn days(&self) -> impl ExactSizeIterator<Item = &DayMapValue> {
            [ #( &self.day~N,)* ].into_iter()
        }
    }
});

impl DayMapValue {
    fn new(year: u16, day: u8) -> Self {
        Self {
            default_day: Day::unsolved(year, day),
            alternatives: None,
        }
    }

    fn insert(&mut self, value: Day) {
        if let Some(key) = value.alt() {
            self.insert_into_option_map(key, value.clone());
        }

        self.default_day = value;
    }

    fn insert_into_option_map(&mut self, key: &'static str, value: Day) {
        if let Some(map) = &mut self.alternatives {
            map.insert(key, value);
        } else {
            let mut new_map = HashMap::new();
            new_map.insert(key, value);
            self.alternatives = Some(new_map);
        }
    }
}
