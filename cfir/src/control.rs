use std::ptr::NonNull;

use crate::data::Data;

///# Region

#[repr(C)]
pub struct Region(pub Vec<ControlOrigin>);

#[repr(C)]
pub enum ControlOrigin {
  Jump(NonNull<Region>),
  Branch(NonNull<Region>, If, bool),
}

///# Control

#[repr(C)]
pub struct Control {
  pub region_source: NonNull<Region>,
  pub control:       ControlInst,
}

#[repr(C)]
pub enum ControlInst {
  Jump,
  // IndirectJump(Data),
  If(If),
  Return(Data),
  Unreachable,
}

#[repr(C)]
pub struct If(pub Data);
