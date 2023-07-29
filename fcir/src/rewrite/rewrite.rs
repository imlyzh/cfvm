use std::collections::HashMap;

use rewrite_system::Rewrite;

use crate::{
  op::Op,
  symbol::{Name, Symbol},
  types::FuncType,
  value::Value,
};

use super::pattern::{CatchExpr, ItemPat, NamePat, OpPat};

impl Rewrite<OpPat, Op> for Vec<(Symbol, Value)> {
  fn rewrite(&self, pat: &OpPat) -> Result<Op, ()> {
    todo!()
  }
}

/*
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
 */

impl Rewrite<NamePat, Name> for Vec<(Symbol, Value)> {
  fn rewrite(&self, pat: &NamePat) -> Result<Name, ()> {
    todo!()
  }
}

/*
impl Rewrite<NamePat, ()> for Name {
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
 */

impl Rewrite<Option<FuncType>, FuncType> for Vec<(Symbol, Value)> {
  fn rewrite(&self, pat: &Option<FuncType>) -> Result<FuncType, ()> {
    todo!()
  }
}
