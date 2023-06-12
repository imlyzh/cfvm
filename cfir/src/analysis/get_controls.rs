use std::ptr::NonNull;

use crate::{
  function::*, control::Control,
};

pub trait GetControls {
  fn get_controls(&self) -> Vec<NonNull<Control>>;
}

impl GetControls for Func {
  #[inline(always)]
  fn get_controls(&self) -> Vec<NonNull<Control>> {
    self.controls.clone()
  }
}
