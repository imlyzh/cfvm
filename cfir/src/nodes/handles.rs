use std::{sync::{Arc, RwLock}};

use sexpr_ir::gast::Handle;

use super::{FunctionDecl, FunctionDef, basicblock::BasicBlockDef, instruction::Instruction, types::{GetType, Type}};



#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub struct LocalSymbol(Handle<String>);

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub struct GlobalSymbol(Handle<String>);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LabelSymbol(Handle<String>);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TypeSymbol(Handle<String>);



#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub enum LazyLoadSymbol<T, R> {
    Symbol(T),
    Reference(R)
}

#[derive(Debug, Clone)]
pub struct SymbolHandle<T, R>(pub Arc<RwLock<LazyLoadSymbol<T, R>>>);


// local value

pub type LocalHandle = SymbolHandle<LocalSymbol, Arc<Instruction>>;


// global value

#[derive(Debug, Clone)]
pub enum GlobalValue {
    Constant(Type, Vec<u8>),
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
    Global(GlobalSymbol)
}

#[derive(Debug, Clone)]
pub enum ValueRef {
    Local(Arc<Instruction>),
    Global(GlobalValue)
}

pub type ValueHandle = SymbolHandle<SymbolRef, ValueRef>;


// basic block symbols

pub type LabelHandle = SymbolHandle<LabelSymbol, Arc<BasicBlockDef>>;


// type

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TypeDef {
    pub name: Handle<String>,
    pub type_: Type,
}

pub type TypeHandle = SymbolHandle<TypeSymbol, Arc<Type>>;
