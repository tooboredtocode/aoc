use aoc_lib::{SolutionPart1, SolutionPart2};
use crate::prelude::*;
use aoc_utils::matrix::{Direction as MatrixDirection, Matrix};

create_solution!(6);

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

impl SolutionPart1 for PuzzleSolution {
    type Input = PuzzleInput;
    type Result = String;

    fn solve(input: Self::Input) -> Result<Self::Result> {
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
                bail!("Guard is stuck in a loop");
            } else {
                v.set(guard.direction, true);
            }

            steps += 1;
        }
        if let Err(GuardMoveError::Stuck) = guard.move_with_grid(&input.obstacles) {
            bail!("Guard is stuck");
        }

        let visited = visited.entry_iter()
            .filter(|entry| entry.get().iter().any(|&visited| visited))
            .count();

        Ok(format!("Guard took {} steps and visited {} cells", steps, visited))
    }
}

impl SolutionPart2 for PuzzleSolution {
    type Input = PuzzleInput;
    type Result = String;

    fn solve(input: Self::Input) -> Result<Self::Result> {
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
                bail!("Guard is stuck in a loop");
            } else {
                v.set(guard.direction, true);
            }
        }
        if let Err(GuardMoveError::Stuck) = guard.move_with_grid(&input.obstacles) {
            bail!("Guard is stuck");
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

        Ok(format!("Found {} cells where adding an obstacle would cause the guard to loop", result))
    }
}

impl aoc_lib::PuzzleInput for PuzzleInput {
    fn from_input(input: &str) -> Result<Self> {
        let matrix = Matrix::from_string_chars(input.trim(), |c| c)
            .context("Failed to parse input as a matrix")?;

        let mut guard = None;
        let obstacles = matrix
            .map(|entry| {
                match entry.get() {
                    '#' => true,
                    '^' => {
                        guard = Some(Guard {
                            position: entry.position(),
                            direction: Direction::Up,
                        });
                        false
                    },
                    _ => false,
                }
            });

        let Some(guard) = guard else {
            bail!("No guard found in the input");
        };

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
    type Error = Anyhow;

    fn try_from(value: MatrixDirection) -> Result<Self, Self::Error> {
        match value {
            MatrixDirection::Up => Ok(Direction::Up),
            MatrixDirection::Down => Ok(Direction::Down),
            MatrixDirection::Left => Ok(Direction::Left),
            MatrixDirection::Right => Ok(Direction::Right),
            _ => bail!("Invalid direction"),
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
