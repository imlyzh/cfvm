pub type Records<'a, Tup> = &'a [Tup];

pub trait Matching<Pat, Output> {
  fn matching(&self, pat: &Pat) -> Option<Output>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
  MatchError,
  RewriteError,
}

pub trait Rewrite<Pat, Output, Err = ()> {
  fn rewrite(&self, pat: &Pat) -> Result<Output, Err>;
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

/*
impl<T: Unify> Unify for Vec<(Vec<usize>, T)> {
  fn unify(&self, other: &Self) -> Option<Self> {
    self
      .iter()
      .flat_map(|a| other.iter().map(move |b| (a, b)))
      .map(|(a, b)| a.unify(b))
      .collect::<Option<Vec<_>>>()
  }
}
// */

pub fn matching_all<Pat, Output, T: Matching<Pat, Output>>(
  match_source: Records<T>,
  pat: &Pat,
) -> Vec<(usize, Output)> {
  match_source
    .iter()
    .enumerate()
    .flat_map(|(index, src)| src.matching(pat).map(|out| (index, out)))
    .collect::<Vec<_>>()
}

pub fn matching_one<Pat, Output, T: Matching<Pat, Output>>(
  match_source: Records<T>,
  pat: &Pat,
) -> Vec<(usize, Output)> {
  let r = matching_all(match_source, pat);
  if r.len() == 1 {
    r
  } else {
    vec![]
  }
}

/*
pub fn and<O: Unify>(a: Records<O>, b: Records<O>) -> Vec<O> {
  // cartesian product
  a.iter()
    .flat_map(|a| b.iter().map(move |b| (a, b)))
    .flat_map(|(a, b)| a.unify(b))
    .collect::<Vec<_>>()
}
 */

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

pub fn tem2rewrite<'a, Tem: 'a, I, T1: Rewrite<Tem, I>>(
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

pub fn matching_and_produce<
  'a,
  Pat,
  Tem,
  I: Clone + Matching<Pat, T1>,
  T1: Unify + Into<T2>,
  T2: Rewrite<Tem, I>,
>(
  pat: Records<(Pat, bool)>, // pat, is matching one
  rewrite_produce: impl 'a + Fn(&T2) -> Result<Vec<I>, Error>,
  input: Records<I>,
) -> Result<Vec<I>, Error> {
  let mut r = None;
  pat
    .iter()
    .flat_map(|(pat, is_matching_one)| {
      if *is_matching_one {
        matching_one(input, pat)
      } else {
        matching_all(input, pat)
      }
    })
    .map(|x| (vec![x.0], x.1))
    .for_each(|x| {
      if r.is_none() {
        r = Some(x)
      } else {
        r = r.as_ref().unwrap().unify(&x)
      }
    });
  let (multi_index, match_result) = r.ok_or(Error::MatchError)?;
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
  'a,
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
  matching_and_produce(pat, tem2rewrite(tem), input)
}
