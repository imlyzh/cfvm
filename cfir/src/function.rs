use std::ptr::NonNull;

use cfvm_common::types::FunctionType;

use crate::{control::Control, data::StackAlloc, effect::Effect};

#[repr(C)]
pub struct Func {
  pub name: NonNull<str>,
  pub type_info: FunctionType,
  pub frameinfo: FrameInfo,
  // bodys
  // pub body: Vec<Node>,
  // pub regions:   Vec<NonNull<Region>>,
  // pub datas:     Vec<NonNull<Data>>,
  pub controls: Vec<NonNull<Control>>,
  pub effects: Vec<NonNull<Effect>>,
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
  pub func: NonNull<Func>,
  pub name: NonNull<str>,
  pub offset: usize,
}

#[repr(C)]
pub struct FrameInfo(pub Vec<NonNull<StackAlloc>>);
