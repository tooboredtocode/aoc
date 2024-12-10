pub trait Puzzle: Send + Sync + 'static {
    const YEAR: u16;
    const DAY: u8;
}
