use crate::data::Data;


#[repr(C)]
pub struct Region (pub Vec<JumpOrBranch>);

#[repr(C)]
pub enum Control {
  Jump(*const Region),
  If(*const If),
  Return(*const Region, Data),
}

#[repr(C)]
pub enum JumpOrBranch {
  Jump(*const Region),
  Branch(*const Branch),
}

#[repr(C)]
pub struct If (pub *const Region, pub Data);

#[repr(C)]
pub enum Branch {
  IfT(*const If),
  IfF(*const If),
}