use std::ptr::NonNull;

use crate::op::Op;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
  Const(Constant),
  Node(DefRef),
  Argument(NonNull<str>),
  Label(NonNull<str>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefRef(pub NonNull<Op>, pub usize);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Constant {
  Bool(bool),
  Int(i64),
  Uint(u64),
  String(String),
}
