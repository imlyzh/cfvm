use fcir::{
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

impl<D> EGraph<D> {
  pub fn matching_op(&mut self, op: OpPat) -> Vec<Vec<(Symbol, ENode<D>)>> {
    let value = ValuePat::Use(OpPatHand::new(op));
    self.matching_value(value)
  }

  pub fn matching_value(&mut self, value: ValuePat) -> Vec<Vec<(Symbol, ENode<D>)>> {
    let form = value.get_form();
    let form = form.unwrap();
    let r = self.likes.find_collect(&form);
    if r.is_none() {
      return vec![];
    }
    let r = r.unwrap();
    // fixme
    // let r =
    r.iter()
      .filter_map(|x| value.matching(x))
      .flatten()
      .collect::<Vec<_>>()
    // .fold(vec![], |a, b| unbalanced_product(&a, &b));
    // if r.is_empty() {
    //   return None;
    // }
    // Some(r)
  }
}

pub type MatchValue<D> = ENode<D>;

impl<D> Matcher<EOp<D>> for OpPat {
  type Output = Vec<Vec<(Symbol, MatchValue<D>)>>;
  // Option<Vec<(Symbol, MatchValue<D>)>>
  fn matching(&self, op: &EOp<D>) -> Self::Output {
    // let mut catch: Vec<(_, MatchValue<D>)> = self.0.matching(&op.opcode)?;
    let _: Option<()> = self.0.matching(&op.opcode);
    // let catch1: Vec<(Symbol, MatchValue<D>)> = self
    self
      .1
      .iter()
      .zip(op.uses.iter())
      .flat_map(|(a, b)| a.matching(&b.as_ref().borrow() as &EClass<D>))
      .collect::<Vec<_>>()
    // .into_iter()
    // .flatten()
    // .collect()
  }
}

impl<D> Matcher<EOpHand<D>> for OpPatHand {
  type Output = Vec<Vec<(Symbol, MatchValue<D>)>>;
  fn matching(&self, i: &EOpHand<D>) -> Self::Output {
    let r = &self.as_ref().borrow() as &OpPat;
    r.matching(&i.as_ref().borrow())
  }
}

// fn match_eop_from_oppat<D>(this: &OpPat, op: &EOp<D>) -> Option<Vec<Vec<(Symbol, MatchValue<D>)>>> {
//   // let mut catch: Vec<(_, MatchValue<D>)> = self.0.matching(&op.opcode)?;
//   let _: Vec<(Symbol, MatchValue<D>)> = this.0.matching(&op.opcode)?;
//   // let catch1: Vec<(Symbol, MatchValue<D>)> = self
//   let catch: Vec<(Symbol, MatchValue<D>)> = this
//     .1
//     .iter()
//     .zip(op.uses.iter())
//     .map(|(a, b)| match_eclass_from_catch_valuepat(a, &b.as_ref().borrow() as &EClass<D>))
//     .collect::<Option<Vec<Vec<_>>>>()?
//     .into_iter()
//     .flatten()
//     .collect();
//   // catch.extend(catch1);
//   Some(catch)
// }

impl<D> Matcher<EClass<D>> for Catch<ValuePat> {
  type Output = Vec<Vec<(Symbol, MatchValue<D>)>>;
  fn matching(&self, i: &EClass<D>) -> Self::Output {
    if let Some(pat) = &self.0 {
      let r = pat.matching(i);
      if let Some(sym) = &self.1 {
        r.into_iter()
          .map(|(v, mut r)| {
            r.push((sym.clone(), v));
            r
          })
          .collect()
      } else {
        r.into_iter().map(|(_, r)| r).collect()
      }
    } else {
      vec![vec![]]
    }
  }
}

// fn match_eclass_from_catch_valuepat<D>(
//   this: &Catch<ValuePat>,
//   i: &EClass<D>,
// ) -> Vec<Vec<(Symbol, MatchValue<D>)>> {
//   if let Some(pat) = &this.0 {
//     let r = match_eclass_from_valuepat(pat, i);
//     if let Some(sym) = &this.1 {
//       r.into_iter()
//         .flat_map(|(v, mut r)| {
//           r.iter_mut().for_each(|r| r.push((sym.clone(), v)));
//           r
//         })
//         .collect()
//     } else {
//       r.into_iter().flat_map(|(_, r)| r).collect()
//     }
//   } else {
//     vec![]
//   }
// }

// fn match_eclass_from_valuepat<D>(
//   this: &ValuePat,
//   i: &EClass<D>,
// ) -> Vec<(ENode<D>, Vec<Vec<(Symbol, MatchValue<D>)>>)> {
//   let mut r = vec![];
//   for node in &i.nodes {
//     if let Some(catch) = this.matching(node) {
//       r.push((node.clone(), catch))
//     }
//   }
//   r
// }

impl<D> Matcher<EClass<D>> for ValuePat {
  type Output = Vec<(ENode<D>, Vec<(Symbol, MatchValue<D>)>)>;
  fn matching(&self, i: &EClass<D>) -> Self::Output {
    let mut r = vec![];
    for node in &i.nodes {
      if let Some(catch) = self.matching(node) {
        for catch in catch {
          r.push((node.clone(), catch))
        }
      }
    }
    r
  }
}

impl<D> Matcher<ENode<D>> for ValuePat {
  type Output = Option<Vec<Vec<(Symbol, MatchValue<D>)>>>;
  fn matching(&self, i: &ENode<D>) -> Self::Output {
    self.matching(&i.body)
  }
}

impl<D> Matcher<RawENode<D>> for ValuePat {
  type Output = Option<Vec<Vec<(Symbol, MatchValue<D>)>>>;
  fn matching(&self, i: &RawENode<D>) -> Self::Output {
    match (self, i) {
      (ValuePat::Use(op), RawENode::Use(op1)) => Some(op.matching(op1)),
      (ValuePat::Const(v), RawENode::Const(v1)) => {
        if v == v1 {
          Some(vec![vec![]])
        } else {
          None
        }
      },
      (ValuePat::Argument(v), RawENode::Argument(v1)) => {
        if v == v1 {
          Some(vec![vec![]])
        } else {
          None
        }
      },
      (ValuePat::Label(v), RawENode::Label(v1)) => {
        if v == v1 {
          Some(vec![vec![]])
        } else {
          None
        }
      },
      _ => None,
    }
  }
}
