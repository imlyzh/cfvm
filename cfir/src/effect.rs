use crate::{control::Region, data::Data};



#[repr(C)]
pub struct Effect {
  pub region_source: *const Region,
  pub effect_source: *const Effect,
  pub effect: EffectInst,
}

#[repr(C)]
pub enum EffectInst {
  Read(Read),
  Write(Write),
  Call(Call),
}

pub struct Barrier {}

pub struct Read {
  pub ptr: Data,
}

pub struct Write {
  pub ptr: Data,
  pub value: Data,
}

pub struct Call {
  // pub function: Func,
  pub args: Vec<Data>,
}