#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Constant {
  Bool(bool),
  Int(i64),
  Uint(u64),
  String(String),
}

use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Symbol(pub Rc<String>);

impl Symbol {
  pub fn new(value: &str) -> Self {
    Symbol(Rc::new(value.to_string()))
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Name(pub Option<Symbol>, pub Symbol);
