use std::cmp::min;
use std::iter;
use crate::util::{StringError};
use aoc_lib::create_puzzle_result;
use itertools::{Itertools, Either};
use nalgebra::{Matrix2, Vector2};

create_solution!(13);

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
        .filter_map(|game| game.min_tokens(3, 1))
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
        .filter_map(|game| game.min_tokens(3, 1))
        .inspect(|_| solvable += 1)
        .map(|(a, b)| a * 3 + b)
        .sum();

    Ok(PuzzleResult { solvable, tokens })
});

impl ArcadeGame {
    /// Finds the minimum number of moves needed to win the prize.
    /// The minimum is determined by the cost of the buttons.
    ///
    /// Returns `None` if the prize is not achievable.
    fn min_tokens(&self, button_a_cost: u64, button_b_cost: u64) -> Option<(u64, u64)> {
        self.find_possible_solutions()
            .min_by(|&(first_a, first_b), &(second_a, second_b)| {
                let first_cost = first_a * button_a_cost + first_b * button_b_cost;
                let second_cost = second_a * button_a_cost + second_b * button_b_cost;

                first_cost.cmp(&second_cost)
            })
    }

    #[allow(non_snake_case)]
    fn find_possible_solutions(&self) -> impl Iterator<Item=(u64, u64)> + '_ {
        let A = Matrix2::from_columns(&[
            self.button_a.distance_moved.map(|x| x as f64),
            self.button_b.distance_moved.map(|x| x as f64)
        ]);
        let b = self.prize_location.map(|x| x as f64);

        // Try to check if there is a unique solution.
        if let Some(x) = A.lu().solve(&b) {
            let move_a = x.x.round() as u64;
            let move_b = x.y.round() as u64;

            // Either the solution would be an integer, or the prize is not solvable with an integer number of moves.
            // (If the solution is an integer, the round function should not change the value.)
            return if self.is_valid_move(move_a, move_b) {
                Some(Either::Left(iter::once((move_a, move_b))))
                    .into_iter()
                    .flatten()
            } else {
                None.into_iter()
                    .flatten()
            }
        }

        // The matrix is not invertible, so the prize either has an infinite number of solutions or
        // no solution.
        let x = A.svd(true, true)
            .solve(&b, 0.0)
            .expect("We already calculated the singular vectors");

        if !b.relative_eq(&(A * x), 1e-6, 1e-6) {
            // The prize has no solution, we only got an approximation.
            return None.into_iter()
                .flatten();
        }

        // The maximum number of moves for the A button before we overshoot the prize.
        let max_a = min(
            self.prize_location.x / self.button_a.distance_moved.x,
            self.prize_location.y / self.button_a.distance_moved.y
        );

        // Iterate over all possible moves for the A button and
        // calculate the corresponding moves for the B button.
        let res = (0..=max_a)
            .filter_map(|a_moves| {
                let remaining = self.prize_location - self.button_a.distance_moved(a_moves);
                if self.button_b.distance_moved.x == 0 && self.button_b.distance_moved.y == 0 {
                    // The B button does not move, so we can only solve the prize if the remaining
                    // distance is zero.
                    if self.is_valid_move(a_moves, 0) {
                        Some((a_moves, 0))
                    } else {
                        None
                    }
                } else if self.button_b.distance_moved.x == 0 {
                    // The B button only moves in the y direction.
                    if self.is_valid_move(a_moves, remaining.y / self.button_b.distance_moved.y) {
                        Some((a_moves, remaining.y / self.button_b.distance_moved.y))
                    } else {
                        None
                    }
                } else if self.button_b.distance_moved.y == 0 {
                    // The B button only moves in the x direction.
                    if self.is_valid_move(a_moves, remaining.x / self.button_b.distance_moved.x) {
                        Some((a_moves, remaining.x / self.button_b.distance_moved.x))
                    } else {
                        None
                    }
                } else {
                    // The B button moves in both directions.
                    let b_moves_x = remaining.x / self.button_b.distance_moved.x;
                    let b_moves_y = remaining.y / self.button_b.distance_moved.y;

                    if b_moves_x == b_moves_y && self.is_valid_move(a_moves, b_moves_x) {
                        Some((a_moves, b_moves_x))
                    } else {
                        None
                    }
                }
            });

        Some(Either::Right(res))
            .into_iter()
            .flatten()
    }

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

impl Button {
    /// Returns the distance moved by the button with the given number of moves.
    pub fn distance_moved(&self, moves: u64) -> Vector2<u64> {
        self.distance_moved * moves
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
