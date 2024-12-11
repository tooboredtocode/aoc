mod try_yield;

use itertools::Itertools;
use crate::util::{matrix, StringError};

#[allow(unused_imports)]
pub use try_yield::{Yield, YieldIter};

impl<T> super::Matrix<T> {
    pub fn try_from_string_chars(string: &str, map: impl Fn(char) -> Result<T, StringError>) -> Result<Self, StringError> {
        let mut width = 0;
        let mut local_width = 0;
        let mut height = 0;

        let store = string.chars()
            .filter_map(|c| {
                if c == '\n' {
                    height += 1;
                    if local_width == 0 {
                        local_width = width;
                    }
                    if local_width != width {
                        return Some(Err(StringError::new("Uneven rows in the input")));
                    }
                    width = 0;
                    None
                } else {
                    width += 1;
                    Some(map(c))
                }
            })
            .try_collect()?;

        if height == 0 {
            return Err(StringError::new("No rows in the input"));
        } else if width == 0 {
            return Err(StringError::new("No columns in the input"));
        }

        // If the last character is not a newline, the last row is not counted
        if string.as_bytes()[string.len() - 1] != b'\n' {
            height += 1;
        }

        Ok(matrix::Matrix::from_vec(height, width, store))
    }

    pub fn from_string_chars(string: &str, map: impl Fn(char) -> T) -> Result<Self, StringError> {
        Self::try_from_string_chars(string, |c| Ok(map(c)))
    }
}
