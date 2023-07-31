use std::ptr::NonNull;

// literal values

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum SimpleValue {
  Number(u64),
  FloatNumber(u64),
  // Char(char),
  Bool(bool),
  Vector(VectorValue),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct RecordValue(pub Vec<(Option<NonNull<str>>, ConstantValue)>);

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct VectorValue(pub Vec<SimpleValue>);

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ArrayValue(pub Vec<ConstantValue>);

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct StringLit(pub NonNull<str>);

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ConstantValue {
  SimpleValue(SimpleValue),
  ArrayValue(ArrayValue),
  RecordValue(RecordValue),
  StringLit(StringLit),
}

impl SimpleValue {
  pub fn get_bool_lit(&self) -> Option<bool> {
    if let SimpleValue::Bool(b) = self {
      Some(*b)
    } else {
      None
    }
  }
}

impl ConstantValue {
  pub fn get_bool_lit(&self) -> Option<bool> {
    if let ConstantValue::SimpleValue(s) = self {
      s.get_bool_lit()
    } else {
      None
    }
  }
}
