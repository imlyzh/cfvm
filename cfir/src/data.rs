use crate::{control::Region, effect::Effect};

#[repr(C)]
pub struct Data {
  pub region_source: *const Region,
  pub data: DataInst,
}

#[repr(C)]
pub enum DataInst {
  Const(),
  // Alloc(),
  TypeCast(*const TypeCast),
  PriOp(*const PriOp),
  BinOp(*const BinOp),
  // Cmp(*const Cmp),
  Phi(*const Phi),
  Effect(*const Effect),
}

#[repr(C)]
pub enum PriOp {
  // Trunc(Data, IntType),
  // ZExt(Data, IntType),
  // SExt(Data, IntType),
  // FTrunc(Data, FloatType),
  // FExt(Data, FloatType),

  GetValue(Data, usize),
  GetItem(Data, usize),
  SetValue(Data, usize, Data),
  SetItem(Data, usize, Data),
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
  FAdd,
  Sub,
  FSub,
  Mul,
  FMul,
  UDiv,
  SDiv,
  URem,
  SRem,
  FRem,
  Shl,
  LShr,
  AShr,
  And,
  Or,
  Xor,
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
