use std::ptr::NonNull;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Name(Option<NonNull<str>>, NonNull<str>);
