use crate::prelude::*;
use std::iter::repeat_n;
use aoc_utils::itertools::Itertools;

create_solution!(9);

#[derive(Debug)]
pub struct PuzzleInputPart1 {
    fs: Vec<Block>
}

#[derive(Debug)]
pub struct PuzzleInputPart2 {
    fs: Vec<ContinuousBlock>
}

#[derive(Debug, Copy, Clone)]
pub struct ContinuousBlock {
    block: Block,
    length: usize
}

#[derive(Debug, Copy, Clone)]
pub enum Block {
    File { id: u16 },
    Empty
}

pub struct PuzzleResult {
    checksum: u64,
    alternative: bool
}

create_solution_part1!((input: PuzzleInputPart1) -> PuzzleResult {
    let mut take_back = input.fs.iter()
        .copied()
        .enumerate()
        .filter_map(|(i, id)| id.filter_map_file(|id| (i, id)))
        .rev() // Rev last so we get the correct index
        .peekable();

    let checksum = input.fs.iter()
        .copied()
        .enumerate()
        .map_while(|(i, block)| {
            if let Some(&(back_i, _)) = take_back.peek() {
                if back_i < i {
                    return None;
                }
            };

            if let Block::File { id } = block {
                return Some(Ok((i, id)));
            }

            if let Some((_, id)) = take_back.next() {
                Some(Ok((i, id)))
            } else {
                Some(Err(Anyhow::msg("Take back iterator should never run out before the main iterator")))
            }
        })
        .map_ok(|(position, id)| {
            position as u64 * id as u64
        })
        .sum::<Result<_, _>>()?;

    Ok(PuzzleResult { checksum, alternative: false })
});

create_solution_part2!((input: PuzzleInputPart2) -> PuzzleResult {
    let mut res = input.fs.clone();

    let take_back = input.fs.iter()
        .copied()
        .enumerate()
        .filter_map(|(i, block)| block.block.filter_map_file(|id| (i, id, block.length)))
        .rev();

    let mut offsets = Vec::new();

    for (mut i, id, length) in take_back {
        let Some((index, remaining)) = res.iter()
            .enumerate()
            .find(|(_, b)| b.block.is_empty() && b.length >= length)
            .map(|(i,b)| (
                i,
                if b.length == length { None } else { Some(b.length - length) }
            ))
        else {
            continue
        };
        // Adjust the index to account for the blocks we've inserted
        for &index in offsets.iter() {
            if index < i {
                i += 1;
            }
        }

        if i <= index {
            continue;
        }

        // Replace the original block
        res[i] = ContinuousBlock::new_empty(length);
        res[index] = ContinuousBlock {
            block: Block::File { id },
            length
        };
        // If the block had remaining space, insert a new empty block
        if let Some(remaining) = remaining {
            res.insert(index + 1, ContinuousBlock::new_empty(remaining));
            offsets.push(index);
        }
    }

    let checksum = res.iter()
        .copied()
        .flat_map(|b| b.iter())
        .enumerate()
        .filter_map(|(i, id)| id.filter_map_file(|id| (i, id)))
        .map(|(position, id)| {
            position as u64 * id as u64
        })
        .sum();

    Ok(PuzzleResult { checksum, alternative: false })
});

impl ContinuousBlock {
    /// Returns an iterator that repeats the block `length` times
    ///
    /// Used to iterate over the blocks in the compressed file system
    fn iter(&self) -> impl Iterator<Item = Block> {
        match self.block {
            Block::File { id } => repeat_n(Block::File { id }, self.length),
            Block::Empty => repeat_n(Block::Empty, self.length)
        }
    }

    fn new_empty(length: usize) -> Self {
        Self {
            block: Block::Empty,
            length
        }
    }

    fn from_string(s: &str) -> impl Iterator<Item = Result<Self, Anyhow>> + '_ {
        s.chars()
            .enumerate()
            .map(|(i, c)| c.to_digit(10)
                .map(|d| (i, d))
                .ok_or_else(|| Anyhow::msg("Input should only contain digits"))
            )
            .map_ok(|(i, repeat)| {
                if i % 2 == 0 {
                    let id = i / 2;
                    ContinuousBlock {
                        block: Block::File { id: id as u16 },
                        length: repeat as usize
                    }
                } else {
                    Self::new_empty(repeat as usize)
                }
            })
    }
}

impl Block {
    fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }

    fn filter_map_file<F, T>(self, f: F) -> Option<T>
    where F: FnOnce(u16) -> T
    {
        match self {
            Self::File { id } => Some(f(id)),
            Self::Empty => None
        }
    }
}

impl PuzzleInput for PuzzleInputPart1 {
    fn from_input(input: &str) -> Result<Self> {
        let fs = ContinuousBlock::from_string(input.trim())
            .map_ok(|block| block.iter())
            .flatten_ok()
            .try_collect()?;

        Ok(Self { fs })
    }
}

impl PuzzleInput for PuzzleInputPart2 {
    fn from_input(input: &str) -> Result<Self> {
        let fs = ContinuousBlock::from_string(input.trim())
            .try_collect()?;

        Ok(Self { fs })
    }
}

impl aoc_lib::PuzzleResult for PuzzleResult {
    fn display(&self) {
        if self.alternative {
            println!("Calculated checksum of alternative compressed file system: {}", self.checksum);
        } else {
            println!("Calculated checksum of compressed file system: {}", self.checksum);
        }
    }
}
