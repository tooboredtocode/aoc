use crate::core::{Input, Puzzle, PuzzlePart2};
use crate::core::aoc_client::AocClientError;
use crate::util::matrix::{Direction as MatrixDirection, Matrix};
use crate::util::StringError;

/// https://adventofcode.com/2024/day/6
pub struct PuzzleSolution;

#[derive(Debug)]
pub struct PuzzleInput {
    obstacles: Matrix<bool>,
    initial_guard: Guard,
}

#[derive(Debug, Clone)]
pub struct Guard {
    position: (usize, usize),
    direction: Direction,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// A map of directions to values
#[derive(Debug, Clone)]
struct DirMap<T> {
    up: T,
    down: T,
    left: T,
    right: T,
}

#[derive(Debug)]
enum GuardMoveError {
    /// Error for when the guard tries to move out of bounds
    OutOfBounds,
    /// Error for when the guard is stuck and has no more moves
    Stuck,
}

impl Puzzle for PuzzleSolution {
    type Input = PuzzleInput;
    type FetchError = AocClientError<PuzzleInput>;
    type SolveError = StringError;

    async fn fetch_input(client: &crate::core::AocClient) -> Result<Self::Input, Self::FetchError> {
        client.get_challenge(2024, 6).await
    }

    async fn solve(input: Self::Input) -> Result<(), Self::SolveError> {
        let mut guard = input.initial_guard.clone();
        let mut visited = Matrix::new_with(input.obstacles.height(), input.obstacles.width(), |_, _| DirMap::new(false));

        visited.get_mut(guard.position.0, guard.position.1)
            .expect("Initial guard position should be in bounds")
            .set(guard.direction, true);

        let mut steps = 0usize;
        // Move the guard until it can't move any more
        while let Ok(_) = guard.move_with_grid(&input.obstacles) {
            let v = visited.get_mut(guard.position.0, guard.position.1)
                .expect("Guard position should be in bounds since it was successfully moved");

            if v.get(guard.direction).eq(&true) {
                // Guard has visited this cell before
                return Err(StringError::new("Guard is stuck in a loop"));
            } else {
                v.set(guard.direction, true);
            }

            steps += 1;
        }
        if let Err(GuardMoveError::Stuck) = guard.move_with_grid(&input.obstacles) {
            return Err(StringError::new("Guard is stuck"));
        }

        let visited = visited.entry_iter()
            .filter(|entry| entry.get().iter().any(|&visited| visited))
            .count();

        println!("Guard took {} steps and visited {} cells", steps, visited);

        Ok(())
    }
}

impl PuzzlePart2 for PuzzleSolution {
    async fn solve_part2(input: Self::Input) -> Result<(), Self::SolveError> {
        let mut guard = input.initial_guard.clone();
        let mut initial_visited = Matrix::new_with(input.obstacles.height(), input.obstacles.width(), |_, _| DirMap::new(false));

        initial_visited.get_mut(guard.position.0, guard.position.1)
            .expect("Initial guard position should be in bounds")
            .set(guard.direction, true);

        // Create the initial visited map to reduce the number of checks
        while let Ok(_) = guard.move_with_grid(&input.obstacles) {
            let v = initial_visited.get_mut(guard.position.0, guard.position.1)
                .expect("Guard position should be in bounds since it was successfully moved");

            if v.get(guard.direction).eq(&true) {
                // Guard has visited this cell before
                return Err(StringError::new("Guard is stuck in a loop"));
            } else {
                v.set(guard.direction, true);
            }
        }
        if let Err(GuardMoveError::Stuck) = guard.move_with_grid(&input.obstacles) {
            return Err(StringError::new("Guard is stuck"));
        }

        // Brute force the solution by adding obstacles and checking if the guard gets stuck
        let result = (0..input.obstacles.height())
            .flat_map(|y| (0..input.obstacles.width()).map(move |x| (x, y)))
            .filter(|&(x, y)| {
                if input.obstacles.get(x, y) == Some(&true) {
                    return false; // Skip obstacles since they are already there
                }

                if !initial_visited.get(x, y).map_or(false, |v| v.iter().any(|&visited| visited)) {
                    return false; // Adding an obstacle to a cell that the guard won't visit will have no effect
                }

                let mut obstacles = input.obstacles.clone();
                obstacles.set(x, y, true);

                let mut guard = input.initial_guard.clone();
                let mut visited = Matrix::new_with(input.obstacles.height(), input.obstacles.width(), |_, _| DirMap::new(false));

                visited.get_mut(guard.position.0, guard.position.1)
                    .expect("Initial guard position should be in bounds")
                    .set(guard.direction, true);

                while let Ok(_) = guard.move_with_grid(&obstacles) {
                    let v = visited.get_mut(guard.position.0, guard.position.1)
                        .expect("Guard position should be in bounds since it was successfully moved");

                    if v.get(guard.direction).eq(&true) {
                        return true;
                    } else {
                        v.set(guard.direction, true);
                    }
                }

                false
            }).count();

        println!("Found {} cells where adding an obstacle would cause the guard to loop", result);

        Ok(())
    }
}

impl Input for PuzzleInput {
    type ParseError = StringError;

