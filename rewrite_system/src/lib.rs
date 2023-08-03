use matcher::{pat2matcher, Matching};
use rewriter::{tem2rewriter, Rewrite};

pub mod matcher;
pub mod rewriter;

pub type Records<'a, Tup> = &'a [Tup];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
  MatchError,
  RewriteError,
}

pub trait Unify: Sized {
  fn unify(&self, other: &Self) -> Option<Self>;
}

impl<T: Unify> Unify for (Vec<usize>, T) {
  fn unify(&self, other: &Self) -> Option<Self> {
    self.1.unify(&other.1).map(|t| {
      let mut i: Vec<usize> = vec![];
      i.extend(&self.0);
      i.extend(&other.0);
      (i, t)
    })
  }
}

pub fn matching_and_rewrite_produce<
  'a,
  Pat,
  Tem,
  I: Clone + Matching<Pat, T1>,
  T1: Unify + Into<T2>,
  T2: Rewrite<Tem, I>,
>(
  match_produce: impl 'a + Fn(Records<I>) -> Option<(Vec<usize>, T1)>,
  rewrite_produce: impl 'a + Fn(&T2) -> Result<Vec<I>, Error>,
  input: Records<I>,
) -> Result<Vec<I>, Error> {
  let (multi_index, match_result) = match_produce(input).ok_or(Error::MatchError)?;
  let rewrite_result = rewrite_produce(&match_result.into())?;
  let mut input = input.iter().cloned().map(Some).collect::<Vec<_>>();
  for i in multi_index {
    input[i] = None;
  }
  let mut input = input.into_iter().flatten().collect::<Vec<_>>();
  input.extend(rewrite_result);
  Ok(input)
}

pub fn matching_and_rewrite<
  Pat,
  Tem,
  I: Clone + Matching<Pat, T1>,
  T1: Unify + Into<T2>,
  T2: Rewrite<Tem, I>,
>(
  pat: Records<(Pat, bool)>, // pat, is matching one
  tem: Records<Tem>,
  input: Records<I>,
) -> Result<Vec<I>, Error> {
  matching_and_rewrite_produce(pat2matcher(pat), tem2rewriter(tem), input)
}
