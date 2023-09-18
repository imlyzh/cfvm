use std::collections::HashMap;

use crate::{op::Space, symbol::Symbol, types::Type};

pub type Region = HashMap<Option<Symbol>, Block>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block(pub Option<Symbol>, pub HashMap<Symbol, Type>, pub Space);
