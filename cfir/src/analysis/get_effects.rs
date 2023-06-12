use std::ptr::NonNull;

use crate::{
  effect::*,
  function::*,
};

pub trait GetEffects {
  fn get_effects(&self) -> Vec<NonNull<Effect>>;
}

impl GetEffects for Func {
  #[inline(always)]
  fn get_effects(&self) -> Vec<NonNull<Effect>> {
    self.effects.clone()
  }
}
