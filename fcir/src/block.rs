use std::collections::HashMap;

use crate::{op::Op, symbol::Symbol};

pub type Region = HashMap<Option<Symbol>, Block>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block(pub Vec<Symbol>, pub Vec<Op>);
