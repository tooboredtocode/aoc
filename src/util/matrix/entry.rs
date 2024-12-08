use std::iter::successors;

use super::{Matrix, Distance, Direction};

#[derive(Debug)]
pub struct MatrixEntry<'a, T> {
    pub(super) matrix: &'a Matrix<T>,
    pub(super) value: &'a T,
    pub(super) x: usize,
    pub(super) y: usize,
}


impl<'a, T> MatrixEntry<'a, T> {
    /// Get the value at this matrix entry.
    pub fn get(&self) -> &'a T {
        self.value
    }

    /// Get the position of this matrix entry.
    pub fn position(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    /// Get the distance to another matrix entry.
    pub fn distance(&self, other: Self) -> Distance {
        let x = other.x as isize - self.x as isize;
        let y = other.y as isize - self.y as isize;
        Distance { x, y }
    }

    /// Get the matrix entry at a given distance if it exists.
    pub fn get_at_distance(&self, distance: Distance) -> Option<Self> {
        let x = self.x as isize + distance.x;
        let y = self.y as isize + distance.y;

        if let (Ok(x), Ok(y)) = (x.try_into(), y.try_into()) {
            self.matrix.get_entry(x, y)
        } else {
            None
        }
    }

    /// Get the matrix entry adjacent to this one in a given direction if it exists.
    pub fn adjacent(&self, direction: Direction) -> Option<Self> {
        let x = match direction {
            Direction::Up | Direction::Down => self.x,
            Direction::UpRight | Direction::Right | Direction::DownRight => self.x + 1,
            Direction::DownLeft | Direction::Left | Direction::UpLeft => self.x.overflowing_sub(1).0,
        };

        let y = match direction {
            Direction::Left | Direction::Right => self.y,
            Direction::UpLeft | Direction::Up | Direction::UpRight => self.y.overflowing_sub(1).0,
            Direction::DownLeft | Direction::Down | Direction::DownRight => self.y + 1,
        };

        self.matrix.get_entry(x, y)
    }

    /// Get an iterator over the matrix entries adjacent to this one in a given direction.
    pub fn adjacent_iter(&self, direction: Direction) -> impl Iterator<Item=Self> + '_ {
        let initial = Some(*self);
        successors(initial, move |entry| entry.adjacent(direction)).skip(1)
    }
}

impl<T> Clone for MatrixEntry<'_, T> {
    fn clone(&self) -> Self {
        *self
    }
}

/// This is a simple reference to a matrix entry, so it's safe to copy.
impl<T> Copy for MatrixEntry<'_, T> {}
