use crate::util::matrix;
use crate::util::matrix::Direction;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Yield {
    /// Continue yielding entries.
    Continue,
    /// Stop yielding entries, we have reached a result.
    Stop,
    /// Stop yielding entries, we have reached a dead end.
    Cancel,
}

pub struct YieldIter<'a, T, P, K: sealed::YieldIterKind> {
    stack: Vec<matrix::MatrixEntry<'a, T>>,
    direction: Vec<Direction>,
    predicate: P,
    allow_diagonal: bool,
    _kind: std::marker::PhantomData<K>,
}

pub struct YieldIterLast;
pub struct YieldIterFull;

impl sealed::YieldIterKind for YieldIterLast {
    type Res<'a, T: 'a> = matrix::MatrixEntry<'a, T>;

    fn make_res<'a, T, P>(_iter: &YieldIter<'a, T, P, Self>, next: &matrix::MatrixEntry<'a, T>) -> Self::Res<'a, T> {
        *next
    }
}

impl sealed::YieldIterKind for YieldIterFull {
    type Res<'a, T: 'a> = Vec<matrix::MatrixEntry<'a, T>>;

    fn make_res<'a, T, P>(iter: &YieldIter<'a, T, P, Self>, next: &matrix::MatrixEntry<'a, T>) -> Self::Res<'a, T> {
        let mut res = Vec::with_capacity(iter.stack.len() + 1);
        res.extend_from_slice(&iter.stack);
        res.push(*next);
        res
    }
}

impl<'a, T> matrix::MatrixEntry<'a, T> {
    pub fn try_yield_last<P>(&'a self, predicate: P, allow_diagonal: bool) -> YieldIter<'a, T, P, YieldIterLast>
    where
        P: Fn(&Self, &Self) -> Yield + 'a
    {
        YieldIter::new(*self, predicate, allow_diagonal)
    }

    pub fn try_yield_full<P>(&'a self, predicate: P, allow_diagonal: bool) -> YieldIter<'a, T, P, YieldIterFull>
    where
        P: Fn(&Self, &Self) -> Yield + 'a
    {
        YieldIter::new(*self, predicate, allow_diagonal)
    }
}

impl<'a, T, P, K: sealed::YieldIterKind> YieldIter<'a, T, P, K> {
    fn new(entry: matrix::MatrixEntry<'a, T>, predicate: P, allow_diagonal: bool) -> Self {
        Self {
            stack: vec![entry],
            direction: vec![Direction::Up],
            predicate,
            allow_diagonal,
            _kind: std::marker::PhantomData,
        }
    }

    fn next_direction(&self) -> Option<Direction> {
        let curr = self.direction.last()?;
        if self.allow_diagonal {
            match curr {
                Direction::Up => Some(Direction::UpRight),
                Direction::UpRight => Some(Direction::Right),
                Direction::Right => Some(Direction::DownRight),
                Direction::DownRight => Some(Direction::Down),
                Direction::Down => Some(Direction::DownLeft),
                Direction::DownLeft => Some(Direction::Left),
                Direction::Left => Some(Direction::UpLeft),
                Direction::UpLeft => None,
            }
        } else {
            match curr {
                Direction::Up => Some(Direction::Right),
                Direction::Right => Some(Direction::Down),
                Direction::Down => Some(Direction::Left),
                Direction::Left => None,
                _ => unreachable!(),
            }
        }
    }

    /// Advance the iterator to the next entry/direction.
    fn advance(&mut self) {
        // Loop until we find a valid next direction/entry, or we run out of stack.
        loop {
            if let Some(next_dir) = self.next_direction() {
                self.direction.last_mut()
                    .map(|d| *d = next_dir);

                break;
            } else {
                self.stack.pop();
                self.direction.pop();

                if self.stack.is_empty() {
                    break;
                }
            }
        }
    }
}

impl<'a, T, P, K> Iterator for YieldIter<'a, T, P, K>
where
    P: Fn(&matrix::MatrixEntry<'a, T>, &matrix::MatrixEntry<'a, T>) -> Yield + 'a,
    K: sealed::YieldIterKind,
{
    type Item = K::Res<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (Some(top), Some(dir)) = (self.stack.last(), self.direction.last()) else {
                return None;
            };

            if let Some(next) = top.adjacent(*dir) {
                match (self.predicate)(top, &next) {
                    Yield::Continue => {
                        self.stack.push(next);
                        self.direction.push(Direction::Up);
                    }
                    Yield::Stop => {
                        let res = K::make_res(self, &next);
                        self.advance();
                        return Some(res);
                    }
                    Yield::Cancel => {
                        self.advance();
                    }
                }
            } else {
                self.advance();
            }
        }
    }
}

mod sealed {
    use crate::util::matrix;
    use crate::util::matrix::ext::YieldIter;

    pub trait YieldIterKind: Sized {
        type Res<'a, T: 'a>;

        fn make_res<'a, T, P>(iter: &YieldIter<'a, T, P, Self>, next: &matrix::MatrixEntry<'a, T>) -> Self::Res<'a, T>;
    }
}
