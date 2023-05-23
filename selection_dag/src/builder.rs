use std::{collections::HashMap, ptr::NonNull};

use cfir::control::Region;

use crate::{basic_block::BasicBlock, function::Func};

pub trait Builder<T> {
  fn build_from(i: T) -> Self;
}

impl Builder<cfir::function::Func> for Func {
  fn build_from(i: cfir::function::Func) -> Self {
    let name = i.name;
    let type_info = i.type_info;
    let frameinfo = i.frameinfo;

    /*
    let mut bbs: HashMap<NonNull<Region>, BasicBlock> = i
      .regions
      .iter()
      .map(|x| (*x, BasicBlock::default()))
      .collect();
     */
    todo!()
  }
}
