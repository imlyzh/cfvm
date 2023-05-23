use control::{Control, Region};
use data::Data;
use effect::Effect;

pub mod control;
pub mod data;
pub mod effect;
pub mod function;

pub trait GetRegion {
  fn get_region(&self);
}

#[repr(C)]
pub enum Node {
  Data(*const Data),
  Region(*const Region),
  Control(*const Control),
  Effect(*const Effect),
}

#[cfg(test)]
mod tests {}
