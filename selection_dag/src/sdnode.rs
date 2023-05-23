use std::ptr::NonNull;

use cfvm_common::constant::ConstantValue;

use crate::basic_block::BasicBlock;

pub struct SDNode {
  pub inst: NonNull<str>,
  // pub cond: NonNUll<BasicBlock>,
  pub ops:  Vec<SDValue>,
}

#[repr(C)]
pub enum SDValue {
  Const(ConstantValue),
  Reg(Reg),
  Control(NonNull<BasicBlock>),
  Node(NonNull<SDNode>),
}

#[repr(C)]
pub struct Reg {
  pub reg_type: RegType,
  pub index:    usize,
}

#[repr(C)]
pub enum RegType {
  Virtual {},
  Physics {},
}
