use std::collections::HashMap;

use fcir::{
  op::Op,
  rewriter::pattern::{Catch, OpPat},
  symbol::Symbol,
};

use crate::enode::ENode;

type MatchRecord<D> = HashMap<Symbol, ENode<D>>;

pub trait Rewriter<D> {
  type Output;
  fn rewrite(&self, record: &MatchRecord<D>) -> Self::Output;
}

impl<D> Rewriter<D> for OpPat {
  type Output = Op;

  fn rewrite(&self, record: &MatchRecord<D>) -> Self::Output {
    todo!()
  }
}

impl<D, T: Rewriter<D>> Rewriter<D> for Catch<T> {
  type Output = Option<T::Output>;

  fn rewrite(&self, record: &MatchRecord<D>) -> Self::Output {
    match (&self.0, &self.1) {
      (None, None) => None,
      (None, Some(sym)) => todo!(),
      (Some(pat), None) => Some(pat.rewrite(record)),
      (Some(pat), Some(sym)) => {
        todo!()
      },
    }
  }
}
