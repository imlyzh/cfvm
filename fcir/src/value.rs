use crate::{op::OpHand, symbol::Symbol};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
  Const(Constant),
  Use(OpHand),
  Argument(Argument),
  Label(Symbol),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Argument(pub Symbol, pub Option<Order>);

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Order {
  Def = 0,
  Use = 1,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Constant {
  Bool(bool),
  Int(i64),
  Uint(u64),
  String(String),
}
