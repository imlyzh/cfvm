pub type Records<'a, Tup> = &'a [Tup];

pub trait Matching<Pat, Output> {
  fn matching(&self, pat: &Pat) -> Option<Output>;
}

pub trait Rewrite<Pat, Output> {
  fn rewrite(&self, pat: &Pat) -> Option<Vec<Output>>;
}

pub trait Unify: Sized {
  fn unify(&self, other: &Self) -> Option<Self>;
}

impl<T: Unify> Unify for Vec<(Vec<usize>, T)> {
  fn unify(&self, other: &Self) -> Option<Self> {
    self
      .iter()
      .flat_map(|a| other.iter().map(move |b| (a, b)))
      .map(|(a, b)| {
        a.1.unify(&b.1).map(|t| {
          let mut i: Vec<usize> = vec![];
          i.extend(&a.0);
          i.extend(&b.0);
          (i, t)
        })
      })
      .collect::<Option<Vec<_>>>()
  }
}

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

pub fn and<O: Unify>(a: Records<O>, b: Records<O>) -> Vec<O> {
  // cartesian product
  a.iter()
    .flat_map(|a| b.iter().map(move |b| (a, b)))
    .flat_map(|(a, b)| a.unify(b))
    .collect::<Vec<_>>()
}

pub fn rewrite_template<Pat, Output, O: Rewrite<Pat, Output>>(
  match_result: Records<O>,
  pat: &Pat,
) -> Option<Vec<Output>> {
  match_result
    .iter()
    .map(|o| o.rewrite(pat))
    .collect::<Option<Vec<_>>>()
    .map(|x| x.into_iter().flatten().collect())
}

pub fn rewrite<Pat, Tem, Output, I: Matching<Pat, T1>, T1: Unify + Rewrite<Tem, Output>>(
  pat: Records<(Pat, bool)>, // pat, is matching one
  tem: Records<Pat>,
  input: Records<I>,
) -> Option<Vec<Output>> {
  let r = pat
    .iter()
    .flat_map(|(pat, is_matching_one)| {
      if *is_matching_one {
        matching_one(input, pat)
      } else {
        matching_all(input, pat)
      }
    })
    .map(|x| (vec![x.0], x.1));
  // .fold(|a, b| a.unify(b)).collect::<Vec<_>>();
  // (&a).unify(&b).into_iter().flatten().collect()
  todo!()
}
