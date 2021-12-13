pub mod parser;

use std::sync::Arc;

use super::{handles::{ConstantValue, Symbol, LocalSymbol, SymbolRef}, types::{ParamsType, TypeBindAttr}, base::Module};


pub type RichModule = Module<Fun>;


#[derive(Debug, Clone, PartialEq)]
pub struct NamedFun {
    pub name: Symbol,
    pub fun: Fun,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Fun {
    pub args: ParamsType,
    pub body: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LetBinding {
    pub bind: (LocalSymbol, Value, Option<TypeBindAttr>),
    pub body: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Let(LetBinding),
    If(Value, Box<Expr>, Box<Expr>), // cond, then, else
    // Cond(Vec<(Value, Expr)>, Box<Expr>),
    // Switch(Value, Vec<(ConstantValue, Expr)>, Box<Expr>),
    While(Value, Box<Expr>, Box<Expr>), // cond, body, accum
    Begin(Vec<Expr>),
    Store(Value, Box<Expr>), // name, value
    Val(Value),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Var(SymbolRef),
    Lit(ConstantValue),
    Call(Call),
    Fun(Arc<Fun>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Call {
    pub fun: Box<Value>,
    pub args: Vec<Value>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    ConstVal(ConstantValue),
    Fun(Arc<Fun>)
}

impl From<Literal> for Value {
    fn from(i: Literal) -> Self {
        match i {
            Literal::ConstVal(c) => Value::Lit(c),
            Literal::Fun(f) => Value::Fun(f),
        }
    }
}

impl Expr {
    pub fn is_let(&self) -> bool {
        matches!(self, Expr::Let(_))
    }
    pub fn is_literal(&self) -> bool {
        match self {
            Expr::Val(v) => v.is_literal(),
            _ => false,
        }
    }
    pub fn get_literal(&self) -> Option<Literal> {
        match self {
            Expr::Val(lit) => lit.get_literal(),
            _ => None,
        }
    }
    pub fn get_value(&self) -> Option<&Value> {
        match self {
            Expr::Val(v) => Some(v),
            _ => None,
        }
    }
}

impl Literal {
    pub fn is_const(&self) -> bool {
        matches!(self, Literal::ConstVal(_))
    }
    pub fn get_const(&self) -> Option<&ConstantValue> {
        if let Literal::ConstVal(v) = self {
            Some(v)
        } else {
            None
        }
    }
    pub fn is_fun(&self) -> bool {
        matches!(self, Literal::Fun(_))
    }
    pub fn get_fun(&self) -> Option<Arc<Fun>> {
        if let Literal::Fun(f) = self {
            Some(f.clone())
        } else {
            None
        }
    }

}

impl Value {
    pub fn is_literal(&self) -> bool {
        matches!(self, Value::Lit(_) | Value::Fun(_))
    }
    pub fn get_literal(&self) -> Option<Literal> {
        match self {
            Value::Lit(lit) => Some(Literal::ConstVal(lit.clone())),
            Value::Fun(f) => Some(Literal::Fun(f.clone())),
            _ => None,
        }
    }
    pub fn get_symbol(&self) -> Option<&SymbolRef> {
        match self {
            Value::Var(sym) => Some(sym),
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
    pub fn get_fun(&self) -> Option<Arc<Fun>> {
        if let Value::Fun(f) = self {
            Some(f.clone())
        } else {
            None
        }
    }
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