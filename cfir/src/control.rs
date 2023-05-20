use crate::data::Data;


#[repr(C)]
pub struct Region (pub Vec<ControlOrigin>);

#[repr(C)]
pub struct Control {
  pub region_source: *const Region,
  pub control: ControlInst,
}

#[repr(C)]
pub enum ControlInst {
  Jump,
  // Jump(Option<Data>),
  If(If),
  Return(Data),
  Unreachable,
}

#[repr(C)]
pub enum ControlOrigin {
  Jump(*const Region),
  Branch(*const Region, If, bool),
}

#[repr(C)]
pub struct If (pub Data);
