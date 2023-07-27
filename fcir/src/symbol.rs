use std::ptr::NonNull;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Symbol(pub NonNull<str>);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Name(pub Option<Symbol>, pub Symbol);
