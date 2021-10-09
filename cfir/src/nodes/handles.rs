use std::sync::{Arc, RwLock};

use sexpr_ir::gast::Handle;

use super::{
    basicblock::BasicBlockDef,
    instruction::Instruction,
    types::{GetType, Type},
    FunctionDecl, FunctionDef,
};

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub struct GlobalSymbol(pub Option<Handle<String>>, pub Handle<String>);

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub struct LocalSymbol(pub Option<Handle<String>>, pub Handle<String>);

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub struct TypeSymbol(pub Option<Handle<String>>, pub TypeDefineSymbol);

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub struct Symbol(pub Arc<String>); // record line key, params name, etc.

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub struct DefineSymbol(pub Handle<String>);

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
pub struct RecordValue(pub Vec<(Option<Symbol>, ConstantValue)>);

#[derive(Debug, Clone)]
pub struct VectorValue(pub Vec<SimpleValue>);

#[derive(Debug, Clone)]
pub enum SimpleValue {
    FloatNumber(String),
    Number(String),
    Char(char),
    Vector(VectorValue),
}

#[derive(Debug, Clone)]
pub struct ArrayValue(pub Vec<ConstantValue>);

#[derive(Debug, Clone)]
pub struct StringLit(pub Handle<String>);

#[derive(Debug, Clone)]
pub enum ConstantValue {
    SimpleValue(SimpleValue),
    ArrayValue(ArrayValue),
    RecordValue(RecordValue),
    StringLit(StringLit),
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
pub struct IsExtern(pub bool);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IsPublic(pub bool);

pub type TypeHandle = SymbolHandle<TypeSymbol, Arc<Type>>;

#[derive(Debug, Clone)]
pub struct TypeDef(pub IsPublic, pub TypeDefineSymbol, pub TypeHandle);

#[derive(Debug, Clone)]
pub struct ConstantDef(pub IsPublic, pub DefineSymbol, pub Type, pub ConstantValue);

#[derive(Debug, Clone)]
pub struct VariableDef(
    pub IsPublic,
    pub DefineSymbol,
    pub Type,
    pub Option<ConstantValue>,
);

pub trait GetValue<Env> {
    type Target;
    fn get_value(&self, env: &Env) -> Option<Self::Target>;
}
