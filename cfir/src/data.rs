use crate::{control::Region, effect::Effect};

#[repr(C)]
pub struct Data {
  pub region_source: *const Region,
  pub data: DataInst,
}

#[repr(C)]
pub enum DataInst {
  Const(),
  Effect(*const Effect),
  BinOp(*const BinOp),
  TypeCast(*const TypeCast),
  Phi(*const Phi),
}

#[repr(C)]
pub struct BinOp {
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
  pub data: Data,
  //pub astype: Type,
}

#[repr(C)]
pub struct Phi {
  pub data: Vec<Data>,
}