    async fn from_input(input: String) -> Result<Self, Self::ParseError> {
        let line_length = input.lines().next().map_or(0, |line| line.chars().count());
        if line_length == 0 {
            return Err(StringError::new("No lines in the input"));
        }
        if input.lines().any(|line| line.chars().count() != line_length) {
            return Err(StringError::new("Lines in the input have different lengths"));
        }
        let row_height = input.lines().count();
        if row_height == 0 {
            return Err(StringError::new("No rows in the input"));
        }

        let mut obstacles = Matrix::new(row_height, line_length, false);
        let mut guard = None;

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => _ = obstacles.set(x, y, true),
                    '^' => guard = Some(Guard {
                        position: (x, y),
                        direction: Direction::Up,
                    }),
                    // There are no other directions possible
                    _ => {}
                }
            }
        }

        let guard = guard.ok_or_else(|| StringError::new("No guard found"))?;

        Ok(PuzzleInput { obstacles, initial_guard: guard })
    }
}

impl Guard {
    fn move_with_grid(&mut self, grid: &Matrix<bool>) -> Result<bool, GuardMoveError> {
        let (x, y) = self.position;
        let curr_pos = grid.get_entry(x, y)
            .ok_or(GuardMoveError::OutOfBounds)?;

        let mut turned = false;
        for _ in 0..4 {
            let next_pos = curr_pos
                .adjacent(self.direction.into())
                .ok_or(GuardMoveError::OutOfBounds)?;

            // Check if the next position is free
            if next_pos.get().eq(&false) {
                // Position is free, move the guard
                self.position = next_pos.position();
                return Ok(turned);
            }

            // Position is not free, turn right
            self.direction = self.direction.turn_right();
            turned = true;
        }

        Err(GuardMoveError::Stuck)
    }
}

impl Direction {
    fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

impl TryFrom<MatrixDirection> for Direction {
    type Error = StringError;

    fn try_from(value: MatrixDirection) -> Result<Self, Self::Error> {
        match value {
            MatrixDirection::Up => Ok(Direction::Up),
            MatrixDirection::Down => Ok(Direction::Down),
            MatrixDirection::Left => Ok(Direction::Left),
            MatrixDirection::Right => Ok(Direction::Right),
            _ => Err(StringError::new("Invalid direction")),
        }
    }
}

impl From<Direction> for MatrixDirection {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => MatrixDirection::Up,
            Direction::Down => MatrixDirection::Down,
            Direction::Left => MatrixDirection::Left,
            Direction::Right => MatrixDirection::Right,
        }
    }
}

impl<T> DirMap<T> {
    fn new(value: T) -> Self
    where
        T: Clone,
    {
        DirMap {
            up: value.clone(),
            down: value.clone(),
            left: value.clone(),
            right: value,
        }
    }

    fn get(&self, direction: Direction) -> &T {
        match direction {
            Direction::Up => &self.up,
            Direction::Down => &self.down,
            Direction::Left => &self.left,
            Direction::Right => &self.right,
        }
    }

    fn set(&mut self, direction: Direction, value: T) {
        match direction {
            Direction::Up => self.up = value,
            Direction::Down => self.down = value,
            Direction::Left => self.left = value,
            Direction::Right => self.right = value,
        }
    }

    fn iter(&self) -> impl Iterator<Item = &T> {
        vec![&self.up, &self.down, &self.left, &self.right].into_iter()
    }
}
