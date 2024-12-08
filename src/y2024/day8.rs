use std::collections::HashMap;
use crate::core::aoc_client::AocClientError;
use crate::core::{Input, Puzzle, PuzzlePart2};
use crate::util::matrix::{Matrix, MatrixEntry};
use crate::util::StringError;

/// https://adventofcode.com/2024/day/8
pub struct PuzzleSolution;

#[derive(Debug)]
pub struct PuzzleInput {
    antennas: Matrix<Option<Antenna>>,
}

#[derive(Debug, Copy, Clone)]
pub struct Antenna {
    id: char,
}

impl Puzzle for PuzzleSolution {
    type Input = PuzzleInput;
    type FetchError = AocClientError<PuzzleInput>;
    type SolveError = StringError;

    async fn fetch_input(client: &crate::core::AocClient) -> Result<Self::Input, Self::FetchError> {
        client.get_challenge(2024, 8).await
    }

    async fn solve(input: Self::Input) -> Result<(), Self::SolveError> {
        let mut antinodes = Matrix::new(input.antennas.width(), input.antennas.height(), false);

        let buckets = input.bucket_antennas();

        for (_, entries) in buckets {
            for (a, b) in entries.iter()
                .copied()
                .flat_map(|a|
                    entries.iter()
                        .copied()
                        .map(move |b| (a, b))
                )
                .filter(|(a, b)| a.position() != b.position())
            {
                let distance_a = a.distance(b) * 2;
                let distance_b = b.distance(a) * 2;

                if let Some(an_a) = a.get_at_distance(distance_a) {
                    let (x, y) = an_a.position();
                    antinodes.set(x, y, true);
                }
                if let Some(an_b) = b.get_at_distance(distance_b) {
                    let (x, y) = an_b.position();
                    antinodes.set(x, y, true);
                }
            }
        }

        let antinodes = antinodes.entry_iter()
            .filter(|entry| *entry.get())
            .count();

        println!("Found {} antinodes", antinodes);

        Ok(())
    }
}

impl PuzzlePart2 for PuzzleSolution {
    async fn solve_part2(input: Self::Input) -> Result<(), Self::SolveError> {
        let mut antinodes = Matrix::new(input.antennas.width(), input.antennas.height(), false);

        let buckets = input.bucket_antennas();

        for (_, entries) in buckets {
            for (a, b) in entries.iter()
                .copied()
                .flat_map(|a|
                    entries.iter()
                        .copied()
                        .map(move |b| (a, b))
                )
                .filter(|(a, b)| a.position() != b.position())
            {
                for i in 1.. {
                    let distance_a = a.distance(b) * i;
                    let distance_b = b.distance(a) * i;

                    let mut break_loop = true;
                    if let Some(an_a) = a.get_at_distance(distance_a) {
                        let (x, y) = an_a.position();
                        antinodes.set(x, y, true);
                        break_loop = false;
                    }
                    if let Some(an_b) = b.get_at_distance(distance_b) {
                        let (x, y) = an_b.position();
                        antinodes.set(x, y, true);
                        break_loop = false;
                    }

                    if break_loop {
                        break;
                    }
                }
            }
        }

        let antinodes = antinodes.entry_iter()
            .filter(|entry| *entry.get())
            .count();

        println!("Found {} extended antinodes", antinodes);

        Ok(())
    }
}

impl PuzzleInput {
    fn bucket_antennas(&self) -> HashMap<char, Vec<MatrixEntry<Option<Antenna>>>> {
        let mut buckets: HashMap<_, Vec<_>> = HashMap::new();

        for (entry, id) in self.antennas
            .entry_iter()
            .filter_map(|entry|
                entry.get()
                    .map(|antenna| (entry, antenna.id))
            )
        {
            buckets.entry(id)
                .or_default()
                .push(entry);
        }

        buckets
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

        let mut antennas = Matrix::new(row_height, line_length, None);

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    'A'..='Z'
                    | 'a'..='z'
                    | '0'..='9' => {
                        antennas[(y, x)] = Some(Antenna { id: c });
                    },
                    _ => {}
                }
            }
        }

        Ok(PuzzleInput { antennas })
    }
}