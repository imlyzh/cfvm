use std::sync::{Arc, RwLock};

use super::types::Type;


pub type Handle<T> = Arc<T>;
pub type MutHandle<T> = Arc<RwLock<T>>;

#[derive(Debug, Hash, Clone, Eq, PartialEq, PartialOrd)]
pub struct GlobalSymbol(pub Option<Symbol>, pub DefineSymbol);

#[derive(Debug, Hash, Clone, Eq, PartialEq, PartialOrd)]
pub struct TypeSymbol(pub Option<Symbol>, pub TypeDefineSymbol);

#[derive(Debug, Hash, Clone, Eq, PartialEq, PartialOrd)]
pub struct Symbol(pub Handle<String>); // record line key, type name, etc.

// text type

#[derive(Debug, Hash, Clone, Eq, PartialEq, PartialOrd)]
pub struct DefineSymbol(pub Handle<String>);

#[derive(Debug, Hash, Clone, Eq, PartialEq, PartialOrd)]
pub struct LocalSymbol(pub Handle<String>);

#[derive(Debug, Hash, Clone, Eq, PartialEq, PartialOrd)]
pub struct LabelSymbol(pub Handle<String>);

#[derive(Debug, Hash, Clone, Eq, PartialEq, PartialOrd)]
pub struct TypeDefineSymbol(pub Handle<String>);

#[derive(Debug, Hash, Clone, Eq, PartialEq, PartialOrd)]
pub enum SymbolRef {
    Local(LocalSymbol),
    Global(GlobalSymbol),
    Symbol(Symbol),
}

// literal values

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct RecordValue(pub Vec<(Option<Symbol>, ConstantValue)>);

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct VectorValue(pub Vec<SimpleValue>);

#[derive(Debug, Clone, PartialEq, PartialOrd)]
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
