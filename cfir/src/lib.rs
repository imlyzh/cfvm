use control::{Region, Control};
use data::Data;
use effect::Effect;

pub mod data;
pub mod control;
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
mod tests {

}
