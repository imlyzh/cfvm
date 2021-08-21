use std::sync::Arc;

use sexpr_ir::gast::Handle;

use super::{MutHandle, handles::{LabelHandle, LocalHandle, LocalSymbol, ValueHandle}, types::{FloatType, IntType, Type}};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum BranchOp {
    IfNil,
    IfNonNil,
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
    False,
    Oeq,
    Oge,
    Ogt,
    Ole,
    Olt,
    One,
    Ord,
    True,
    Ueq,
    Uge,
    Ugt,
    Ule,
    Ult,
    Une,
    Uno,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Index {
    Index(u64),
    Symbol(Handle<String>),
}

pub type IndexList = Vec<Index>;


#[derive(Debug, Clone)]
pub enum Operator {
    Alloca(Type, ValueHandle),
    GetPtr(ValueHandle),
    Load(Type, ValueHandle),
    Cast(Type, ValueHandle),
    Add(ValueHandle, ValueHandle),
    FAdd(ValueHandle, ValueHandle),
    Sub(ValueHandle, ValueHandle),
    FSub(ValueHandle, ValueHandle),
    Mul(ValueHandle, ValueHandle),
    FMul(ValueHandle, ValueHandle),
    UDiv(ValueHandle, ValueHandle),
    SDiv(ValueHandle, ValueHandle),
    URem(ValueHandle, ValueHandle),
    SRem(ValueHandle, ValueHandle),
    FRem(ValueHandle, ValueHandle),
    Shl(ValueHandle, ValueHandle),
    LShr(ValueHandle, ValueHandle),
    AShr(ValueHandle, ValueHandle),
    And(ValueHandle, ValueHandle),
    Or(ValueHandle, ValueHandle),
    Xor(ValueHandle, ValueHandle),
    GetValue(ValueHandle, IndexList, ValueHandle),
    SetValue(ValueHandle, IndexList, ValueHandle),
    GetItem(ValueHandle, ValueHandle, ValueHandle),
    SetItem(ValueHandle, ValueHandle, ValueHandle),
    Trunc(ValueHandle, IntType),
    ZExt(ValueHandle, IntType),
    SExt(ValueHandle, IntType),
    FTrunc(ValueHandle, FloatType),
    FExt(ValueHandle, FloatType),
    ICmp(ICmpOp, ValueHandle, ValueHandle),
    FCmp(FCmpOp, ValueHandle, ValueHandle),
    PhiInst(Vec<(ValueHandle, LabelHandle)>),
    Call(ValueHandle, Vec<ValueHandle>),
}

type IsVolatile = bool;

#[derive(Debug, Clone)]
pub struct BindMetadata {
    pub is_atomic: bool,
    // pub is_volatile: bool,
    pub is_mutable: bool,
}


#[derive(Debug, Clone)]
pub enum Instruction {
    Store(LocalHandle, ValueHandle, IsVolatile),
    BindOperator(LocalSymbol, MutHandle<BindMetadata>, MutHandle<Operator>),
    Operator(MutHandle<Operator>),
}

#[derive(Debug, Clone)]
pub enum Terminator {
    Ret(Option<ValueHandle>),
    Branch(BranchOp, ValueHandle, LabelHandle, LabelHandle),
    Cond(Vec<(ValueHandle, LabelHandle)>),
    Switch(ValueHandle, Arc<(u64, LabelHandle)>),
    Unrechable,
}
