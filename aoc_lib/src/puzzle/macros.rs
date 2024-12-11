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

#[macro_export]
macro_rules! create_puzzle_result {
    ($t:ident, $f:literal $(, $p:ident $(: $pty:ty)? )*) => {
        #[derive(Debug)]
        pub struct $t {
            $( $($p: $pty)? ),*
        }

        $crate::impl_puzzle_result!($t, $f $(, $p)*);
    };
}
