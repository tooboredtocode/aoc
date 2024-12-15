use anyhow::Result;

pub trait PuzzleInput: Sized {
    /// Indicates if the input prefers to be parsed from an owned string.
    const PREFERS_OWNED_INPUT: bool = false;

    /// Parse the input from a string.
    fn from_input(input: &str) -> Result<Self>;

    /// Parse the input from an owned string.
    ///
    /// This is implemented by default, but can be overwritten if the parsing is more efficient
    /// when done on an owned string.
    ///
    /// # Note
    /// To indicate that the input is more performant to parse from an owned string, you can
    /// override the `PREFERS_OWNED_INPUT` constant to `true`.
    fn from_input_owned(input: String) -> Result<Self> {
        Self::from_input(&input)
    }
}
