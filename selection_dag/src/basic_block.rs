use crate::sdnode::SDNode;

#[repr(C)]
#[derive(Debug, Default, Clone)]
pub struct BasicBlock {
  // pub name: *const str,
  pub insts: Vec<SDNode>,
}
