use crate::function::Func;

pub trait Builder<T> {
  fn build_from(i: &T) -> Self;
}

impl Builder<cfir::function::Func> for Func {
  fn build_from(i: &cfir::function::Func) -> Self {
    todo!()
  }
}
