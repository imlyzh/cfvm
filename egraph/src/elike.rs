use std::collections::HashMap;

use fcir::form::Form;

use crate::eclass::Id;

#[derive(Debug, Clone)]
pub struct ELike<D>(pub HashMap<Form, Id<D>>);

impl<D> Default for ELike<D> {
  fn default() -> ELike<D> {
    ELike(HashMap::new())
  }
}

impl<D> ELike<D> {
  fn new() -> ELike<D> {
    Default::default()
  }
}
