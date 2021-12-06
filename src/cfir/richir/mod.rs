pub mod parser;

use std::{sync::Arc, collections::HashMap};

use super::{handles::{ConstantValue, Symbol, LocalSymbol, SymbolRef, DefineSymbol, TypeSymbol}, types::{FunctionType, ParamsType, TypeBindAttr}};


#[derive(Debug, Clone, PartialEq)]
pub struct Module {
    pub name: Symbol,
    pub constants: HashMap<DefineSymbol, Constant>,
    pub variables: HashMap<DefineSymbol, Variable>,
    pub function_decl: HashMap<DefineSymbol, FunDecl>,
    pub functions: HashMap<DefineSymbol, Fun>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Constant {
    pub name: Symbol,
    pub type_: TypeSymbol,
    pub value: ConstantValue,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    pub name: Symbol,
    pub type_: TypeSymbol,
    pub value: Option<ConstantValue>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunDecl {
    pub name: DefineSymbol,
    pub header: FunctionType,
    pub is_pure: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NamedFun {
    pub name: Symbol,
    pub fun: Arc<Fun>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Fun {
    pub params: ParamsType,
    pub body: Arc<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LetBinding {
    pub bind: (LocalSymbol, Arc<Value>, Option<TypeBindAttr>),
    pub body: Arc<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Let(Arc<LetBinding>),
    If(Arc<Value>, Arc<Expr>, Arc<Expr>), // cond, then, else
    // Cond(Vec<(Value, Expr)>, Arc<Expr>),
    While(Arc<Value>, Arc<Expr>, Arc<Expr>), // cond, body, accum
    Begin(Vec<Expr>),
    Store(Arc<Value>, Arc<Expr>), // name, value
    Val(Value),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Var(SymbolRef),
    Lit(Arc<ConstantValue>),
    Call(Arc<Call>),
    Fun(Arc<Fun>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    ConstVal(Arc<ConstantValue>),
    Fun(Arc<Fun>)
}

impl Value {
    pub fn is_literal(&self) -> bool {
        match self {
            Value::Lit(_) |
            Value::Fun(_) => true,
            _ => false,
        }
    }
    pub fn get_literal(&self) -> Option<Literal> {
        match self {
            Value::Lit(lit) => Some(Literal::ConstVal(lit.clone())),
            Value::Fun(f) => Some(Literal::Fun(f.clone())),
            _ => None,
        }
    }
    pub fn get_bool_lit(&self) -> Option<bool> {
        if let Value::Lit(l) = self {
            l.get_bool_lit()
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Call {
    fun: SymbolRef,
    args: Vec<Value>,
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