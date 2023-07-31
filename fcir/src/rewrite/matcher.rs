use rewrite_system::Matching;

use crate::{
  op::Op,
  symbol::{Name, Symbol},
  types::FuncType,
  value::Value,
};

use super::pattern::{CatchExpr, ItemPat, MatchResult1, NamePat, OpPat};

/*
pub trait Matching<T> {
  type Output;
  fn matching(&self, src: &T) -> Option<Self::Output>;
}
 */

impl Matching<OpPat, MatchResult1> for Op {
  fn matching(&self, pat: &OpPat) -> Option<MatchResult1> {
    self.opcode.matching(&pat.opcode)?;
    self.sign.matching(&pat.sign)?;
    let mut r = pat
      .uses
      .iter()
      .zip(self.uses.iter())
      .map(|(pat, src)| item_pat_matching(src, pat))
      .collect::<Option<Vec<_>>>()?
      .into_iter()
      .flatten()
      .collect::<Vec<_>>();
    let mut r1 = pat
      .defs
      .iter()
      .zip(self.defs.iter())
      .map(|(pat, src)| item_pat_matching(src, pat))
      .collect::<Option<Vec<_>>>()?
      .into_iter()
      .flatten()
      .map(|x| (x.0, Value::Use(x.1)))
      .collect::<Vec<_>>();
    r.append(&mut r1);
    Some(r)
  }
}

fn item_pat_matching<T: Clone + Eq>(this: &T, pat: &ItemPat<T>) -> Option<Option<(Symbol, T)>> {
  match pat {
    ItemPat::Catch(sym) => catch_expr_matching_anything(this, sym),
    ItemPat::Literal(lit) if lit == this => Some(None),
    _ => None,
  }
}

pub fn catch_expr_matching_anything<T: Clone>(
  this: &T,
  pat: &CatchExpr,
) -> Option<Option<(Symbol, T)>> {
  if let Some(_t) = &pat.1 {
    unimplemented!()
  }
  if let Some(s) = &pat.0 {
    return Some(Some((s.clone(), this.clone())));
  }
  Some(None)
}

impl Matching<NamePat, ()> for Name {
  fn matching(&self, pat: &NamePat) -> Option<()> {
    if self.0 != pat.0 {
      return None;
    }
    if let Some(pat) = &pat.1 {
      self.1.matching(pat)
    } else {
      Some(())
    }
  }
}

impl Matching<Symbol, ()> for Symbol {
  fn matching(&self, pat: &Symbol) -> Option<()> {
    if self != pat {
      None
    } else {
      Some(())
    }
  }
}

impl Matching<Option<FuncType>, ()> for FuncType {
  fn matching(&self, pat: &Option<FuncType>) -> Option<()> {
    if let Some(pat) = pat {
      if pat != self {
        return None;
      }
    }
    Some(())
  }
}
