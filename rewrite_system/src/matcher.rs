use crate::{Records, Unify};

pub trait Matching<Pat, Output> {
  fn matching(&self, pat: &Pat) -> Option<Output>;
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

pub fn pat2matcher<'a, Pat: 'a, I: Clone + Matching<Pat, T1>, T1: Unify>(
  pat: Records<'a, (Pat, bool)>,
) -> impl 'a + Fn(Records<I>) -> Option<(Vec<usize>, T1)> {
  |input: Records<I>| {
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
    r
  }
}
