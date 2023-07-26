use crate::symbol::Symbol;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
  Const(Constant),
  Use(Symbol),
  Argument(Symbol),
  Label(Symbol),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Constant {
  Bool(bool),
  Int(i64),
  Uint(u64),
  String(String),
}
