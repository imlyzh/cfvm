use std::{collections::HashMap, ptr::NonNull};

use crate::{block::Region, types::FuncType, value::{Value, Constant}, name::Name};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Op {
  pub opcode: Name,
  pub output: Vec<NonNull<str>>,
  pub input:  Vec<Value>,
  pub attr:   HashMap<NonNull<str>, Constant>,
  pub ragion: Region,
  pub sign:   FuncType,
}
