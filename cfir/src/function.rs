use crate::{data::Data, control::Region, effect::Effect};



#[repr(C)]
pub struct Func {
  pub name: *const str,
  pub regions: Vec<*const Region>,
  pub datas: Vec<*const Data>,
  pub effects: Vec<*const Effect>,
}
