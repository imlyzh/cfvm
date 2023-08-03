use crate::{Error, Records};

pub trait PartialRewrite<Pat, Output> {
  fn partial_rewrite(&self, pat: &Pat) -> Output;
}

pub trait Rewrite<Pat, Output, Err = ()> {
  fn rewrite(&self, pat: &Pat) -> Result<Output, Err>;
}

/*
#[cfg(feature = "parallel")]
pub fn parallel_rewrite_template<
  Pat: Send + Sync,
  Output: Send + Sync,
  O: Send + Sync + Rewrite<Pat, Output>>(
  match_result: Records<O>,
  pat: &Pat,
) -> Result<Vec<Output>, Error> {
  use rayon::prelude::*;
  match_result
    .par_iter()
    .map(|o| o.rewrite(pat))
    .collect::<Result<Vec<_>, _>>()
    .map_err(|_| Error::RewriteError)
  // .map(|x| x.into_iter().flatten().collect())
}
//  */

pub fn rewrite_template<Pat, Output, O: Rewrite<Pat, Output>>(
  match_result: Records<O>,
  pat: &Pat,
) -> Result<Vec<Output>, Error> {
  match_result
    .iter()
    .map(|o| o.rewrite(pat))
    .collect::<Result<Vec<_>, _>>()
    .map_err(|_| Error::RewriteError)
  // .map(|x| x.into_iter().flatten().collect())
}

pub fn tem2rewriter<'a, Tem: 'a, I, T1: Rewrite<Tem, I>>(
  tem: Records<'a, Tem>,
) -> impl 'a + Fn(&T1) -> Result<Vec<I>, Error> {
  |match_result: &T1| {
    tem
      .iter()
      .map(|tem| match_result.rewrite(tem))
      .collect::<Result<Vec<_>, _>>()
      .map_err(|_| Error::RewriteError)
  }
}

#[cfg(feature = "parallel")]
pub fn tem2parallel_rewriter<
  'a,
  Tem: 'a + Sync + Send,
  I: Sync + Send,
  T1: Sync + Send + Rewrite<Tem, I>,
>(
  tem: Records<'a, Tem>,
) -> impl 'a + Fn(&T1) -> Result<Vec<I>, Error> {
  |match_result: &T1| {
    use rayon::prelude::*;
    tem
      .par_iter()
      .map(|tem| match_result.rewrite(tem))
      .collect::<Result<Vec<_>, _>>()
      .map_err(|_| Error::RewriteError)
  }
}
