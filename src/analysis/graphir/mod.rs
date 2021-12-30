pub mod live_analysis;
pub mod untils;

pub trait RootAnalysis {
    type Output;
    fn live_analysis(&self) -> Self::Output;
}

pub trait Analysis {
    type Context;
    fn live_analysis(&self, record: Self::Context) -> Self::Context;
}
