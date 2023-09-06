use std::{cell::RefCell, hash::Hash, rc::Rc};

use fcir::{
  symbol::{Name, Symbol},
  value::{Argument, Constant},
};

use crate::{
  eclass::EClass,
  enode::{ENode, EOp, EOpHand, RawENode},
  form::{Form, GetForm},
};

pub trait Matcher<T, D> {
  fn matching(&self, i: &T) -> Option<Vec<(Symbol, MatchValue<D>)>>;
}

pub type MatchValue<D> = ENode<D>;

/*
#[derive(Debug, Clone, PartialEq, Hash)]
pub enum MatchValue<D> {
  // Name(Name),
  Const(Constant),
  Use(Id<D>),
  Argument(Argument),
  Label(Symbol),
}


//  */

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpPat(
  // pub Catch<Name>,
  pub Name,
  pub Vec<Catch<ValuePat>>,
);

impl GetForm for OpPat {
  fn get_form(&self) -> Form {
    Form::Form(
      // self.0 .0.clone(),
      self.0.clone(),
      self.1.iter().map(GetForm::get_form).collect(),
    )
  }
}

impl<D> Matcher<EOp<D>, D> for OpPat {
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

#[derive(Debug, Clone, Eq)]
pub struct OpPatHand(Rc<RefCell<OpPat>>);

impl OpPatHand {
  pub fn new(value: OpPat) -> Self {
    Self(Rc::new(RefCell::new(value)))
  }
}

impl GetForm for OpPatHand {
  fn get_form(&self) -> Form {
    self.as_ref().borrow().get_form()
  }
}

impl<D> Matcher<EOpHand<D>, D> for OpPatHand {
  fn matching(&self, i: &EOpHand<D>) -> Option<Vec<(Symbol, MatchValue<D>)>> {
    self.as_ref().borrow().matching(&i.as_ref().borrow())
  }
}

impl AsRef<RefCell<OpPat>> for OpPatHand {
  fn as_ref(&self) -> &RefCell<OpPat> {
    self.0.as_ref()
  }
}

impl Hash for OpPatHand {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    Rc::into_raw(self.0.clone()).hash(state);
  }
}

impl PartialEq for OpPatHand {
  fn eq(&self, other: &Self) -> bool {
    Rc::into_raw(self.0.clone()) as usize == Rc::into_raw(other.0.clone()) as usize
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Catch<T>(pub T, pub Option<Symbol>);

impl<T: GetForm> GetForm for Catch<T> {
  fn get_form(&self) -> Form {
    self.0.get_form()
  }
}

impl<D> Matcher<EClass<D>, D> for Catch<ValuePat> {
  fn matching(&self, i: &EClass<D>) -> Option<Vec<(Symbol, MatchValue<D>)>> {
    self.0.matching(i)
  }
}

/*
// disable name catch
impl<D> Matcher<Name, D> for Catch<Name> {
  fn matching(&self, i: &Name) -> Option<Vec<(Symbol, MatchValue<D>)>> {
    if &self.0 != i {
      return None;
    }
    if let Some(sym) = self.1.as_ref() {
      Some(vec![(sym.clone(), MatchValue::Name(i.clone()))])
    } else {
      Some(vec![])
    }
  }
}
//  */

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValuePat {
  Const(Constant),
  Use(OpPatHand),
  Argument(Argument),
  Label(Symbol),
}

impl GetForm for ValuePat {
  fn get_form(&self) -> Form {
    match self {
      ValuePat::Use(op) => op.get_form(),
      ValuePat::Const(_) | ValuePat::Argument(_) | ValuePat::Label(_) => Form::Atom,
    }
  }
}

impl<D> Matcher<EClass<D>, D> for ValuePat {
  fn matching(&self, i: &EClass<D>) -> Option<Vec<(Symbol, MatchValue<D>)>> {
    for node in &i.nodes {
      if let Some(r) = self.matching(node) {
        return Some(r);
      }
    }
    None
  }
}

impl<D> Matcher<ENode<D>, D> for ValuePat {
  fn matching(&self, i: &ENode<D>) -> Option<Vec<(Symbol, MatchValue<D>)>> {
    self.matching(&i.body)
  }
}

impl<D> Matcher<RawENode<D>, D> for ValuePat {
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

impl<D> Matcher<Name, D> for Name {
  fn matching(&self, i: &Name) -> Option<Vec<(Symbol, MatchValue<D>)>> {
    if self == i {
      Some(vec![])
    } else {
      None
    }
  }
}

/*
impl<D> IntoMatchValue<D> for Name {
  fn into_match_value(&self) -> MatchValue<D> {
    MatchValue::Name(self.clone())
  }
}
 */
