use crate::{
  op::Op,
  symbol::{Name, Symbol},
  types::FuncType,
  value::Value,
};

use super::pattern::{CatchExpr, ItemPat, NamePat, OpPat};

pub trait Matching<T> {
  type Output;
  fn matching(&self, src: &T) -> Option<Self::Output>;
}

impl Matching<Op> for OpPat {
  type Output = Vec<(Symbol, Value)>;

  fn matching(&self, src: &Op) -> Option<Self::Output> {
    self.opcode.matching(&src.opcode)?;
    self.sign.matching(&src.sign)?;
    let mut r = self
      .uses
      .iter()
      .zip(src.uses.iter())
      .map(|(pat, src)| pat.matching(src))
      .collect::<Option<Vec<_>>>()?
      .into_iter()
      .flatten()
      .collect::<Vec<_>>();
    let mut r1 = self
      .defs
      .iter()
      .zip(src.defs.iter())
      .map(|(pat, src)| pat.matching(src))
      .collect::<Option<Vec<_>>>()?
      .into_iter()
      .flatten()
      .map(|x| (x.0, Value::Use(x.1)))
      .collect::<Vec<_>>();
    r.append(&mut r1);
    Some(r)
  }
}

impl<T: Clone + Eq> Matching<T> for ItemPat<T> {
  type Output = Option<(Symbol, T)>;

  fn matching(&self, src: &T) -> Option<Self::Output> {
    match self {
      ItemPat::Catch(sym) => sym.matching(src),
      ItemPat::Literal(lit) if lit == src => Some(None),
      _ => None,
    }
  }
}

impl<T: Clone> Matching<T> for CatchExpr {
  type Output = Option<(Symbol, T)>;

  fn matching(&self, src: &T) -> Option<Self::Output> {
    if let Some(_t) = &self.1 {
      unimplemented!()
    }
    if let Some(s) = &self.0 {
      return Some(Some((s.clone(), src.clone())));
    }
    Some(None)
  }
}

impl Matching<Name> for NamePat {
  type Output = ();

  fn matching(&self, src: &Name) -> Option<Self::Output> {
    if self.0 != src.0 {
      return None;
    }
    if let Some(pat) = &self.1 {
      if pat != &src.1 {
        return None;
      }
    }
    Some(())
  }
}

impl Matching<FuncType> for Option<FuncType> {
  type Output = ();

  fn matching(&self, src: &FuncType) -> Option<Self::Output> {
    if let Some(pat) = self {
      if pat != src {
        return None;
      }
    }
    Some(())
  }
}
