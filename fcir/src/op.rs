use std::collections::HashMap;

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

#[derive(Debug, Clone, Hash, PartialEq, Eq)] // fixme: Hash
pub struct OpHand(pub *const Op);

impl AsRef<Op> for OpHand {
  fn as_ref(&self) -> &Op {
    unsafe { self.0.as_ref().unwrap() }
  }
}

pub type Space = Vec<OpHand>;
