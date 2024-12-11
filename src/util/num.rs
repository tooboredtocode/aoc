pub trait NumUtils: Sized {
    fn count_digits(self) -> Self;
    fn split_digits_at(self, at: usize) -> (Self, Self);
}

macro_rules! impl_num_utils_unsigned {
    ($($t:ty),*) => {
        $(
            impl NumUtils for $t {
                fn count_digits(self) -> Self {
                    let mut n = self;
                    let mut count = 0;
                    while n > 0 {
                        n /= 10;
                        count += 1;
                    }
                    count
                }

                fn split_digits_at(self, at: usize) -> (Self, Self) {
                    let mut n = self;
                    let mut count = 0;
                    let mut left = 0;
                    let mut right = 0;
                    while n > 0 {
                        let digit = n % 10;
                        n /= 10;
                        count += 1;
                        if count <= at {
                            right += digit * 10u32.pow(count as u32 - 1) as Self;
                        } else {
                            left += digit * 10u32.pow((count - at) as u32 - 1) as Self;
                        }
                    }
                    (left, right)
                }
            }
        )*
    };
}

macro_rules! impl_num_utils_signed {
    ($($t:ty | $tu:ty ),*) => {
        $(
            impl NumUtils for $t {
                fn count_digits(self) -> Self {
                    (self.abs() as $tu).count_digits() as Self
                }

                fn split_digits_at(self, at: usize) -> (Self, Self) {
                    let sign = self.signum();
                    let value = self.abs() as $tu;
                    let (left, right) = value.split_digits_at(at);
                    (left as Self * sign, right as Self)
                }
            }
        )*
    };
}

impl_num_utils_unsigned!(u8, u16, u32, u64, u128, usize);
impl_num_utils_signed!(i8 | u8, i16 | u16, i32 | u32, i64 | u64, i128 | u128, isize | usize);
