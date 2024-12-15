use std::sync::Arc;
use async_trait::async_trait;
use anyhow::Result;

use crate::{AocClient, SolutionPart1};
use crate::puzzle::{SolutionPart1Ext};

#[async_trait]
pub trait PartialSolvedDayTrait: sealed::Sealed {
    fn year(&self) -> u16;
    fn day(&self) -> u8;
    fn alt(&self) -> Option<&'static str>;

    async fn run_part1(&self, client: &AocClient) -> Result<()>;
    async fn bench_part1(&self, client: &AocClient, iterations: u32) -> Result<()>;
}

pub struct PartialSolvedDay<P: SolutionPart1> {
    marker: std::marker::PhantomData<P>,
}

pub type PartialSolvedDayBox = Arc<dyn PartialSolvedDayTrait>;

impl<P: SolutionPart1> PartialSolvedDay<P> {
    pub fn new() -> Self {
        Self {
            marker: std::marker::PhantomData,
        }
    }

    pub fn boxed() -> PartialSolvedDayBox {
        Arc::new(Self::new())
    }

    pub fn to_trait(self) -> PartialSolvedDayBox {
        Arc::new(self)
    }
}

#[async_trait]
impl<P: SolutionPart1> PartialSolvedDayTrait for PartialSolvedDay<P> {
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
}

impl<P: SolutionPart1> sealed::Sealed for PartialSolvedDay<P> {}

mod sealed {
    pub trait Sealed {}
}