use crate::basic_block::BasicBlock;

pub struct SDNode {
  pub inst: *const str,
  // pub cond: *const BasicBlock,
  pub ops:  Vec<SDValue>,
}

#[repr(C)]
pub enum SDValue {
  Const(),
  Reg(Reg),
  Control(*const BasicBlock),
  Node(*const SDNode),
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
