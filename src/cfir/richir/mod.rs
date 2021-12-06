pub mod parser;

use std::{sync::Arc, collections::HashMap};

use super::handles::{ConstantValue, Symbol, LocalSymbol, SymbolRef};


#[derive(Debug, Clone, PartialEq)]
pub struct Module {
    pub name: Symbol,
    pub constants: HashMap<Arc<String>, Constant>,
    pub variables: HashMap<Arc<String>, Variable>,
    pub functions: HashMap<Arc<String>, Fun>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Constant {

}

#[derive(Debug, Clone, PartialEq)]
pub struct Variable {

}

#[derive(Debug, Clone, PartialEq)]
pub struct NamedFun {
    pub name: Symbol,
    pub fun: Arc<Fun>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Fun {
    pub params: Vec<Arc<String>>,
    pub body: Arc<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LetBinding {
    pub bind: HashMap<LocalSymbol, Arc<Value>>,
    pub body: Arc<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Let(Arc<LetBinding>),
    If(Arc<Value>, Arc<Expr>, Arc<Expr>),
    // Cond(Vec<(Value, Expr)>, Arc<Expr>),
    While(Arc<Value>, Arc<Expr>, Vec<Expr>),
    Begin(Vec<Expr>),
    Store(Arc<Value>, Arc<Expr>),
    Val(Value),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Var(SymbolRef),
    Lit(Arc<ConstantValue>),
    Call(SymbolRef, Vec<Value>),
    Fun(Fun),
}

/*
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    SimpleLit(SimpleLiteral),
    Str(Arc<String>),
    Array(Vec<Literal>),
    Record(Arc<String>, HashMap<Arc<String>, Arc<Literal>>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum SimpleLiteral {
    Bool(bool),
    U8(u8),
    U64(u64),
    I64(i64),
    F32(f32),
    F64(f64),
    Vector(Vec<SimpleLiteral>),
}
 */