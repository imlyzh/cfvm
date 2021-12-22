pub mod instruction;

pub mod parser;

use std::{
    collections::HashMap,
    // sync::{Arc, RwLock},
};

use self::instruction::{BindOperator, Instruction, Terminator};

use super::{
    base::{
        // ConstantDef, VariableDef, TypeDef,
        Module
    },
    types::{
        Type, FunctionType, PointerType, FirstClassType, SimpleType, GetType,
        IsExtern, IsPublic, InlineType, FunctionAttr
    },
    handles::{DefineSymbol, LabelSymbol, MutHandle, ConstantValue, SymbolRef}
};


pub type RichModule = Module<FunctionDef>;

#[derive(Debug, Clone)]
pub struct FunctionDef {
    pub name: DefineSymbol,
    pub header: FunctionType,
    pub function_attr: FunctionAttr,
    pub blocks: MutHandle<Vec<MutHandle<BasicBlockDef>>>,
    pub block_map: MutHandle<HashMap<LabelSymbol, usize>>,
}

#[derive(Debug, Clone)]
pub struct BasicBlockDef {
    pub label: Option<LabelSymbol>,
    // pub prev_block: MutHandle<Vec<MutHandle<BasicBlockDef>>>,
    // pub variable_defs: MutHandle<HashMap<LabelSymbol, MutHandle<BindOperator>>>,
    pub instructions: MutHandle<Vec<MutHandle<Instruction>>>,
    pub terminator: MutHandle<Terminator>,
}

impl GetType for FunctionDef {
    fn get_type(&self) -> Type {
        let func_type = Type::FunType(self.header.clone());
        // let r = func_type;
        let r = Type::FCType(FirstClassType::SimpleType(SimpleType::Pointer(
            PointerType(Box::new(func_type)),
        )));
        r
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Var(SymbolRef),
    Lit(ConstantValue),
    // Call(Call),
}