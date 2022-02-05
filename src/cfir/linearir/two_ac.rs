
use crate::cfir::{types::TypeBindAttr, handles::{SymbolRef, LTMHand, SimpleValue}, linearir::{IndexList, ICmpOp, FCmpOp}};

use super::{super::{
    handles::{LabelSymbol},
    types::{FloatType, IntType, Type},
    // MutHandle,
}, BranchOp};

#[derive(Debug, Clone)]
pub enum Operator {
    Const(SymbolRef, TypeBindAttr, SimpleValue),
    GetPtr(SymbolRef, SymbolRef, Option<IndexList>),
    Load(SymbolRef, Type, SymbolRef),
    Cast(SymbolRef, Type, SymbolRef),
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
    GetValue(SymbolRef, SymbolRef, IndexList),
    GetItem(SymbolRef, SymbolRef, SymbolRef),
    SetValue(SymbolRef, IndexList, SymbolRef),
    SetItem(SymbolRef, SymbolRef, SymbolRef),
    Trunc(SymbolRef, IntType),
    ZExt(SymbolRef, IntType),
    SExt(SymbolRef, IntType),
    FTrunc(SymbolRef, FloatType),
    FExt(SymbolRef, FloatType),
    ICmp(ICmpOp, SymbolRef, SymbolRef),
    FCmp(FCmpOp, SymbolRef, SymbolRef),
    Call(SymbolRef, Vec<SymbolRef>),
}
