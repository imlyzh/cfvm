use crate::op::Op;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block(pub Vec<Op>);
