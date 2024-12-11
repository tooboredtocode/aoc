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

impl Direction {
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
