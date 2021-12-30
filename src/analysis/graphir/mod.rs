pub mod live_analysis;
pub mod untils;

pub trait RootAnalysis {
    type Output;
    fn analysis(&self) -> Self::Output;
}

pub trait Analysis {
    type Context;
    fn analysis(&self, record: Self::Context) -> Self::Context;
}
