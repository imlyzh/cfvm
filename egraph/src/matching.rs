use cfir::{
  rewriter::pattern::*,
  rewriter::{
    form::GetForm,
    pattern::{Matcher, ValuePat},
  },
  symbol::Symbol,
};

use crate::{
  eclass::EClass,
  egraph::EGraph,
  enode::{ENode, EOp, EOpHand, RawENode},
};

type MatchRecord<D> = Vec<(Symbol, ENode<D>)>;

impl<D> EGraph<D> {
  pub fn matching_op(&mut self, op: OpPat) -> Vec<(ENode<D>, MatchRecord<D>)> {
    let value = ValuePat::Use(OpPatHand::new(op), 0); // FIXME: rewrite system
    self.matching_value(value)
  }

  pub fn matching_value(&mut self, value: ValuePat) -> Vec<(ENode<D>, MatchRecord<D>)> {
    let form = value.get_form();
    let form = form.unwrap();
    let r = self.likes.find_collect(&form);
    if r.is_none() {
      return vec![];
    }
    let r = r.unwrap();

    r.iter()
      .filter_map(|node| -> Option<(ENode<D>, Vec<MatchRecord<D>>)> { value.matching(node) })
      .flat_map(|(node, records)| -> Vec<(ENode<D>, MatchRecord<D>)> {
        records
          .into_iter()
          .map(|record| (node.clone(), record))
          .collect()
      })
      .collect::<Vec<_>>()
  }
}

pub type MatchValue<D> = ENode<D>;

impl<D> Matcher<EOp<D>> for OpPat {
  type Output = Vec<MatchRecord<D>>;
  fn matching(&self, op: &EOp<D>) -> Self::Output {
    let _: Option<()> = self.0.matching(&op.opcode);
    self
      .1
      .iter()
      .zip(op.uses.iter())
      .flat_map(|(a, b)| a.matching(&b.as_ref().borrow() as &EClass<D>))
      .collect::<Vec<_>>()
  }
}

impl<D> Matcher<EOpHand<D>> for OpPatHand {
  type Output = Vec<MatchRecord<D>>;
  fn matching(&self, i: &EOpHand<D>) -> Self::Output {
    let r = &self.as_ref().borrow() as &OpPat;
    r.matching(&i.as_ref().borrow())
  }
}

impl<D> Matcher<EClass<D>> for Catch<ValuePat> {
  type Output = Vec<MatchRecord<D>>;
  fn matching(&self, i: &EClass<D>) -> Self::Output {
    match (&self.0, &self.1) {
      (None, Some(sym)) => i
        .nodes
        .iter()
        .map(|node| vec![(sym.clone(), node.clone())])
        .collect(),
      (Some(pat), None) => pat.matching(i).into_iter().map(|(_, r)| r).collect(),
      (Some(pat), Some(sym)) => pat
        .matching(i)
        .into_iter()
        .map(|(node, mut r)| {
          r.push((sym.clone(), node));
          r
        })
        .collect(),
      (None, None) => vec![],
    }
  }
}

impl<D> Matcher<EClass<D>> for ValuePat {
  type Output = Vec<(ENode<D>, MatchRecord<D>)>;
  fn matching(&self, i: &EClass<D>) -> Self::Output {
    let mut r = vec![];
    for node in &i.nodes {
      if let Some((node, catch)) = self.matching(node) {
        for catch in catch {
          r.push((node.clone(), catch))
        }
      }
    }
    r
  }
}

impl<D> Matcher<ENode<D>> for ValuePat {
  type Output = Option<(ENode<D>, Vec<MatchRecord<D>>)>;
  fn matching(&self, i: &ENode<D>) -> Self::Output {
    let r = self.matching(&i.body)?;
    Some((i.clone(), r))
  }
}

impl<D> Matcher<RawENode<D>> for ValuePat {
  type Output = Option<Vec<MatchRecord<D>>>;
  fn matching(&self, i: &RawENode<D>) -> Self::Output {
    match (self, i) {
      (ValuePat::Use(op, loff), RawENode::Use(op1, roff)) => {
        if loff == roff {
          Some(op.matching(op1))
        } else {
          None
        }
      },
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
