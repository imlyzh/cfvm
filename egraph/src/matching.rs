use fcir::{
  rewriter::{
    form::GetForm,
    pattern::{Matcher, ValuePat},
  },
  symbol::Symbol,
};

use crate::{egraph::EGraph, enode::ENode};

impl<D> EGraph<D> {
  pub fn matching(&mut self, value: ValuePat) -> Option<Vec<Vec<(Symbol, ENode<D>)>>> {
    let form = value.get_form();
    let r = self.likes.find_collect(&form)?;
    // fixme
    let r = r
      .iter()
      .filter_map(|x| value.matching(x))
      .collect::<Vec<_>>();
    if r.is_empty() {
      return None;
    }
    Some(r)
  }
}
