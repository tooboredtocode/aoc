use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Distance {
    pub(super) x: isize,
    pub(super) y: isize,
}

impl Distance {
    /// Get the x component of the distance.
    pub fn x(&self) -> isize {
        self.x
    }

    /// Get the y component of the distance.
    pub fn y(&self) -> isize {
        self.y
    }

    /// Calculate the Euclidean distance.
    pub fn euclidean(&self) -> f64 {
        ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt()
    }
}

impl Add for Distance {
    type Output = Distance;

    fn add(self, other: Distance) -> Distance {
        Distance {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Distance {
    fn add_assign(&mut self, other: Distance) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Sub for Distance {
    type Output = Distance;

    fn sub(self, other: Distance) -> Distance {
        Distance {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl SubAssign for Distance {
    fn sub_assign(&mut self, other: Distance) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

macro_rules! impl_integers {
    ($($t:ty),*) => {
        $(
            impl Mul<$t> for Distance {
                type Output = Distance;

                fn mul(self, rhs: $t) -> Distance {
                    Distance {
                        x: self.x * rhs as isize,
                        y: self.y * rhs as isize,
                    }
                }
            }

            impl MulAssign<$t> for Distance {
                fn mul_assign(&mut self, rhs: $t) {
                    self.x *= rhs as isize;
                    self.y *= rhs as isize;
                }
            }

            impl Mul<Distance> for $t {
                type Output = Distance;

                fn mul(self, rhs: Distance) -> Distance {
                    Distance {
                        x: self as isize * rhs.x,
                        y: self as isize * rhs.y,
                    }
                }
            }
        )*
    };
}

impl_integers!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
