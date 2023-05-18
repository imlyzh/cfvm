use control::{Region, Control, Branch};
use data::Data;
use effect::Effect;

pub mod data;
pub mod control;
pub mod effect;


pub trait GetRegion {
  fn get_region(&self);
}


#[repr(C)]
pub enum Node {
  Data(*const Data),
  Region(*const Region),
  Control(*const Control),
  Branch(*const Branch),
  Effect(*const Effect),
}



#[cfg(test)]
mod tests {

}
