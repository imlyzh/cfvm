use std::ptr::NonNull;

use crate::data::Data;

///# Region

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct Region(pub Vec<ControlOrigin>);

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub enum ControlOrigin {
  Jump(NonNull<Region>),
  Branch(NonNull<Region>, If, bool),
}

///# Control

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct Control {
  pub region_source: NonNull<Region>,
  pub control:       ControlInst,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub enum ControlInst {
  Jump,
  // IndirectJump(Data),
  If(If),
  Return(Data),
  Unreachable,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct If(pub Data);
