use std::ptr::NonNull;

use crate::{control::Region, data::Data};



#[repr(C)]
pub struct Effect {
  pub region_source: NonNull<Region>,
  pub effect_source: NonNull<Effect>,
  pub effect: EffectInst,
}

#[repr(C)]
pub enum EffectInst {
  // Barrier,
  // MachineCode(NonNull< MachineCode),
  Read {
    ptr: Data,
  },
  Write {
    ptr: Data,
    value: Data,
  },
  Call {
    // pub func: Func,
    args: Vec<Data>,
  },
  IndirectCall {
    func: Data,
    args: Vec<Data>,
  },
}

/*
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
//  */