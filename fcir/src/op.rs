use std::{collections::HashMap, ptr::NonNull};

use crate::{block::Region, types::FuncType, value::{Value, Constant}};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Op {
  pub opcode: NonNull<str>,
  pub output: Vec<NonNull<str>>,
  pub input:  Vec<Value>,
  pub attr:   HashMap<NonNull<str>, Constant>,
  pub ragion: Region,
  pub sign:   FuncType,
}
