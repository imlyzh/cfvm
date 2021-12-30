pub mod instruction;

pub mod parser;

use std::{
    collections::HashMap,
    // sync::{Arc, RwLock},
};

use self::instruction::{Instruction, Terminator};

use super::{
    base::{
        // ConstantDef, VariableDef, TypeDef,
        Module
    },
    types::{
        Type, FunctionType, PointerType, FirstClassType, SimpleType, GetType,
        FunctionAttr
    },
    handles::{DefineSymbol, LabelSymbol, MutHandle, ConstantValue, SymbolRef, LTMHand}
};

pub type GraphModule = Module<FunctionDef>;

#[derive(Debug, Clone)]
pub struct FunctionDef {
    pub name: DefineSymbol,
    pub header: FunctionType,
    pub function_attr: FunctionAttr,
    pub bbs: LTMHand<Vec<LTMHand<BasicBlockDef>>>,
    // pub bbs_map: LTMHand<HashMap<LabelSymbol, usize>>,
}

#[derive(Debug, Clone)]
pub struct BasicBlockDef {
    pub label: LabelSymbol,
    // pub prev_block: LTMHand<Vec<LTMHand<BasicBlockDef>>>,
    // pub variable_defs: LTMHand<HashMap<LabelSymbol, LTMHand<BindOperator>>>,
    pub instructions: LTMHand<Vec<LTMHand<Instruction>>>,
    pub terminator: Option<LTMHand<Terminator>>,
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
}