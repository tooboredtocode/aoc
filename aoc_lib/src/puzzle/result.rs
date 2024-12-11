pub trait PuzzleResult {
    /// Display the result.
    fn display(&self);
}

impl PuzzleResult for () {
    fn display(&self) {
        println!("No result to display");
    }
}

impl PuzzleResult for String {
    fn display(&self) {
        println!("{}", self);
    }
}
