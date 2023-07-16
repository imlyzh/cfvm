use std::{collections::HashMap, ptr::NonNull};

use crate::{block::Block, types::FuncType, value::Value};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Op {
  pub opcode: NonNull<str>,
  pub output: Vec<NonNull<str>>,
  pub input:  Vec<Value>,
  pub attr:   HashMap<NonNull<str>, ()>,
  pub blocks: HashMap<Option<NonNull<str>>, Block>,
  pub sign:   FuncType,
}
