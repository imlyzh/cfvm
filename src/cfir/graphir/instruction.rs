// use std::collections::BTreeSet;

use crate::cfir::{types::TypeBindAttr, handles::LTMHand};

use super::{super::{
    handles::{LabelSymbol, LocalSymbol, SimpleValue, Symbol, SymbolRef},
    types::{FloatType, IntType, Type},
    // MutHandle,
}, Value};

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
    Alloca(
        // TypeSymbol, Option<AllocaType>,
        TypeBindAttr,
        Option<SymbolRef>),
    GetPtr(SymbolRef, Option<IndexList>),
    Load(Type, SymbolRef),
    Cast(Type, SymbolRef),
    Add(SymbolRef, SymbolRef),
    FAdd(SymbolRef, SymbolRef),
    Sub(SymbolRef, SymbolRef),
    FSub(SymbolRef, SymbolRef),
    Mul(SymbolRef, SymbolRef),
    FMul(SymbolRef, SymbolRef),
    UDiv(SymbolRef, SymbolRef),
    SDiv(SymbolRef, SymbolRef),
    URem(SymbolRef, SymbolRef),
    SRem(SymbolRef, SymbolRef),
    FRem(SymbolRef, SymbolRef),
    Shl(SymbolRef, SymbolRef),
    LShr(SymbolRef, SymbolRef),
    AShr(SymbolRef, SymbolRef),
    And(SymbolRef, SymbolRef),
    Or(SymbolRef, SymbolRef),
    Xor(SymbolRef, SymbolRef),
    GetValue(SymbolRef, IndexList),
    GetItem(SymbolRef, SymbolRef),
    SetValue(SymbolRef, IndexList, SymbolRef),
    SetItem(SymbolRef, SymbolRef, SymbolRef),
    Trunc(SymbolRef, IntType),
    ZExt(SymbolRef, IntType),
    SExt(SymbolRef, IntType),
    FTrunc(SymbolRef, FloatType),
    FExt(SymbolRef, FloatType),
    ICmp(ICmpOp, SymbolRef, SymbolRef),
    FCmp(FCmpOp, SymbolRef, SymbolRef),
    Phi(Vec<(LabelSymbol, SymbolRef)>),
    Call(SymbolRef, Vec<SymbolRef>),
}

#[derive(Debug, Clone)]
pub struct Store(
    pub LocalSymbol,
    pub SymbolRef,
    pub TypeBindAttr,
);

#[derive(Debug, Clone)]
pub struct BindOperator(
    pub LocalSymbol,
    pub LTMHand<Operator>,
    // pub TypeBindAttr,
);

#[derive(Debug, Clone)]
pub enum Instruction {
    Store(Store),
    BindOperator(BindOperator),
    Operator(LTMHand<Operator>),
}

#[derive(Debug, Clone)]
pub struct Ret(pub Option<Value>);

#[derive(Debug, Clone)]
pub struct Branch(
    pub BranchOp,
    pub SymbolRef,
    pub LabelSymbol,
    pub LabelSymbol,
);

#[derive(Debug, Clone)]
pub struct Conds(pub Vec<(SymbolRef, LabelSymbol)>, pub Option<LabelSymbol>);

#[derive(Debug, Clone)]
pub struct Switch(pub SymbolRef, pub Vec<(SimpleValue, LabelSymbol)>);

#[derive(Debug, Clone)]
pub enum Terminator {
    Ret(Ret),
    Branch(Branch),
    Conds(Conds),
    Switch(Switch),
    Unrechable,
}
