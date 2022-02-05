
use crate::cfir::{types::TypeBindAttr, handles::{SymbolRef, SimpleValue}, linearir::{IndexList, ICmpOp, FCmpOp}};

use super::{super::{
    types::{FloatType, IntType, Type},
    // MutHandle,
}};


enum Bytecode {
    Const(TypeBindAttr, SimpleValue),
    GetPtr(Option<IndexList>),
    Load(Type, SymbolRef),
    Cast(Type, SymbolRef),
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
    GetValue(IndexList),
    GetItem,
    SetValue(IndexList),
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