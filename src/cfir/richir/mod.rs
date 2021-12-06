pub mod parser;

use std::{sync::Arc, collections::HashMap};


#[derive(Debug, Clone, PartialEq)]
struct Module {
    name: String,
    // imports: Vec<String>,
    // exports: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LetBinding {
    pub bind: HashMap<Arc<String>, Arc<Value>>,
    pub body: Arc<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Let(Arc<LetBinding>),
    If(Arc<Value>, Arc<Expr>, Arc<Expr>),
    // Cond(Vec<(Value, Expr)>, Arc<Expr>),
    While(Value, Arc<Expr>, Vec<Expr>),
    Begin(Vec<Expr>),
    Store(Arc<Value>, Arc<Expr>),
    Val(Value),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Var(Arc<String>),
    Lit(Arc<Literal>),
    Call(Arc<String>, Vec<Value>),
    Fun(Vec<Arc<String>>, Arc<Expr>),
}

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
