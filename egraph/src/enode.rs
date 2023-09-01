use std::{cell::RefCell, rc::Rc};

use fcir::{
  block::Region,
  op::Attr,
  symbol::{Name, Symbol},
  types::FuncType,
  value::{Argument, Constant},
};

use crate::eclass::Id;

/// EOp from/into Op
#[derive(Debug, Clone)]
pub struct EOp<D> {
  pub opcode: Name,
  pub uses: Vec<Id<D>>,
  pub attr: Attr,
  pub region: Region,
  pub sign: FuncType,
}

impl<D> PartialEq for EOp<D> {
  fn eq(&self, other: &Self) -> bool {
    self.opcode == other.opcode
      && self.uses == other.uses
      && self.attr == other.attr
      && self.region == other.region
      && self.sign == other.sign
  }
}

#[derive(Debug)]
pub struct EOpHand<D>(pub Rc<RefCell<EOp<D>>>);

impl<D> Clone for EOpHand<D> {
  fn clone(&self) -> Self {
    Self(self.0.clone())
  }
}

impl<D> PartialEq for EOpHand<D> {
  fn eq(&self, other: &Self) -> bool {
    self.0 == other.0
  }
}

impl<D> EOpHand<D> {
  pub fn new(value: EOp<D>) -> Self {
    Self(Rc::new(RefCell::new(value)))
  }
}

/// ENode from/into Value
#[derive(Debug, Clone)]
pub enum ENode<D> {
  Const(Constant),
  Use(EOpHand<D>),
  Argument(Argument),
  Label(Symbol),
}

impl<D> PartialEq for ENode<D> {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Self::Const(l0), Self::Const(r0)) => l0 == r0,
      (Self::Use(l0), Self::Use(r0)) => l0 == r0,
      (Self::Argument(l0), Self::Argument(r0)) => l0 == r0,
      (Self::Label(l0), Self::Label(r0)) => l0 == r0,
      _ => false,
    }
  }
}

impl<D> From<&Constant> for ENode<D> {
  fn from(value: &Constant) -> Self {
    ENode::Const(value.clone())
  }
}

impl<D> From<&Argument> for ENode<D> {
  fn from(value: &Argument) -> Self {
    ENode::Argument(value.clone())
  }
}

impl<D> From<&Symbol> for ENode<D> {
  fn from(value: &Symbol) -> Self {
    ENode::Label(value.clone())
  }
}
