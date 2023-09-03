use fcir::symbol::{Name, Symbol};

use crate::pattern::MatchValue;

#[derive(Debug, Clone, PartialEq)]
pub struct OpTemplate<D>(pub Insert<Name>, pub Vec<Insert<MatchValue<D>>>);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Insert<T> {
  Var(Symbol),
  Lit(T),
}
