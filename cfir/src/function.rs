use std::ptr::NonNull;

use cfvm_common::types::FunctionType;

use crate::{
  control::{Control, Region},
  data::{Data, StackAlloc},
  effect::Effect,
  GetRegions,
};

#[repr(C)]
pub struct Func {
  pub name:      NonNull<str>,
  pub type_info: FunctionType,
  pub frameinfo: FrameInfo,
  // bodys
  // pub body: Vec<Node>,
  // pub regions:   Vec<NonNull<Region>>,
  // pub datas:     Vec<NonNull<Data>>,
  pub controls:  Vec<NonNull<Control>>,
  pub effects:   Vec<NonNull<Effect>>,
}

impl GetRegions for Func {
  fn get_regions(&self) -> Vec<NonNull<Region>> {
    // self.controls.iter().map(|x| unsafe {x.as_ref()}.get_regions())
    todo!()
  }
}

/*
#[repr(C)]
pub enum Node {
  Data(NonNull<Data>),
  Region(NonNull<Region>),
  Control(NonNull<Control>),
  Effect(NonNull<Effect>),
}
//  */

#[repr(C)]
pub struct Input {
  pub func:   NonNull<Func>,
  pub name:   NonNull<str>,
  pub offset: usize,
}

#[repr(C)]
pub struct FrameInfo(pub Vec<NonNull<StackAlloc>>);
