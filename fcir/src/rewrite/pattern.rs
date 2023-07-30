use std::collections::HashMap;

use crate::{symbol::Symbol, types::FuncType, value::Value};

pub type MatchResult1 = Vec<(Symbol, Value)>;
pub type MatchResult2 = HashMap<Symbol, Value>;
/*
pub struct CombinePat {
  pub opcode: NamePat,
  pub defs: Vec<ItemPat<Symbol>>,
  pub uses: Vec<ItemPat<Value>>,
  // pub ragion: Option<Catch>,
  pub sign: Option<FuncType>,
}
// */

pub struct OpPat {
  pub opcode: NamePat,
  pub defs: Vec<ItemPat<Symbol>>,
  pub uses: Vec<ItemPat<Value>>,
  // pub ragion: Option<Catch>,
  pub sign: Option<FuncType>,
}

pub struct NamePat(pub Option<Symbol>, pub Option<Symbol>);

pub enum ItemPat<T> {
  Catch(Box<CatchExpr>),
  Literal(T),
}

pub type CatchExpr = (Option<Symbol>, Option<ValueType>);

pub enum ValueType {
  Use,
  Argument,
  Label,
}
