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

#[macro_export]
macro_rules! impl_puzzle_result {
    ($t:ty, $f:literal $(, $p:ident)*) => {
        impl $crate::puzzle::PuzzleResult for $t {
            fn display(&self) {
                println!($f, $(self.$p),*);
            }
        }
    };
}
