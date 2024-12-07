#![allow(dead_code)]

use std::iter::successors;

#[derive(Debug)]
pub struct Matrix<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

#[derive(Debug)]
pub struct MatrixEntry<'a, T> {
    matrix: &'a Matrix<T>,
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl<T> Matrix<T> {
    /// Create a new matrix with the given dimensions and default value.
    pub fn new(width: usize, height: usize, default: T) -> Self
    where
        T: Clone,
    {
        Self {
            data: vec![default; width * height],
            width,
            height,
        }
    }

    /// Create a new matrix with the given dimensions and a function to generate the values.
    pub fn new_with(width: usize, height: usize, f: impl Fn(usize, usize) -> T) -> Self {
        // Create a reference to the function that we can move into the closure
        let f_ref = &f;

        let data = (0..height).flat_map(|y|
            (0..width).map(move |x|
                f_ref(x, y)
            )
        ).collect();
        Self {
            data,
            width,
            height,
        }
    }

    /// Create a new matrix from a vector of data.
    ///
    /// # Panics
    ///
    /// Panics if the width times the height is not equal to the length of the data.
    pub fn from_vec(width: usize, height: usize, data: Vec<T>) -> Self {
        assert_eq!(width * height, data.len());
        Self {
            data,
            width,
            height,
        }
    }

    /// Get the value at the given position.
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x < self.width && y < self.height {
            Some(&self.data[y * self.width + x])
        } else {
            None
        }
    }

    /// Get the value at the given position.
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        if x < self.width && y < self.height {
            Some(&mut self.data[y * self.width + x])
        } else {
            None
        }
    }

    /// Get a reference to the entry at the given position.
    pub fn get_entry(&self, x: usize, y: usize) -> Option<MatrixEntry<T>> {
        if x < self.width && y < self.height {
            Some(MatrixEntry { matrix: self, x, y })
        } else {
            None
        }
    }

    /// Set the value at the given position.
    pub fn set(&mut self, x: usize, y: usize, value: T) -> Option<()> {
        if x < self.width && y < self.height {
            self.data[y * self.width + x] = value;
            Some(())
        } else {
            None
        }
    }

    /// Get the width of the matrix.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Get the height of the matrix.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Get an iterator over all entries in the matrix.
    pub fn entry_iter(&self) -> impl Iterator<Item=MatrixEntry<T>> {
        (0..self.height).flat_map(move |y|
            (0..self.width).map(move |x|
                MatrixEntry { matrix: self, x, y }
            )
        )
    }

    /// Map the matrix to a new matrix with a different type.
    ///
    /// Note: The matrix dimensions will remain the same.
    pub fn map<U>(&self, mut f: impl FnMut(&T) -> U) -> Matrix<U> {
        let data = self.data.iter().map(|item| f(item)).collect();
        Matrix::from_vec(self.width, self.height, data)
    }
}

impl<T> Clone for Matrix<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            width: self.width,
            height: self.height,
        }
    }
}

impl<'a, T> MatrixEntry<'a, T> {
    /// Get the value at this matrix entry.
    pub fn get(&self) -> &'a T {
        self.matrix.get(self.x, self.y)
            .expect("Matrix entry should be in bounds")
    }

    /// Get the position of this matrix entry.
    pub fn position(&self) -> (usize, usize) {
        (self.x, self.y)
    }

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

        if x < self.matrix.width && y < self.matrix.height {
            Some(MatrixEntry { matrix: self.matrix, x, y })
        } else {
            None
        }
    }

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

impl Direction {
    pub fn iter() -> impl Iterator<Item=Direction> {
        [
            Direction::Up,
            Direction::UpRight,
            Direction::Right,
            Direction::DownRight,
            Direction::Down,
            Direction::DownLeft,
            Direction::Left,
            Direction::UpLeft,
        ].iter().copied()
    }
}
