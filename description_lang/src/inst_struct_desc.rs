use std::collections::HashMap;

/// example:
/// ```
/// sip(Register(r)) = (r)
/// sip(Literal(l)) = (0x114, l)
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MappingRecord<'a>(pub HashMap<&'a str, MappingList<'a>>);

pub type MappingList<'a> = Vec<InstStructMapping<'a>>;

/// name(pattern...) = (construct)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstStructMapping<'a>(pub &'a str, pub Params<'a>, pub Vec<InstConstruct<'a>>);

pub type Params<'a> = Vec<InstPattern<'a>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstPattern<'a> {
  Predicate(&'a str, Vec<&'a str>),
  // Literal(Vec<u8>),
  Constant(&'a str),
  Var(&'a str),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstConstruct<'a> {
  MappingCall(&'a str, Params<'a>),
  Literal(Vec<u8>),
  Var(&'a str),
}
