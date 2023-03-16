use crate::control::Region;

#[repr(C)]
pub enum Data {
  Const(),
  BinOp(*const BinOp),
  TypeCast(*const TypeCast),
  Phi(*const Phi),
}

#[repr(C)]
pub struct BinOp {
  pub control: *const Region,
  pub data0: Data,
  pub data1: Data,
  pub opcode: Opcode,
}

#[repr(C)]
pub enum Opcode {
  Add,
  Sub,
  Mul,
  Div,
}

#[repr(C)]
pub struct TypeCast {
  pub control: *const Region,
  pub data: Data,
  //pub astype: Type,
}

#[repr(C)]
pub struct Phi {
  pub control: *const Region,
  pub data: Vec<Data>,
}