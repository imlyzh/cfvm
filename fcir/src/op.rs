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
  pub defs: Vec<Symbol>,
  pub uses: Vec<Value>,
  pub attr: HashMap<Symbol, Constant>,
  pub ragion: Region,
  pub sign: FuncType,
}

pub type Space = Vec<Op>;
