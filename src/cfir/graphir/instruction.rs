use std::collections::BTreeSet;

use crate::cfir::types::AllocaType;

use super::{
    handles::{LabelHandle, LocalHandle, LocalSymbol, SimpleValue, Symbol, ValueHandle},
    types::{FloatType, IntType, Type},
    MutHandle,
};

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
    Symbol(Symbol),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IndexList(pub Vec<Index>);

#[derive(Debug, Clone)]
pub enum Operator {
    Alloca(Option<AllocaType>, Type),
    GetPtr(ValueHandle, Option<IndexList>),
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
    GetValue(ValueHandle, IndexList),
    GetItem(ValueHandle, ValueHandle),
    SetValue(ValueHandle, IndexList, ValueHandle),
    SetItem(ValueHandle, ValueHandle, ValueHandle),
    Trunc(ValueHandle, IntType),
    ZExt(ValueHandle, IntType),
    SExt(ValueHandle, IntType),
    FTrunc(ValueHandle, FloatType),
    FExt(ValueHandle, FloatType),
    ICmp(ICmpOp, ValueHandle, ValueHandle),
    FCmp(FCmpOp, ValueHandle, ValueHandle),
    Phi(Vec<(LabelHandle, ValueHandle)>),
    Call(ValueHandle, Vec<ValueHandle>),
}

#[derive(Debug, Clone)]
pub struct Store(
    pub LocalHandle,
    pub ValueHandle,
    pub IsAtomic,
    pub IsVolatile,
);

#[derive(Debug, Clone)]
pub struct BindOperator(
    pub LocalSymbol,
    pub MutHandle<Operator>,
    pub IsAtomic,
    pub IsVolatile,
);

#[derive(Debug, Clone)]
pub enum Instruction {
    Store(Store),
    BindOperator(BindOperator),
    Operator(MutHandle<Operator>),
}

#[derive(Debug, Clone)]
pub struct Ret(pub Option<ValueHandle>);

#[derive(Debug, Clone)]
pub struct Branch(
    pub BranchOp,
    pub ValueHandle,
    pub LabelHandle,
    pub LabelHandle,
);

#[derive(Debug, Clone)]
pub struct Conds(pub Vec<(ValueHandle, LabelHandle)>, pub Option<LabelHandle>);

#[derive(Debug, Clone)]
pub struct Switch(pub ValueHandle, pub Vec<(SimpleValue, LabelHandle)>);

#[derive(Debug, Clone)]
pub enum Terminator {
    Ret(Ret),
    Branch(Branch),
    Conds(Conds),
    Switch(Switch),
    Unrechable,
}
