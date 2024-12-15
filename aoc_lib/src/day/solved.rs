use std::sync::Arc;
use async_trait::async_trait;
use anyhow::Result;

use crate::{AocClient, SolutionPart1, SolutionPart2};
use crate::puzzle::{SolutionPart1Ext, SolutionPart2Ext};

#[async_trait]
pub trait SolvedDayTrait: sealed::Sealed {
    fn year(&self) -> u16;
    fn day(&self) -> u8;
    fn alt(&self) -> Option<&'static str>;

    async fn run_part1(&self, client: &AocClient) -> Result<()>;
    async fn bench_part1(&self, client: &AocClient, iterations: u32) -> Result<()>;

    async fn run_part2(&self, client: &AocClient) -> Result<()>;
    async fn bench_part2(&self, client: &AocClient, iterations: u32) -> Result<()>;
}

pub struct SolvedDay<P: SolutionPart1 + SolutionPart2> {
    marker: std::marker::PhantomData<P>,
}

pub type SolvedDayBox = Arc<dyn SolvedDayTrait>;

impl<P: SolutionPart1 + SolutionPart2> SolvedDay<P> {
    pub fn new() -> Self {
        Self {
            marker: std::marker::PhantomData,
        }
    }

    pub fn boxed() -> SolvedDayBox {
        Arc::new(Self::new())
    }

    pub fn to_trait(self) -> SolvedDayBox {
        Arc::new(self)
    }
}

#[async_trait]
impl<P: SolutionPart1 + SolutionPart2> SolvedDayTrait for SolvedDay<P> {
    fn year(&self) -> u16 {
        P::YEAR
    }

    fn day(&self) -> u8 {
        P::DAY
    }

    fn alt(&self) -> Option<&'static str> {
        P::ALT
    }

    async fn run_part1(&self, client: &AocClient) -> Result<()> {
        P::run_part1(client).await
    }

    async fn bench_part1(&self, client: &AocClient, iterations: u32) -> Result<()> {
        P::bench_part1(client, iterations).await
    }

    async fn run_part2(&self, client: &AocClient) -> Result<()> {
        P::run_part2(client).await
    }

    async fn bench_part2(&self, client: &AocClient, iterations: u32) -> Result<()> {
        P::bench_part2(client, iterations).await
    }
}

impl<P: SolutionPart1 + SolutionPart2> sealed::Sealed for SolvedDay<P> {}

mod sealed {
    pub trait Sealed {}
}