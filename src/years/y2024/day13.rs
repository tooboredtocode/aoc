use crate::util::StringError;
use aoc_lib::create_puzzle_result;
use itertools::{Either, Itertools};
use nalgebra::{Matrix2, Vector2};

create_solution!(13);
create_alt_solution!(13, MultipleSolutions, "Handle Multiple Solutions");
create_alt_solution!(13, NoMatrix, "No Matrix Equations");

#[derive(Debug)]
pub struct PuzzleInput {
    arcade_games: Vec<ArcadeGame>,
}

#[derive(Debug, Copy, Clone)]
pub struct ArcadeGame {
    button_a: Button,
    button_b: Button,
    prize_location: Vector2<u64>,
}

#[derive(Debug, Copy, Clone)]
pub struct Button {
    distance_moved: Vector2<u64>,
}

create_puzzle_result!(PuzzleResult, "Found {} solvable game(s), which cost at least {} tokens in total to win the prize.", solvable: u64, tokens: u64);

create_solution_part1!((input: PuzzleInput) -> PuzzleResult {
    let mut solvable = 0;

    let tokens: u64 = input.arcade_games.iter()
        .filter_map(|game| game.find_solution())
        .inspect(|_| solvable += 1)
        .map(|(a, b)| a * 3 + b)
        .sum();

    Ok(PuzzleResult { solvable, tokens })
});

create_alt_solution_part1!(MultipleSolutions, (input: PuzzleInput) -> PuzzleResult {
    let mut solvable = 0;

    let tokens: u64 = input.arcade_games.iter()
        .filter_map(|game| game.find_solution_alt())
        .inspect(|_| solvable += 1)
        .map(|solution| match solution {
            Either::Left((a, b)) => a * 3 + b,
            Either::Right(MultipleSol { min_a, .. }) => min_a.0 * 3 + min_a.1,
        })
        .sum();

    Ok(PuzzleResult { solvable, tokens })
});

create_alt_solution_part1!(NoMatrix, (input: PuzzleInput) -> PuzzleResult {
    let mut solvable = 0;

    let tokens: u64 = input.arcade_games.iter()
        .filter_map(|game| game.find_solution_no_matrix())
        .inspect(|_| solvable += 1)
        .map(|(a, b)| a * 3 + b)
        .sum();

    Ok(PuzzleResult { solvable, tokens })
});

const ADD_PART_2: Vector2<u64> = Vector2::new(10000000000000, 10000000000000);

create_solution_part2!((input: PuzzleInput) -> PuzzleResult {
    let mut solvable = 0;

    let tokens: u64 = input.arcade_games.iter()
        .map(|game| {
            let mut game = *game;
            game.prize_location += ADD_PART_2;
            game
        })
        .filter_map(|game| game.find_solution())
        .inspect(|_| solvable += 1)
        .map(|(a, b)| a * 3 + b)
        .sum();

    Ok(PuzzleResult { solvable, tokens })
});

create_alt_solution_part2!(MultipleSolutions, (input: PuzzleInput) -> PuzzleResult {
    let mut solvable = 0;

    let tokens: u64 = input.arcade_games.iter()
        .map(|game| {
            let mut game = *game;
            game.prize_location += ADD_PART_2;
            game
        })
        .filter_map(|game| game.find_solution_alt())
        .inspect(|_| solvable += 1)
        .map(|solution| match solution {
            Either::Left((a, b)) => a * 3 + b,
            Either::Right(MultipleSol { min_a, .. }) => min_a.0 * 3 + min_a.1,
        })
        .sum();

    Ok(PuzzleResult { solvable, tokens })
});

create_alt_solution_part2!(NoMatrix, (input: PuzzleInput) -> PuzzleResult {
    let mut solvable = 0;

    let tokens: u64 = input.arcade_games.iter()
        .map(|game| {
            let mut game = *game;
            game.prize_location += ADD_PART_2;
            game
        })
        .filter_map(|game| game.find_solution_no_matrix())
        .inspect(|_| solvable += 1)
        .map(|(a, b)| a * 3 + b)
        .sum();

    Ok(PuzzleResult { solvable, tokens })
});

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct MultipleSol {
    // The Solution with the least amount of moves for button A.
    min_a: (u64, u64),
    // The Solution with the least amount of moves for button B.
    min_b: (u64, u64),
}

