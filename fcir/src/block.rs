use std::{collections::HashMap, ptr::NonNull};

use crate::op::Op;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Region {
  pub blocks: HashMap<Option<NonNull<str>>, Block>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block(pub Vec<Op>);
