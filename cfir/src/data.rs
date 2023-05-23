use std::ptr::NonNull;

use cfvm_common::{
  constant::ConstantValue,
  types::{FloatType, IntType, Type},
};

use crate::{
  control::Region,
  effect::Effect,
  function::{Func, Input},
  GetRegions,
};

#[repr(C)]
pub struct Data {
  pub region_source: NonNull<Region>,
  pub data: DataInst,
}

impl GetRegions for Data {
  fn get_regions(&self) -> Vec<NonNull<Region>> {
    let mut r = self.data.get_regions();
    r.push(self.region_source);
    r
  }
}

#[repr(C)]
pub enum DataInst {
  Const(Box<Type>, ConstantValue),
  Alloc(NonNull<StackAlloc>),
  Input(NonNull<Input>),
  TypeCast(NonNull<TypeCast>),
  PriOp(NonNull<PriOp>),
  BinOp(NonNull<BinOp>),
  AddrOp(NonNull<AddrOp>),
  // Cmp(NonNull<Cmp>),
  Phi(NonNull<Phi>),
  Effect(NonNull<Effect>),
}

impl GetRegions for DataInst {
  fn get_regions(&self) -> Vec<NonNull<Region>> {
    match self {
      DataInst::Const(_, _) | DataInst::Alloc(_) | DataInst::Input(_) => vec![],
      DataInst::TypeCast(d) => unsafe { d.as_ref() }.get_regions(),
      DataInst::PriOp(d) => unsafe { d.as_ref() }.get_regions(),
      DataInst::BinOp(d) => unsafe { d.as_ref() }.get_regions(),
      DataInst::AddrOp(d) => unsafe { d.as_ref() }.get_regions(),
      DataInst::Phi(d) => unsafe { d.as_ref() }.get_regions(),
      DataInst::Effect(d) => todo!(),
    }
  }
}

#[repr(C)]
pub struct StackAlloc {
  pub func:  NonNull<Func>,
  pub name:  NonNull<str>,
  pub type_: NonNull<Type>,
}

#[repr(C)]
pub enum PriOp {
  Trunc(Data, IntType),
  ZExt(Data, IntType),
  SExt(Data, IntType),
  FTrunc(Data, FloatType),
  FExt(Data, FloatType),
}

impl GetRegions for PriOp {
  fn get_regions(&self) -> Vec<NonNull<Region>> {
    match self {
      PriOp::Trunc(d, _)
      | PriOp::ZExt(d, _)
      | PriOp::SExt(d, _)
      | PriOp::FTrunc(d, _)
      | PriOp::FExt(d, _) => d.get_regions(),
    }
  }
}

#[repr(C)]
pub struct BinOp {
  pub data0:  Data,
  pub data1:  Data,
  pub opcode: Opcode,
}

impl GetRegions for BinOp {
  fn get_regions(&self) -> Vec<NonNull<Region>> {
    let mut r = self.data0.get_regions();
    r.append(&mut self.data1.get_regions());
    r
  }
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
  // icmp
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
  // fcmp
  Oeq,
  Oge,
  Ogt,
  Ole,
  Olt,
  One,
  // unsupported fcmp
  /*
  Ord,
  Ueq,
  Uge,
  Ugt,
  Ule,
  Ult,
  Une,
  Uno,
  False,
  True,
  // */
}

#[repr(C)]
pub enum AddrOp {
  RuntimeArrayItem(Data, Data),
  ArrayItem(Data, usize),
  TupleItem(Data, usize),
  // StructItem(Data, Symbol),
}

impl GetRegions for AddrOp {
  fn get_regions(&self) -> Vec<NonNull<Region>> {
    match self {
      AddrOp::RuntimeArrayItem(data0, data1) => {
        let mut r = data0.get_regions();
        r.append(&mut data1.get_regions());
        r
      },
      AddrOp::ArrayItem(data, _) | AddrOp::TupleItem(data, _) => data.get_regions(),
    }
  }
}

#[repr(C)]
pub struct TypeCast {
  pub data:   Data,
  pub astype: Type,
}

impl GetRegions for TypeCast {
  fn get_regions(&self) -> Vec<NonNull<Region>> {
    self.data.get_regions()
  }
}

#[repr(C)]
pub struct Phi {
  pub data: Vec<Data>,
}

impl GetRegions for Phi {
  fn get_regions(&self) -> Vec<NonNull<Region>> {
    self.data.iter().flat_map(Data::get_regions).collect()
  }
}

/*
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
  // unsupported fcmp
  /*
  Ord,
  Ueq,
  Uge,
  Ugt,
  Ule,
  Ult,
  Une,
  Uno,
  False,
  True,
  // */
}
// */
