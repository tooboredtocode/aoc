use crate::matrix;
use crate::matrix::Direction;

/// Extension trait for matrix iterators.
///
/// This trait provides utility methods for matrix iterators.
pub(super) trait MatrixExtUtils<'a, T: 'a> {
    fn stack(&self) -> &Vec<matrix::MatrixEntry<'a, T>>;
    fn stack_mut(&mut self) -> &mut Vec<matrix::MatrixEntry<'a, T>>;
    fn direction(&self) -> &Vec<Direction>;
    fn direction_mut(&mut self) -> &mut Vec<Direction>;

    fn allow_diagonal(&self) -> bool;

    fn next_direction(&mut self) -> Option<Direction> {
        let curr = self.direction().last()?;
        if self.allow_diagonal() {
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
                self.direction_mut().last_mut()
                    .map(|d| *d = next_dir);

                break;
            } else {
                self.stack_mut().pop();
                self.direction_mut().pop();

                if self.stack().is_empty() {
                    break;
                }
            }
        }
    }
}