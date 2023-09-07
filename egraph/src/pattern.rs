
use fcir::{
  symbol::Symbol,
  rewriter::pattern::*,
};

use crate::{
  eclass::EClass,
  enode::{ENode, EOp, EOpHand, RawENode},
};

pub type MatchValue<D> = ENode<D>;


impl<D> Matcher<EOp<D>, MatchValue<D>> for OpPat {
  fn matching(&self, op: &EOp<D>) -> Option<Vec<(Symbol, MatchValue<D>)>> {
    // let mut catch: Vec<(_, MatchValue<D>)> = self.0.matching(&op.opcode)?;
    let _: Vec<(Symbol, MatchValue<D>)> = self.0.matching(&op.opcode)?;
    // let catch1: Vec<(Symbol, MatchValue<D>)> = self
    let catch: Vec<(Symbol, MatchValue<D>)> = self
      .1
      .iter()
      .zip(op.uses.iter())
      .map(|(a, b)| a.matching(&b.as_ref().borrow() as &EClass<D>))
      .collect::<Option<Vec<_>>>()?
      .into_iter()
      .flatten()
      .collect();
    // catch.extend(catch1);
    Some(catch)
  }
}


impl<D> Matcher<EOpHand<D>, MatchValue<D>> for OpPatHand {
  fn matching(&self, i: &EOpHand<D>) -> Option<Vec<(Symbol, MatchValue<D>)>> {
    self.as_ref().borrow().matching(&i.as_ref().borrow())
  }
}

impl<D> Matcher<EClass<D>, MatchValue<D>> for Catch<ValuePat> {
  fn matching(&self, i: &EClass<D>) -> Option<Vec<(Symbol, MatchValue<D>)>> {
    self.0.matching(i)
  }
}

impl<D> Matcher<EClass<D>, MatchValue<D>> for ValuePat {
  fn matching(&self, i: &EClass<D>) -> Option<Vec<(Symbol, MatchValue<D>)>> {
    for node in &i.nodes {
      if let Some(r) = self.matching(node) {
        return Some(r);
      }
    }
    None
  }
}

impl<D> Matcher<ENode<D>, MatchValue<D>> for ValuePat {
  fn matching(&self, i: &ENode<D>) -> Option<Vec<(Symbol, MatchValue<D>)>> {
    self.matching(&i.body)
  }
}

impl<D> Matcher<RawENode<D>, MatchValue<D>> for ValuePat {
  fn matching(&self, i: &RawENode<D>) -> Option<Vec<(Symbol, MatchValue<D>)>> {
    match (self, i) {
      (ValuePat::Use(op), RawENode::Use(op1)) => op.matching(op1),
      (ValuePat::Const(v), RawENode::Const(v1)) => {
        if v == v1 {
          Some(vec![])
        } else {
          None
        }
      },
      (ValuePat::Argument(v), RawENode::Argument(v1)) => {
        if v == v1 {
          Some(vec![])
        } else {
          None
        }
      },
      (ValuePat::Label(v), RawENode::Label(v1)) => {
        if v == v1 {
          Some(vec![])
        } else {
          None
        }
      },
      _ => None,
    }
  }
}
