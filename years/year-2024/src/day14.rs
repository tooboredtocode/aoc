use crate::prelude::*;
use aoc_utils::itertools::Itertools;
use aoc_utils::matrix::{Direction, Matrix};
use aoc_utils::nalgebra::Vector2;

create_solution!(14);

#[derive(Debug)]
pub struct Input {
    robots: Vec<Robot>,
}

#[derive(Debug, Copy, Clone)]
pub struct Robot {
    position: Vector2<u64>,
    speed: Vector2<i64>,
}

create_puzzle_result!(PuzzleResultPart1, "Safety factor after 100 seconds is: {}", safety_factor: u64);
create_puzzle_result!(PuzzleResultPart2, "The christmas easter egg appears after {} seconds", seconds: u64);

const WIDTH: u64 = 101;
const HEIGHT: u64 = 103;

const VERTICAL_DIVIDER: u64 = WIDTH / 2;
const HORIZONTAL_DIVIDER: u64 = HEIGHT / 2;

create_solution_part1!((input: Input) -> PuzzleResultPart1 {
    let (sector_1, sector_2, sector_3, sector_4) = input.count_sectors_after(100);

    let safety_factor = sector_1 * sector_2 * sector_3 * sector_4;

    Ok(PuzzleResultPart1 { safety_factor })
});

create_solution_part2!((input: Input) -> PuzzleResultPart2 {
    let (seconds, _) = (0..)
        .map(|seconds| {
            let mut res = Matrix::new(HEIGHT as usize, WIDTH as usize, false);
            for pos in input.positions_after(seconds) {
                res[(pos.y as usize, pos.x as usize)] = true;
            }
            (seconds, res)
        })
        .find(|(_, matrix)| {
            matrix.entry_iter()
                .filter(|entry| *entry.get())
                .any(|entry| {
                    Direction::iter(false)
                        .any(|dir| {
                            entry.adjacent_iter(dir)
                                .take_while(|e| *e.get())
                                .count() > 10
                        })
                })
        })
        .expect("Iter should not be empty");

    Ok(PuzzleResultPart2 { seconds })
});

impl Input {
    fn positions_after(&self, seconds: u64) -> impl Iterator<Item = Vector2<u64>> + '_ {
        self.robots.iter()
            .map(move |robot| {
                let mut pos = robot.position
                    .map(|x| x as i64);

                let travelled = robot.speed * (seconds as i64);

                pos += travelled;

                let x = pos.x.rem_euclid(WIDTH as i64) as u64;
                let y = pos.y.rem_euclid(HEIGHT as i64) as u64;

                Vector2::new(x, y)
            })
    }

    fn count_sectors_after(&self, seconds: u64) -> (u64, u64, u64, u64) {
        self.positions_after(seconds)
            .fold((0,0,0,0), |(mut sector_1, mut sector_2, mut sector_3, mut sector_4), pos| {
                if pos.x < VERTICAL_DIVIDER {
                    if pos.y < HORIZONTAL_DIVIDER {
                        sector_1 += 1;
                    } else if HORIZONTAL_DIVIDER < pos.y {
                        sector_3 += 1;
                    }
                } else if VERTICAL_DIVIDER < pos.x {
                    if pos.y < HORIZONTAL_DIVIDER {
                        sector_2 += 1;
                    } else if HORIZONTAL_DIVIDER < pos.y {
                        sector_4 += 1;
                    }
                }

                (sector_1, sector_2, sector_3, sector_4)
            })
    }
}

impl PuzzleInput for Input {
    fn from_input(input: &str) -> Result<Self> {
        let res = input.trim()
            .lines()
            .map(|line| {
                let Some((pos, speed)) = line.trim()
                    .strip_prefix("p=")
                    .and_then(|rest| rest.split_once(" v="))
                else {
                    bail!("Failed to parse robot position and speed");
                };

                let position = parse_coords(pos)?;
                let speed = parse_coords(speed)?;

                Ok(Robot { position, speed })
            })
            .try_collect()?;

        Ok(Input { robots: res })
    }
}

fn parse_coords<T>(input: &str) -> Result<Vector2<T>>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::error::Error + Send + Sync + 'static

{
    let (x, y) = input.split_once(',')
        .ok_or_else(|| Anyhow::msg("Failed to parse coordinates"))?;

    let x = x.parse()
        .context("Failed to parse x coordinate")?;
    let y = y.parse()
        .context("Failed to parse y coordinate")?;

    Ok(Vector2::new(x, y))

}
