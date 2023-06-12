use std::{collections::HashMap, ptr::NonNull};

use cfir::{
  analysis::{get_data_dep::GetDataDep, get_effects::GetEffects, get_region::GetRegions},
  control::Region,
};

use crate::{basic_block::BasicBlock, function::Func};

pub trait Builder<T> {
  fn build_from(i: &T) -> Self;
}

impl Builder<cfir::function::Func> for Func {
  fn build_from(i: &cfir::function::Func) -> Self {
    let name = i.name;
    let type_info = &i.type_info;
    let frameinfo = &i.frameinfo;

    let mut datas = i.get_data_dep();
    let mut effects = i.get_effects();

    let mut bbs: HashMap<NonNull<Region>, BasicBlock> = i
      .get_regions()
      .into_iter()
      .map(|x| (x, BasicBlock::default()))
      .collect();

    todo!()
  }
}
