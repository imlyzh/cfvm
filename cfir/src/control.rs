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
  If(*const If),
  Return(Data),
}

#[repr(C)]
pub enum ControlOrigin {
  Jump(*const Region),
  Branch(*const Branch),
}

#[repr(C)]
pub struct If (pub Data);

#[repr(C)]
pub enum Branch {
  IfT(*const If),
  IfF(*const If),
}