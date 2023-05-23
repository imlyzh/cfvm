use std::ptr::NonNull;

use control::Region;

pub mod control;
pub mod data;
pub mod effect;
pub mod function;

pub trait GetRegions {
  fn get_regions(&self) -> Vec<NonNull<Region>>;
}
