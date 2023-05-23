use crate::sdnode::SDNode;

#[repr(C)]
pub struct BasicBlock {
  // pub name: *const str,
  pub insts: Vec<SDNode>,
}
