use std::{collections::HashMap, hash::Hash, rc::Rc};

use crate::{
  block::Region,
  symbol::{Name, Symbol},
  types::FuncType,
  value::{Constant, Value},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Op {
  pub opcode: Name,
  // pub defs: Vec<Symbol>,
  pub uses: Vec<Value>,
  pub attr: Attr,
  pub region: Region,
  pub sign: FuncType,
}

pub type Attr = HashMap<Symbol, Constant>;

#[derive(Debug, Clone, PartialEq, Eq)] // fixme: Hash
pub struct OpHand(pub Rc<Op>);

impl OpHand {
  pub fn new(op: Op) -> Self {
    Self(Rc::new(op))
  }
}

impl Hash for OpHand {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    Rc::into_raw(self.0.clone()).hash(state);
  }
}

impl AsRef<Op> for OpHand {
  fn as_ref(&self) -> &Op {
    // unsafe { self.0.as_ref().unwrap() }
    self.0.as_ref()
  }
}

pub type Space = Vec<OpHand>;
