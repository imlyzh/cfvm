
/// name(pattern...) = (construct)
#[derive(Debug)]
pub struct InstStructMapping<'a> (pub &'a str, pub Vec<InstPattern<'a>>, pub Vec<InstConstruct<'a>>);

#[derive(Debug)]
pub enum InstPattern<'a> {
  Predicate(&'a str, Vec<&'a str>),
  // Literal(Vec<u8>),
  Constant(&'a str),
  Var(&'a str),
}

#[derive(Debug)]
pub enum InstConstruct<'a> {
  MappingCall(&'a str),
  Literal(Vec<u8>),
  Var(&'a str),
}