use fcir::{
  block::Region,
  op::Attr,
  symbol::{Name, Symbol},
  types::FuncType,
  value::{Argument, Constant, Order, Value},
};

use crate::eclass::{EClass, Id};

/// EOp from/into Op
#[derive(Debug, Clone)]
pub struct EOp<D> {
  pub opcode: Name,
  pub uses: Vec<Id<D>>,
  pub attr: Attr,
  pub ragion: Region,
  pub sign: FuncType,
}

#[derive(Debug, Clone)]
pub struct EOpHand<D>(pub *const EOp<D>);

/// ENode from/into Value
#[derive(Debug, Clone)]
pub enum ENode<D> {
  Const(Constant),
  Use(Id<D>),
  Argument(Argument),
  Label(Symbol),
}

impl<D> From<Constant> for ENode<D> {
  fn from(value: Constant) -> Self {
    ENode::Const(value)
  }
}

impl<D> From<Argument> for ENode<D> {
  fn from(value: Argument) -> Self {
    ENode::Argument(value)
  }
}

impl<D> From<Symbol> for ENode<D> {
  fn from(value: Symbol) -> Self {
    ENode::Label(value)
  }
}
