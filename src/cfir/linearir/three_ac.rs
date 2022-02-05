

use crate::cfir::{types::TypeBindAttr, handles::{SymbolRef, LTMHand, SimpleValue}, linearir::{IndexList, ICmpOp, FCmpOp}};

use super::{super::{
    handles::{LabelSymbol},
    types::{FloatType, IntType, Type},
    // MutHandle,
}, BranchOp};

#[derive(Debug, Clone)]
pub enum Operator {
    Const(TypeBindAttr, SimpleValue),
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
    Trunc(SymbolRef, IntType),
    ZExt(SymbolRef, IntType),
    SExt(SymbolRef, IntType),
    FTrunc(SymbolRef, FloatType),
    FExt(SymbolRef, FloatType),
    ICmp(ICmpOp, SymbolRef, SymbolRef),
    FCmp(FCmpOp, SymbolRef, SymbolRef),
    Call(SymbolRef, Vec<SymbolRef>),
}

#[derive(Debug, Clone)]
pub enum Store {
    Store(LabelSymbol, SymbolRef, TypeBindAttr),
    SetValue(SymbolRef, IndexList, SymbolRef),
    SetItem(SymbolRef, SymbolRef, SymbolRef),
}

#[derive(Debug, Clone)]
pub struct BindOperator(
    pub LabelSymbol,
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
pub struct Ret(pub Option<SymbolRef>);

#[derive(Debug, Clone)]
pub struct Branch(
    pub BranchOp,
    pub SymbolRef,
    pub LabelSymbol,
    pub LabelSymbol,
);

#[derive(Debug, Clone)]
pub enum Terminator {
    Ret(Ret),
    Branch(Branch),
    Jump(LabelSymbol),
    Jump2value(SymbolRef),
    Unrechable,
}