impl ArcadeGame {
    #[allow(non_snake_case)]
    fn find_solution(&self) -> Option<(u64, u64)> {
        let A = Matrix2::from_columns(&[
            self.button_a.distance_moved.map(|x| x as f64),
            self.button_b.distance_moved.map(|x| x as f64)
        ]);
        let b = self.prize_location.map(|x| x as f64);

        let Some(A_inv) = A.try_inverse() else {
            return None;
        };

        let solution = A_inv * b;

        if solution.x < 0.0 || solution.y < 0.0 {
            // The solution includes negative moves, which is not allowed.
            return None;
        }

        let a_moves = solution.x.round() as u64;
        let b_moves = solution.y.round() as u64;

        // Check if the solution is also a valid integer solution.
        if self.is_valid_move(a_moves, b_moves) {
            Some((a_moves, b_moves))
        } else {
            None
        }
    }

    fn find_solution_no_matrix(&self) -> Option<(u64, u64)> {
        let a = self.button_a.distance_moved;
        let b = self.button_b.distance_moved;
        let res = self.prize_location;

        let det = a.x * b.y - a.y * b.x;
        if det == 0 {
            return None;
        }

        let res_a = (res.x * b.y - res.y * b.x) / det;
        let res_b = (a.x * res.y - a.y * res.x) / det;

        if self.is_valid_move(res_a, res_b) {
            Some((res_a, res_b))
        } else {
            None
        }
    }

    #[allow(non_snake_case)]
    fn find_solution_alt(&self) -> Option<Either<(u64, u64), MultipleSol>> {
        let A = Matrix2::from_columns(&[
            self.button_a.distance_moved.map(|x| x as f64),
            self.button_b.distance_moved.map(|x| x as f64)
        ]);
        let b = self.prize_location.map(|x| x as f64);

        let Some(A_inv) = A.try_inverse() else {
            use approx::RelativeEq;

            // The matrix is singular, so there is no unique solution.
            let a_float = self.button_a.distance_moved
                .map(|x| x as f64);

            if !b.angle(&a_float).relative_eq(&0.0, 1e-3, 1e-3) {
                // The prize location is not on the line between the two buttons.
                // -> There is no solution.
                return None;
            }

            let factor_a = self.button_a.distance_moved.x;
            let factor_b = self.button_b.distance_moved.x;
            let factor_result = self.prize_location.x;

            let max_a = factor_result / factor_a;
            let max_b = factor_result / factor_b;

            let min_a = (0..=max_a)
                .map(|a| {
                    let b = (factor_result - factor_a * a) / factor_b;
                    (a, b)
                })
                .find(|(a, b)| self.is_valid_move(*a, *b));

            let min_b = (0..=max_b)
                .map(|b| {
                    let a = (factor_result - factor_b * b) / factor_a;
                    (a, b)
                })
                .find(|(a, b)| self.is_valid_move(*a, *b));

            return match (min_a, min_b) {
                (Some(min_a), Some(min_b)) => {
                    Some(Either::Right(MultipleSol { min_a, min_b }))
                },
                (Some(min_a), None) => {
                    Some(Either::Left(min_a))
                },
                (None, Some(min_b)) => {
                    Some(Either::Left(min_b))
                },
                (None, None) => {
                    None
                },
            };
        };

        let solution = A_inv * b;

        if solution.x < 0.0 || solution.y < 0.0 {
            // The solution includes negative moves, which is not allowed.
            return None;
        }

        let a_moves = solution.x.round() as u64;
        let b_moves = solution.y.round() as u64;

        // Check if the solution is also a valid integer solution.
        if self.is_valid_move(a_moves, b_moves) {
            Some(Either::Left((a_moves, b_moves)))
        } else {
            None
        }
    }
}

impl ArcadeGame {
    /// Checks if a combination of moves is valid.
    fn is_valid_move(&self, a_moves: u64, b_moves: u64) -> bool {
        let position = self.calculate_position(a_moves, b_moves);
        position == self.prize_location
    }

