
use crate::cfir::{types::TypeBindAttr, handles::{SymbolRef, SimpleValue, GlobalSymbol, LocalSymbol}, linearir::{IndexList, ICmpOp, FCmpOp}};

use super::{super::{
    types::{FloatType, IntType, Type},
    // MutHandle,
}, Index};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Bytecode {
    Const(TypeBindAttr, SimpleValue),
    GetPtr(Option<Index>),
    Load(Type, usize),
    LoadStructRef(Type, usize),
    LoadGlobal(Type, usize),
    Cast(Type),
    // StackAlloc(Type),
    // HeapAlloc(Type),
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
    GetValue(Index),
    GetItem,
    SetValue(Index),
    SetItem,
    Trunc(IntType),
    ZExt(IntType),
    SExt(IntType),
    FTrunc(FloatType),
    FExt(FloatType),
    ICmp(ICmpOp),
    FCmp(FCmpOp),
    // Call(SymbolRef, Vec<SymbolRef>),
    Call,
}