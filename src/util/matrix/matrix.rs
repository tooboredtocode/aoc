use std::ops::{Index, IndexMut};
use crate::util::matrix::MatrixEntry;

#[derive(Debug)]
pub struct Matrix<T> {
    pub(super) data: Vec<T>,
    pub(super) width: usize,
    pub(super) height: usize,
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
        match self.get(x, y) {
            Some(value) => Some(MatrixEntry { matrix: self, value, x, y }),
            None => None,
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
    ///
    /// Note: The iterator will always yield the entries in row-major order.
    pub fn entry_iter(&self) -> impl Iterator<Item=MatrixEntry<T>> {
        (0..self.height).flat_map(move |y|
            (0..self.width).map(move |x| {
                // Note: the index is safe because we know the bounds are correct
                let value = &self.data[y * self.width + x];
                MatrixEntry { matrix: self, value, x, y }
            })
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

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        self.get(x, y).expect("Index out of bounds")
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        self.get_mut(x, y).expect("Index out of bounds")
    }
}
