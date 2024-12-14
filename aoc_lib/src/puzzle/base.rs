pub trait Puzzle: Send + Sync + 'static {
    const YEAR: u16;
    const DAY: u8;
    /// An alternative solution to the puzzle.
    const ALT: Option<&'static str> = None;
}
