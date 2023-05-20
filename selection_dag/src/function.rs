use crate::basic_block::BasicBlock;

#[repr(C)]
pub struct Func {
  // pub name: *const str,
  pub bbs: Vec<*const BasicBlock>,
  pub start: usize,
}
