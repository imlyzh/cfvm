use std::{collections::HashMap, ptr::NonNull};

use cfir::{
  analysis::{get_data_dep::GetDataDep, get_region::body2regions},
  control::Region,
};

use crate::{basic_block::BasicBlock, function::Func};

pub trait Builder<T> {
  fn build_from(i: &T) -> Self;
}

impl Builder<cfir::function::Func> for Func {
  fn build_from(i: &cfir::function::Func) -> Self {
    // let name = i.name;
    // let type_info = &i.type_info;
    // let frameinfo = &i.frameinfo;

    let effects = &i.effects;
    let controls = &i.controls;
    let datas = i.get_data_dep();
    let regions = body2regions(controls, effects, &datas);

    let bbs: HashMap<NonNull<Region>, BasicBlock> = regions
      .into_iter()
      .map(|x| (x, BasicBlock::default()))
      .collect();

    todo!()
  }
}
