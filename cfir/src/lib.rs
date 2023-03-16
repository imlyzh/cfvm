use control::{Region, Control, Branch};
use data::Data;

pub mod data;
pub mod control;
pub mod effect;


pub trait GetControl {
  fn get_control(&self);
}


#[repr(C)]
pub enum Node {
  Data(*const Data),
  Region(*const Region),
  Control(*const Control),
  Branch(*const Branch),
}



#[cfg(test)]
mod tests {

}
