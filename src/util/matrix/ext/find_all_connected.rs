use rustc_hash::FxHashSet;
use crate::util::matrix::ext::util::MatrixExtUtils;
use crate::util::matrix::{Direction, MatrixEntry};

pub struct FindAllConnectedIter<'a, T, P>
where
    P: FnMut(&MatrixEntry<T>, &MatrixEntry<T>) -> bool + 'a,
    T: 'a
{
    stack: Vec<MatrixEntry<'a, T>>,
    direction: Vec<Direction>,
    visited: FxHashSet<(usize, usize)>,
    predicate: P,
    allow_diagonal: bool,
    yielded_start: bool,
}

impl<'a, T> MatrixEntry<'a, T> {
    /// Yields all connected entries that satisfy the predicate.
    pub fn find_all_connected<P>(&'a self, predicate: P, allow_diagonal: bool) -> FindAllConnectedIter<'a, T, P>
    where
        P: FnMut(&MatrixEntry<T>, &MatrixEntry<T>) -> bool + 'a
    {
        FindAllConnectedIter::new(*self, predicate, allow_diagonal)
    }
}

impl<'a, T, P> FindAllConnectedIter<'a, T, P>
where
    P: FnMut(&MatrixEntry<T>, &MatrixEntry<T>) -> bool + 'a,
    T: 'a
{
    fn new(start: MatrixEntry<'a, T>, predicate: P, allow_diagonal: bool) -> Self {
        Self {
            stack: vec![start],
            direction: vec![Direction::Up],
            visited: FxHashSet::default(),
            predicate,
            allow_diagonal,
            yielded_start: false,
        }
    }
}

impl<'a, T, P> Iterator for FindAllConnectedIter<'a, T, P>
where
    P: FnMut(&MatrixEntry<T>, &MatrixEntry<T>) -> bool + 'a,
    T: 'a
{
    type Item = MatrixEntry<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.yielded_start {
            // Yield the starting entry once.
            self.yielded_start = true;
            self.visited.insert(self.stack[0].position());
            return Some(self.stack[0]);
        }

        loop {
            let (Some(top), Some(dir)) = (self.stack.last(), self.direction.last()) else {
                return None;
            };

            let Some(next) = top.adjacent(*dir) else {
                self.advance();
                continue;
            };

            if self.visited.contains(&next.position()) {
                self.advance();
                continue;
            } else {
                self.visited.insert(next.position());
            }

            if !(self.predicate)(top, &next) {
                self.advance();
                continue;
            }

            self.stack.push(next);
            self.direction.push(Direction::Up);
            return Some(next);
        }
    }
}

impl<'a, T, P> MatrixExtUtils<'a, T> for FindAllConnectedIter<'a, T, P>
where
    P: FnMut(&MatrixEntry<T>, &MatrixEntry<T>) -> bool + 'a,
    T: 'a
{
    fn stack(&self) -> &Vec<MatrixEntry<'a, T>> {
        &self.stack
    }

    fn stack_mut(&mut self) -> &mut Vec<MatrixEntry<'a, T>> {
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
