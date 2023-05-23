use std::ptr::NonNull;

// literal values

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum SimpleValue {
  FloatNumber(f64),
  Number(u64),
  // Char(char),
  Bool(bool),
  Vector(VectorValue),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct RecordValue(pub Vec<(Option<NonNull<str>>, ConstantValue)>);

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct VectorValue(pub Vec<SimpleValue>);

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ArrayValue(pub Vec<ConstantValue>);

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct StringLit(pub NonNull<str>);

#[derive(Debug, Clone, PartialEq, PartialOrd)]
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
