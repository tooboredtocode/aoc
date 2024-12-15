#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

impl Direction {
    pub fn rotate_right(self, allow_diagonal: bool) -> Self {
        if allow_diagonal {
            match self {
                Direction::Up => Direction::UpRight,
                Direction::UpRight => Direction::Right,
                Direction::Right => Direction::DownRight,
                Direction::DownRight => Direction::Down,
                Direction::Down => Direction::DownLeft,
                Direction::DownLeft => Direction::Left,
                Direction::Left => Direction::UpLeft,
                Direction::UpLeft => Direction::Up
            }
        } else {
            match self {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
                _ => panic!("Cannot rotate diagonal direction right when allow_diagonal is false"),
            }
        }
    }

    pub fn iter(allow_diagonal: bool) -> impl Iterator<Item=Direction> {
        if allow_diagonal {
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
        } else {
            [
                Direction::Up,
                Direction::Right,
                Direction::Down,
                Direction::Left,
            ].iter().copied()
        }
    }

    pub fn iter_all() -> impl Iterator<Item=Direction> {
        Self::iter(true)
    }
}
