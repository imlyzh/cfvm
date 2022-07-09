use std::{sync::{Arc, RwLock}, cell::RefCell, fmt::Display};

use super::types::Type;


pub type Handle<T> = Arc<T>;
pub type MutHandle<T> = Arc<RwLock<T>>;
// Local Thread Mutable
pub type LTMHand<T> = RefCell<T>;

#[derive(Debug, Hash, Clone, Eq, PartialEq, PartialOrd)]
pub struct GlobalSymbol(pub DefineSymbol);

impl Display for GlobalSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}


#[derive(Debug, Hash, Clone, Eq, PartialEq, PartialOrd)]
pub struct TypeSymbol(pub TypeDefineSymbol);

impl Display for TypeSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Hash, Clone, Eq, PartialEq, PartialOrd)]
pub struct Symbol(pub Handle<String>); // record line key, type name, etc.

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Symbol {
    pub fn new(s: String) -> Self {
        Symbol(Arc::new(s))
    }

    pub fn from(s: &str) -> Self {
        Symbol(Arc::new(s.to_string()))
    }
}

// text type

#[derive(Debug, Hash, Clone, Eq, PartialEq, PartialOrd)]
pub struct DefineSymbol(pub Symbol);

impl Display for DefineSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "@{}", self.0)
    }
}

#[derive(Debug, Hash, Clone, Eq, PartialEq, PartialOrd)]
pub struct LocalSymbol(pub Symbol);

impl Display for LocalSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "%{}", self.0)
    }
}

#[derive(Debug, Hash, Clone, Eq, PartialEq, PartialOrd)]
pub struct LabelSymbol(pub Symbol);

impl Display for LabelSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Hash, Clone, Eq, PartialEq, PartialOrd)]
pub struct TypeDefineSymbol(pub Handle<String>);

impl Display for TypeDefineSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "type {}", self.0)
    }
}

#[derive(Debug, Hash, Clone, Eq, PartialEq, PartialOrd)]
pub enum SymbolRef {
    Local(LocalSymbol),
    Global(GlobalSymbol),
    Symbol(Symbol),
}

// literal values

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct RecordValue(pub Vec<(Option<Symbol>, ConstantValue)>);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct VectorValue(pub Vec<SimpleValue>);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SimpleValue {
    FloatNumber(String),
    Number(String),
    Char(char),
    Bool(bool),
    Vector(VectorValue),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ArrayValue(pub Vec<ConstantValue>);

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct StringLit(pub Handle<String>);

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ConstantValue {
    SimpleValue(SimpleValue),
    ArrayValue(ArrayValue),
    RecordValue(RecordValue),
    StringLit(StringLit),
}

impl SimpleValue {
    pub fn get_bool_lit(&self) -> Option<bool> {
        if let SimpleValue::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }
}

impl ConstantValue {
    pub fn get_bool_lit(&self) -> Option<bool> {
        if let ConstantValue::SimpleValue(s) = self {
            s.get_bool_lit()
        } else {
            None
        }
    }
}


#[derive(Debug, Hash, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum SymbolHandle<T, R> {
    Symbol(T),
    Reference(R),
}

pub type TypeHandle = SymbolHandle<TypeSymbol, Box<Type>>;
