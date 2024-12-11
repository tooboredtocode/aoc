use rustc_hash::FxHashMap;
use aoc_lib::{SolutionPart1, SolutionPart2};
use crate::util::matrix::{Matrix, MatrixEntry};
use crate::util::StringError;

create_solution!(8);

#[derive(Debug)]
pub struct PuzzleInput {
    antennas: Matrix<Option<Antenna>>,
}

#[derive(Debug, Copy, Clone)]
pub struct Antenna {
    id: char,
}

pub struct PuzzleResult {
    antinodes: usize,
    extended: bool,
}

impl SolutionPart1 for PuzzleSolution {
    type Input = PuzzleInput;
    type SolveError = StringError;
    type Result = PuzzleResult;

    fn solve(input: Self::Input) -> Result<Self::Result, Self::SolveError> {
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

        Ok(PuzzleResult { antinodes, extended: false })
    }
}

impl SolutionPart2 for PuzzleSolution {
    type Input = PuzzleInput;
    type SolveError = StringError;
    type Result = PuzzleResult;

    fn solve(input: Self::Input) -> Result<Self::Result, Self::SolveError> {
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

        Ok(PuzzleResult { antinodes, extended: true })
    }
}

impl aoc_lib::PuzzleResult for PuzzleResult {
    fn display(&self) {
        if self.extended {
            println!("Found {} extended antinodes", self.antinodes);
        } else {
            println!("Found {} antinodes", self.antinodes);
        }
    }
}

impl PuzzleInput {
    fn bucket_antennas(&self) -> FxHashMap<char, Vec<MatrixEntry<Option<Antenna>>>> {
        let mut buckets: FxHashMap<_, Vec<_>> = FxHashMap::default();

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

impl aoc_lib::PuzzleInput for PuzzleInput {
    type ParseError = StringError;

    fn from_input(input: &str) -> Result<Self, Self::ParseError> {
        let antennas = Matrix::from_string_chars(input.trim(), |c| match c {
            'A'..='Z'
            | 'a'..='z'
            | '0'..='9' => Some(Antenna { id: c }),
            _ => None,
        })?;

        Ok(PuzzleInput { antennas })
    }
}
