use rewrite_system::Rewrite;

use crate::{op::Op, symbol::Name, types::FuncType};

use super::pattern::{ItemPat, MatchResult, NamePat, OpPat};

impl Rewrite<OpPat, Op> for MatchResult {
  fn rewrite(&self, pat: &OpPat) -> Result<Op, ()> {
    let opcode = self.rewrite(&pat.opcode)?;
    let defs = pat
      .defs
      .iter()
      .map(|pat| self.rewrite(pat))
      .collect::<Result<Vec<_>, _>>()?;
    let uses = pat
      .uses
      .iter()
      .map(|pat| self.rewrite(pat))
      .collect::<Result<Vec<_>, _>>()?;
    let sign = self.rewrite(&pat.sign)?;

    Ok(Op {
      opcode,
      defs,
      uses,
      attr: Default::default(),
      ragion: Default::default(),
      sign,
    })
  }
}

impl<T> Rewrite<ItemPat<T>, T> for MatchResult {
  fn rewrite(&self, pat: &ItemPat<T>) -> Result<T, ()> {
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

impl Rewrite<NamePat, Name> for MatchResult {
  fn rewrite(&self, pat: &NamePat) -> Result<Name, ()> {
    Ok(Name(pat.0.clone(), pat.1.clone().ok_or(())?))
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

impl Rewrite<Option<FuncType>, FuncType> for MatchResult {
  fn rewrite(&self, pat: &Option<FuncType>) -> Result<FuncType, ()> {
    if pat.is_some() {
      Ok(pat.clone().unwrap())
    } else {
      Err(())
    }
  }
}
