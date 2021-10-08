use std::sync::{Arc, RwLock};

use sexpr_ir::gast::Handle;

use super::{
    basicblock::BasicBlockDef,
    instruction::Instruction,
    types::{GetType, Type},
    FunctionDecl, FunctionDef,
};


#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub struct GlobalSymbol(pub Handle<String>);

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub struct TypeSymbol(pub Handle<String>);


#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub struct Symbol(pub Arc<String>); // record line key, params name, etc.

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub struct DefineSymbol(pub Handle<String>);

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub struct LocalSymbol(pub Handle<String>);

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub struct LabelSymbol(pub Handle<String>);

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub struct TypeDefineSymbol(pub Handle<String>);


#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub enum LazyLoadSymbol<T, R> {
    Symbol(T),
    Reference(R),
}

#[derive(Debug, Clone)]
pub struct SymbolHandle<T, R>(pub Arc<RwLock<LazyLoadSymbol<T, R>>>);

impl<T, R> SymbolHandle<T, R> {
    pub fn from(symbol: T) -> Self {
        SymbolHandle(Arc::new(RwLock::new(LazyLoadSymbol::Symbol(symbol))))
    }
    pub fn new(reference: R) -> Self {
        SymbolHandle(Arc::new(RwLock::new(LazyLoadSymbol::Reference(reference))))
    }
}

// local value

pub type LocalHandle = SymbolHandle<LocalSymbol, Arc<Instruction>>;

// global value

#[derive(Debug, Clone)]
pub struct RecordValue(pub Vec<(Option<Handle<String>>, ConstantValue)>);

#[derive(Debug, Clone)]
pub enum ConstantValue {
    Int(Vec<u8>),
    Float(Vec<u8>),
    Bytes(Vec<u8>),
    RawChars(Vec<u8>),
    Chars(Vec<char>),
    Array(Vec<ConstantValue>),
    Record(RecordValue),
}

#[derive(Debug, Clone)]
pub enum GlobalValue {
    Constant(Type, ConstantValue),
    FunctionDef(Arc<FunctionDef>),
    FunctionDecl(Arc<FunctionDecl>),
}

impl GetType for GlobalValue {
    fn get_type(&self) -> Type {
        match self {
            GlobalValue::Constant(t, _) => t.clone(),
            GlobalValue::FunctionDef(f) => f.get_type(),
            GlobalValue::FunctionDecl(f) => f.get_type(),
        }
    }
}

pub type GlobalHandle = SymbolHandle<GlobalSymbol, GlobalValue>;

// union value

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub enum SymbolRef {
    Local(LocalSymbol),
    Global(GlobalSymbol),
}

#[derive(Debug, Clone)]
pub enum ValueRef {
    Local(Arc<Instruction>),
    Global(GlobalValue),
}

pub type ValueHandle = SymbolHandle<SymbolRef, ValueRef>;

// basic block symbols

pub type LabelHandle = SymbolHandle<LabelSymbol, Arc<BasicBlockDef>>;

// type

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IsPub(pub bool);

pub type TypeHandle = SymbolHandle<TypeSymbol, Arc<Type>>;

#[derive(Debug, Clone)]
pub struct TypeDef (pub IsPub, pub TypeDefineSymbol, pub TypeHandle);


#[derive(Debug, Clone)]
pub struct ConstantDef(pub IsPub, pub DefineSymbol, pub Type, pub GlobalValue);

#[derive(Debug, Clone)]
pub struct VariableDef(pub IsPub, pub DefineSymbol, pub Type, pub Option<GlobalValue>);

#[derive(Debug, Clone)]
pub struct Attris(pub Vec<Handle<String>>);

impl Attris {
    pub fn have_flag(&self, flag: &str) -> bool {
        self.0.iter().any(|f| f.as_str() == flag)
    }

    pub fn have_flags(&self, flags: &[&str]) -> bool {
        self.0.iter().any(|f| flags.contains(&f.as_str()))
    }

    pub fn have_and_only_have_flags(&self, flags: &[&str]) -> Option<Vec<bool>> {
        if self.0.len() != flags.len() {
            return None;
        }
        let r: Vec<bool> = flags.iter().map(|x| self.have_flag(x)).collect();
        Some(r)
    }
}
