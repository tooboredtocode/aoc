use crate::matrix;
use itertools::Itertools;
use std::convert::Infallible;
use std::{error, fmt};

#[derive(Debug)]
pub enum TryParseStringError<E> {
    ParseError(ParseStringError),
    MapError(E),
}

#[derive(Debug)]
pub enum ParseStringError {
    UnevenRows,
    NoRows,
    NoColumns,
}

impl<T> matrix::Matrix<T> {
    pub fn try_from_string_chars<F, E>(string: &str, map: F) -> Result<Self, TryParseStringError<E>>
    where
        F: Fn(char) -> Result<T, E>,
    {
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
                        return Some(Err(ParseStringError::UnevenRows.into()));
                    }
                    width = 0;
                    None
                } else {
                    width += 1;
                    Some(map(c).map_err(TryParseStringError::MapError))
                }
            })
            .try_collect()?;

        if height == 0 {
            return Err(ParseStringError::NoRows.into());
        } else if width == 0 {
            return Err(ParseStringError::NoColumns.into());
        }

        // If the last character is not a newline, the last row is not counted
        if string.as_bytes()[string.len() - 1] != b'\n' {
            height += 1;
        }

        Ok(matrix::Matrix::from_vec(height, width, store))
    }

    pub fn from_string_chars(string: &str, map: impl Fn(char) -> T) -> Result<Self, ParseStringError> {
        match Self::try_from_string_chars(string, |c| Ok(map(c))) {
            Ok(matrix) => Ok(matrix),
            Err(e) => {
                let TryParseStringError::<Infallible>::ParseError(e) = e;
                Err(e)
            }
        }
    }
}

impl<E> From<ParseStringError> for TryParseStringError<E> {
    fn from(e: ParseStringError) -> Self {
        TryParseStringError::ParseError(e)
    }
}

impl fmt::Display for ParseStringError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseStringError::UnevenRows => write!(f, "Matrix has rows of uneven width"),
            ParseStringError::NoRows => write!(f, "Matrix has no rows"),
            ParseStringError::NoColumns => write!(f, "Matrix has no columns"),
        }
    }
}

impl<E: fmt::Display> fmt::Display for TryParseStringError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TryParseStringError::ParseError(e) => e.fmt(f),
            TryParseStringError::MapError(_) => write!(f, "The mapping function failed"),
        }
    }
}

impl error::Error for ParseStringError {}

impl<E: error::Error + 'static> error::Error for TryParseStringError<E> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            TryParseStringError::ParseError(_) => None,
            TryParseStringError::MapError(e) => Some(e),
        }
    }
}
