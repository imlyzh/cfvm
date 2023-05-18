use crate::basic_block::BasicBlock;


#[repr(C)]
pub enum SDNode {
  Const(),
  Inst(*const Inst),
  // InstBundle(*const InstBundle),
}

#[repr(C)]
pub struct Inst {
  pub inst: *const str,
  pub cond: *const BasicBlock,
  pub output: Vec<SDNode>,
  pub input: Vec<SDNode>,
  pub control: Vec<*const BasicBlock>,
}

/*
#[repr(C)]
pub struct InstBundle(pub Vec<Inst>);
//  */