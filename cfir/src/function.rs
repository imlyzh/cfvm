use std::collections::HashMap;

use crate::{data::{Data, StackAlloc}, control::Region, effect::Effect};



#[repr(C)]
pub struct Func {
  pub name: *const str,
  // pub return_type: Type,
  pub inputs: *const Inputs,
  pub frameinfo: *const FrameInfo,
  // bodys
  pub regions: Vec<*const Region>,
  pub datas: Vec<*const Data>,
  pub effects: Vec<*const Effect>,
}


#[repr(C)]
pub struct Inputs {
  pub login_offset_map: HashMap<*const str, usize>,
  pub inputs: Vec<*const Input>,
}

#[repr(C)]
pub struct Input {
  pub inputs: *const Inputs,
  pub name: *const str,
  // pub type_: Type,
}

#[repr(C)]
pub struct FrameInfo(Vec<*const StackAlloc>);
