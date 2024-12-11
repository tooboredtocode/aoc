use aoc_lib::{SolutionPart1, SolutionPart2};
use crate::util::StringError;

create_solution!(5);

#[derive(Debug)]
pub struct PuzzleInput {
    rules: rules::Rules,
    updates: Vec<Update>,
}

#[derive(Debug, Clone)]
pub struct Update {
    pages: Vec<u32>,
}

impl SolutionPart1 for PuzzleSolution {
    type Input = PuzzleInput;
    type SolveError = StringError;
    type Result = String;

    fn solve(input: Self::Input) -> Result<Self::Result, Self::SolveError> {
        let valid_updates = input.updates.iter().filter(|update| {
            update.pages.iter()
                .enumerate()
                .map(|(i, page)| { // Get the pages that come after this page
                    let next = update.pages
                        .iter()
                        .skip(i + 1)
                        .copied();
                    (page, next)
                })
                .all(|(&page, mut next)| {
                    // Check if any of the pages that must come don't have to come before this page
                    next.all(|n| !input.rules.must_come_before(page, n))
                })
        }).collect::<Vec<_>>();

        let res = valid_updates.iter().map(|update| {
            let middle = update.pages.len() / 2;
            update.pages.get(middle)
        }).sum::<Option<u32>>()
            .ok_or_else(|| StringError::new("Failed to sum, some valid updates are empty"))?;

        Ok(format!("Found {} valid updates, result: {}", valid_updates.len(), res))
    }
}

impl SolutionPart2 for PuzzleSolution {
    type Input = PuzzleInput;
    type SolveError = StringError;
    type Result = String;

    fn solve(input: Self::Input) -> Result<Self::Result, Self::SolveError> {
        let mut invalid_updates = input.updates.iter().filter(|update| {
            update.pages.iter()
                .enumerate()
                .map(|(i, page)| { // Get the pages that come after this page
                    let next = update.pages
                        .iter()
                        .skip(i + 1)
                        .copied();
                    (page, next)
                })
                .any(|(&page, mut next)| {
                    // Check if any of the pages that come after this page must come before it
                    // and the update is invalid as such
                    next.any(|n| input.rules.must_come_before(page, n))
                })
        })
            .cloned()
            .collect::<Vec<_>>();

        for update in invalid_updates.iter_mut() {
            // Try to reorder the pages in the update so that it becomes valid
            let mut new_pages = Vec::with_capacity(update.pages.len());
            while let Some(page) = update.pages.pop() {
                let insert = new_pages.iter()
                    .enumerate()
                    .filter(|(_, &n)| input.rules.must_come_after(page, n))
                    .map(|(i, _)| i)
                    .min();

                if let Some(i) = insert {
                    new_pages.insert(i, page);
                } else {
                    new_pages.push(page);
                }
            }

            update.pages = new_pages;
        }

        let res = invalid_updates.iter().map(|update| {
            let middle = update.pages.len() / 2;
            update.pages.get(middle)
        }).sum::<Option<u32>>()
            .ok_or_else(|| StringError::new("Failed to sum, some invalid updates are empty"))?;

        Ok(format!("Found {} invalid updates\nResult of reordered invalid updates: {}", invalid_updates.len(), res))
    }
}

impl aoc_lib::PuzzleInput for PuzzleInput {
    type ParseError = StringError;

    fn from_input(input: &str) -> Result<Self, Self::ParseError> {
        let (rules_text, updates_text) = input.split_once("\n\n")
            .ok_or_else(|| StringError::new("Failed to split text"))?;

        let mut rules = rules::Rules::new();

        rules_text.lines().try_for_each(|line| {
            let (from, to) = line.split_once("|")
                .ok_or_else(|| StringError::new("Failed to split line"))?;

            let from = from.parse::<u32>()
                .map_err(|e| StringError::with_cause("Failed to parse from", e))?;
            let to = to.parse::<u32>()
                .map_err(|e| StringError::with_cause("Failed to parse to", e))?;

            rules.add_rule(from, to);

            Ok(())
        })?;

        let updates = updates_text.lines().map(|line| {
            let pages = line.split(',').map(|page| {
                page.parse::<u32>()
                    .map_err(|e| StringError::with_cause("Failed to parse page", e))
            }).collect::<Result<Vec<_>, _>>()?;

            Ok(Update {
                pages,
            })
        }).collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            rules,
            updates,
        })
    }
}

// Note: we may not use all methods, but they should remain for completeness’s sake
#[allow(dead_code)]
mod rules {
    use rustc_hash::{FxHashMap, FxHashSet};

    #[derive(Debug)]
    pub struct Rules {
        mapping: FxHashMap<u32, RuleOrdering>,
    }

    #[derive(Debug, Clone)]
    struct RuleOrdering {
        /// The pages that must come before this page
        before: FxHashSet<u32>,
        /// The pages that must come after this page
        after: FxHashSet<u32>,
    }

    impl Rules {
        pub(super) fn new() -> Self {
            Self {
                mapping: FxHashMap::default(),
            }
        }

        pub(super) fn add_rule(&mut self, prev: u32, next: u32) {
            self.mapping.entry(next).or_insert_with(RuleOrdering::new).before.insert(prev);
            self.mapping.entry(prev).or_insert_with(RuleOrdering::new).after.insert(next);
        }

        pub(super) fn remove_rule(&mut self, prev: u32, next: u32) {
            if let Some(rule) = self.mapping.get_mut(&next) {
                rule.before.remove(&prev);
            }
            if let Some(rule) = self.mapping.get_mut(&prev) {
                rule.after.remove(&next);
            }
        }

        /// Get all the pages that must come before the given page
        pub(super) fn get_prev(&self, page: u32) -> impl Iterator<Item = u32> + '_ {
            self.mapping.get(&page)
                .map(|rule| rule.before.iter().copied())
                .into_iter()
                .flatten()
        }

        /// Get all the pages that must come after the given page
        pub(super) fn get_next(&self, page: u32) -> impl Iterator<Item = u32> + '_ {
            self.mapping.get(&page)
                .map(|rule| rule.after.iter().copied())
                .into_iter()
                .flatten()
        }

        /// Check if `before` must come before `page`
        pub(super) fn must_come_before(&self, page: u32, before: u32) -> bool {
            self.mapping.get(&page)
                .map(|rule| rule.before.contains(&before))
                .unwrap_or(false)
        }

        /// Check if `after` must come after `page`
        pub(super) fn must_come_after(&self, page: u32, after: u32) -> bool {
            self.mapping.get(&page)
                .map(|rule| rule.after.contains(&after))
                .unwrap_or(false)
        }
    }

    impl RuleOrdering {
        fn new() -> Self {
            Self {
                before: FxHashSet::default(),
                after: FxHashSet::default(),
            }
        }
    }
}