    /// Calculates the position of the arm after the given number of moves.
    #[allow(non_snake_case)]
    fn calculate_position(&self, a_moves: u64, b_moves: u64) -> Vector2<u64> {
        let A = Matrix2::from_columns(&[
            self.button_a.distance_moved,
            self.button_b.distance_moved
        ]);

        let moves = Vector2::new(a_moves, b_moves);

        A * moves
    }
}

impl aoc_lib::PuzzleInput for PuzzleInput {
    type ParseError = StringError;

    fn from_input(input: &str) -> Result<Self, Self::ParseError> {
        fn parse_button(button: &str) -> Result<Button, StringError> {
            let button = button.strip_prefix("X+")
                .ok_or_else(|| StringError::new("Button not in expected format"))?;

            let (x, y) = button.split_once(", Y+")
                .ok_or_else(|| StringError::new("Button not in expected format"))?;

            let x = x.parse().map_err(|e| StringError::with_cause("Button X not a number: ", e))?;
            let y = y.parse().map_err(|e| StringError::with_cause("Button Y not a number: ", e))?;

            Ok(Button { distance_moved: Vector2::new(x, y) })
        }

        let arcade_games = input.trim().split("\n\n")
            .map(|game| {
                let mut lines = game.lines();
                let button_a = lines.next().ok_or_else(|| StringError::new("Button A missing"))?;
                let button_b = lines.next().ok_or_else(|| StringError::new("Button B missing"))?;
                let prize = lines.next().ok_or_else(|| StringError::new("Prize missing"))?;

                let button_a = match button_a.strip_prefix("Button A: ") {
                    Some(button_a) => parse_button(button_a)?,
                    None => return Err(StringError::new("Button A not in expected format")),
                };

                let button_b = match button_b.strip_prefix("Button B: ") {
                    Some(button_b) => parse_button(button_b)?,
                    None => return Err(StringError::new("Button B not in expected format")),
                };

                let prize = match prize.strip_prefix("Prize: X=") {
                    Some(prize) => {
                        let (x, y) = prize.split_once(", Y=")
                            .ok_or_else(|| StringError::new("Prize not in expected format"))?;

                        let x = x.parse()
                            .map_err(|e| StringError::with_cause("Prize X not a number: ", e))?;
                        let y = y.parse()
                            .map_err(|e| StringError::with_cause("Prize Y not a number: ", e))?;

                        Vector2::new(x, y)
                    },
                    None => return Err(StringError::new("Prize not in expected format")),
                };

                Ok(ArcadeGame { button_a, button_b, prize_location: prize })
            })
            .try_collect()?;

        Ok(Self { arcade_games })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_1() {
        let game = ArcadeGame {
            button_a: Button { distance_moved: Vector2::new(1, 0) },
            button_b: Button { distance_moved: Vector2::new(0, 1) },
            prize_location: Vector2::new(2, 2),
        };

        let expected = Some(Either::Left((2, 2)));
        let actual = game.find_solution_alt();
        assert_eq!(expected, actual, "Got unexpected solution");
    }

    #[test]
    fn test_solution_2() {
        let game = ArcadeGame {
            button_a: Button { distance_moved: Vector2::new(1, 0) },
            button_b: Button { distance_moved: Vector2::new(1, 0) },
            prize_location: Vector2::new(2, 2),
        };

        let expected = None;
        let actual = game.find_solution_alt();
        assert_eq!(expected, actual, "Got unexpected solution");
    }

    #[test]
    fn test_solution_3() {
        let game = ArcadeGame {
            button_a: Button { distance_moved: Vector2::new(1, 1) },
            button_b: Button { distance_moved: Vector2::new(2, 2) },
            prize_location: Vector2::new(10, 10),
        };

        let expected = Some(Either::Right(MultipleSol { min_a: (0, 5), min_b: (10, 0) }));
        let actual = game.find_solution_alt();
        assert_eq!(expected, actual, "Got unexpected solution");
    }

    #[test]
    fn test_solution_4() {
        let game = ArcadeGame {
            button_a: Button { distance_moved: Vector2::new(94, 34) },
            button_b: Button { distance_moved: Vector2::new(22, 67) },
            prize_location: Vector2::new(8400, 5400),
        };

        let expected = Some(Either::Left((80, 40)));
        let actual = game.find_solution_alt();
        assert_eq!(expected, actual, "Got unexpected solution");
    }
}
