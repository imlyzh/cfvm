use crate::basic_block::BasicBlock;

#[repr(C)]
pub struct Func {
  // pub name: *const str,
  pub bbs: Vec<BasicBlock>,
  pub start: usize,
}