use std::ptr::NonNull;

use cfir::function::FrameInfo;
use cfvm_common::types::FunctionType;

use crate::basic_block::BasicBlock;

#[repr(C)]
pub struct Func {
  pub name: NonNull<str>,
  pub type_info: FunctionType,
  pub frameinfo: FrameInfo,
  pub bbs: Vec<BasicBlock>,
  pub start: usize,
}
