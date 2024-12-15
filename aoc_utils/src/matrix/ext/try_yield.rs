use crate::matrix;
use crate::matrix::Direction;
use crate::matrix::ext::util::MatrixExtUtils;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Yield {
    /// Continue yielding entries.
    Continue,
    /// Stop yielding entries, we have reached a result.
    Stop,
    /// Stop yielding entries, we have reached a dead end.
    Cancel,
}

pub struct YieldIter<'a, T, P, K: sealed::YieldIterKind<'a, T>> {
    stack: Vec<matrix::MatrixEntry<'a, T>>,
    direction: Vec<Direction>,
    predicate: P,
    allow_diagonal: bool,
    kind: K
}

pub struct YieldIterLast;
pub struct YieldIterFull;
pub struct YieldIterCustom<F> {
    func: F,
}

impl<'a, T: 'a> sealed::YieldIterKind<'a, T> for YieldIterLast {
    type Res = matrix::MatrixEntry<'a, T>;

    fn make_res<P>(&self, _iter: &YieldIter<'a, T, P, Self>, next: &matrix::MatrixEntry<'a, T>) -> Self::Res {
        *next
    }
}

impl<'a, T: 'a> sealed::YieldIterKind<'a, T> for YieldIterFull {
    type Res = Vec<matrix::MatrixEntry<'a, T>>;

    fn make_res<P>(&self, iter: &YieldIter<'a, T, P, Self>, next: &matrix::MatrixEntry<'a, T>) -> Self::Res {
        let mut res = Vec::with_capacity(iter.stack.len() + 1);
        res.extend_from_slice(&iter.stack);
        res.push(*next);
        res
    }
}

impl<'a, T, F, R> sealed::YieldIterKind<'a, T> for YieldIterCustom<F>
where
    F: Fn(&[matrix::MatrixEntry<'a, T>], matrix::MatrixEntry<'a, T>) -> R,
    T: 'a
{
    type Res = R;

    fn make_res<P>(&self, iter: &YieldIter<'a, T, P, Self>, next: &matrix::MatrixEntry<'a, T>) -> Self::Res {
        (self.func)(&iter.stack, *next)
    }
}

impl<'a, T> matrix::MatrixEntry<'a, T> {
    /// Yields the last entry in the path that satisfies the predicate.
    pub fn try_yield_last<P>(&'a self, predicate: P, allow_diagonal: bool) -> YieldIter<'a, T, P, YieldIterLast>
    where
        P: FnMut(&Self, &Self) -> Yield + 'a
    {
        YieldIter::new(*self, predicate, allow_diagonal, YieldIterLast)
    }

    /// Yields the full path that satisfies the predicate.
    pub fn try_yield_full<P>(&'a self, predicate: P, allow_diagonal: bool) -> YieldIter<'a, T, P, YieldIterFull>
    where
        P: FnMut(&Self, &Self) -> Yield + 'a
    {
        YieldIter::new(*self, predicate, allow_diagonal, YieldIterFull)
    }

    /// Yields a custom result based on the path that satisfies the predicate.
    pub fn try_yield_custom<P, F, R>(&'a self, predicate: P, allow_diagonal: bool, func: F) -> YieldIter<'a, T, P, YieldIterCustom<F>>
    where
        P: FnMut(&Self, &Self) -> Yield + 'a,
        F: Fn(&[Self], Self) -> R
    {
        YieldIter::new(*self, predicate, allow_diagonal, YieldIterCustom { func })
    }
}

impl<'a, T, P, K: sealed::YieldIterKind<'a, T>> YieldIter<'a, T, P, K> {
    fn new(entry: matrix::MatrixEntry<'a, T>, predicate: P, allow_diagonal: bool, kind: K) -> Self {
        Self {
            stack: vec![entry],
            direction: vec![Direction::Up],
            predicate,
            allow_diagonal,
            kind
        }
    }
}

impl<'a, T, P, K> Iterator for YieldIter<'a, T, P, K>
where
    P: FnMut(&matrix::MatrixEntry<'a, T>, &matrix::MatrixEntry<'a, T>) -> Yield + 'a,
    K: sealed::YieldIterKind<'a, T>
{
    type Item = K::Res;

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
                        let res = self.kind.make_res(self, &next);
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

impl<'a, T, P, K: sealed::YieldIterKind<'a, T>> MatrixExtUtils<'a, T> for YieldIter<'a, T, P, K> {
    fn stack(&self) -> &Vec<matrix::MatrixEntry<'a, T>> {
        &self.stack
    }

    fn stack_mut(&mut self) -> &mut Vec<matrix::MatrixEntry<'a, T>> {
        &mut self.stack
    }

    fn direction(&self) -> &Vec<Direction> {
        &self.direction
    }

    fn direction_mut(&mut self) -> &mut Vec<Direction> {
        &mut self.direction
    }

    fn allow_diagonal(&self) -> bool {
        self.allow_diagonal
    }
}

mod sealed {
    use crate::matrix;
    use crate::matrix::ext::YieldIter;

    pub trait YieldIterKind<'a, T>: Sized {
        type Res;

        fn make_res<P>(&self, iter: &YieldIter<'a, T, P, Self>, next: &matrix::MatrixEntry<'a, T>) -> Self::Res;
    }
}
