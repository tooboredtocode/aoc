use rustc_hash::FxHashMap;
use itertools::{Either, Itertools};
use aoc_lib::create_puzzle_result;
use crate::util::num::NumUtils;
use crate::util::StringError;

create_solution!(11);

#[derive(Debug)]
pub struct PuzzleInput {
    stones: Vec<Stone>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Stone {
    value: u64,
}

create_puzzle_result!(PuzzleResultPart1, "Stones after 25 blinks: {}", stones: u64);
create_puzzle_result!(PuzzleResultPart2, "Stones after 75 blinks: {}", stones: u64);

create_solution_part1!((input: PuzzleInput) -> PuzzleResultPart1 {
    let mut cache = FxHashMap::default();
    let stones = input.stones.iter()
        .map(|stone| stone.count_stones_after_n_blinks(25, &mut cache))
        .sum();

    Ok(PuzzleResultPart1 { stones })
});

create_solution_part2!((input: PuzzleInput) -> PuzzleResultPart2 {
    let mut cache = FxHashMap::default();
    let stones = input.stones.iter()
        .map(|stone| stone.count_stones_after_n_blinks(75, &mut cache))
        .sum();

    Ok(PuzzleResultPart2 { stones })
});

impl Stone {
    fn new(value: u64) -> Self {
        Self { value }
    }

    fn count_stones_after_n_blinks(&self, n: u64, memoized: &mut FxHashMap<(u64, u64), u64>) -> u64 {
        if n == 0 {
            // No more blinks, return 1 since they cannot be split further
            return 1;
        }

        if let Some(&result) = memoized.get(&(n, self.value)) {
            return result;
        }

        match self.run_sim_once() {
            Either::Left(next) => {
                let res = next.count_stones_after_n_blinks(n - 1, memoized);
                memoized.insert((n, self.value), res);
                res
            },
            Either::Right([left, right]) => {
                let left_result = left.count_stones_after_n_blinks(n - 1, memoized);
                let right_result = right.count_stones_after_n_blinks(n - 1, memoized);
                let result = left_result + right_result;
                memoized.insert((n, self.value), result);
                result
            },
        }
    }

    fn run_sim_once(&self) -> Either<Stone, [Stone; 2]> {
        match self.value {
            0 => Either::Left(Stone::new(1)),
            value if value.count_digits() % 2 == 0 => {
                let (left, right) = value.split_digits_at(value.count_digits() as usize / 2);
                Either::Right([Stone::new(left), Stone::new(right)])
            },
            value => Either::Left(Stone::new(value * 2024)),
        }
    }
}

impl aoc_lib::PuzzleInput for PuzzleInput {
    type ParseError = StringError;

    fn from_input(input: &str) -> Result<Self, Self::ParseError> {
        let stones = input.trim()
            .split_whitespace()
            .map(|s|
                s.parse()
                    .map(|value| Stone { value })
                    .map_err(|e| StringError::with_cause("Failed to parse stone: ", e))
            )
            .try_collect()?;

        Ok(Self { stones })
    }
}

#[cfg(test)]
mod test {
    use aoc_lib::PuzzleInput as _;
    use super::*;

    const INPUT: &str = "125 17";

    #[test]
    fn test_parse_input() {
        let input = PuzzleInput::from_input(INPUT).expect("Input should be valid");
        assert_eq!(input.stones.len(), 2);
        assert_eq!(input.stones[0].value, 125);
        assert_eq!(input.stones[1].value, 17);
    }

    #[test]
    fn test_run_sim_once() {
        let stone = Stone::new(125);
        assert_eq!(stone.run_sim_once(), Either::Left(Stone::new(253000)));

        let stone = Stone::new(17);
        assert_eq!(stone.run_sim_once(), Either::Right([Stone::new(1), Stone::new(7)]));

        let stone = Stone::new(0);
        assert_eq!(stone.run_sim_once(), Either::Left(Stone::new(1)));
    }

    #[test]
    fn test_run_0_times() {
        let input = PuzzleInput::from_input(INPUT).expect("Input should be valid");
        let mut cache = FxHashMap::default();

        let result = input.stones.iter()
            .map(|stone| stone.count_stones_after_n_blinks(0, &mut cache))
            .sum::<u64>();

        assert_eq!(result, 2);
    }

    #[test]
    fn test_run_2_times() {
        let input = PuzzleInput::from_input(INPUT).expect("Input should be valid");
        let mut cache = FxHashMap::default();

        let result = input.stones.iter()
            .map(|stone| stone.count_stones_after_n_blinks(2, &mut cache))
            .sum::<u64>();

        assert_eq!(result, 4);
    }


    #[test]
    fn test_run_25_times() {
        let input = PuzzleInput::from_input(INPUT).expect("Input should be valid");
        let mut cache = FxHashMap::default();

        let result = input.stones.iter()
            .map(|stone| stone.count_stones_after_n_blinks(25, &mut cache))
            .sum::<u64>();

        assert_eq!(result, 55312);
    }
}
