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
  // Input(),
  TypeCast(*const TypeCast),
  PriOp(*const PriOp),
  BinOp(*const BinOp),
  AddrOp(*const AddrOp),
  // Cmp(*const Cmp),
  Phi(*const Phi),
  Effect(*const Effect),
}

#[repr(C)]
pub enum PriOp {
  Unimplmention,
  // Trunc(Data, IntType),
  // ZExt(Data, IntType),
  // SExt(Data, IntType),
  // FTrunc(Data, FloatType),
  // FExt(Data, FloatType),
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
pub enum AddrOp {
  RuntimeArrayItem(Data, Data),
  ArrayItem(Data, usize),
  TupleItem(Data, usize),
  // StructItem(Data, Symbol),
}

#[repr(C)]
pub struct Cmp {
  pub data0: Data,
  pub data1: Data,
  pub opcode: CmpOp,
}

#[repr(C)]
pub enum CmpOp {
  ICmp(ICmpOp),
  FCmp(FCmpOp),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ICmpOp {
  Eq,
  Ne,
  Sge,
  Sgt,
  Sle,
  Slt,
  Uge,
  Ugt,
  Ule,
  Ult,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FCmpOp {
  Oeq,
  Oge,
  Ogt,
  Ole,
  Olt,
  One,
  // Ord,
  // Ueq,
  // Uge,
  // Ugt,
  // Ule,
  // Ult,
  // Une,
  // Uno,
  // False,
  // True,
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
