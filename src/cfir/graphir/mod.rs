// pub mod basicblock;
// pub mod instruction;

pub mod parser;

use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use super::{
    base::{
        // ConstantDef, VariableDef, TypeDef,
        Module
    },
    types::{
        Type, FunctionType, PointerType, FirstClassType, SimpleType, GetType,
        IsExtern, IsPublic, InlineType
    },
    handles::{DefineSymbol, LabelSymbol}
};


pub type MutHandle<T> = Arc<RwLock<T>>;


pub type RichModule = Module<FunctionDef>;

#[derive(Debug, Clone)]
pub struct FunctionAttr {
    pub is_extern: IsExtern,
    pub is_public: IsPublic,
    pub is_inline: Option<InlineType>,
}

#[derive(Debug, Clone)]
pub struct FunctionDef {
    pub name: DefineSymbol,
    pub header: FunctionType,
    pub function_attr: FunctionAttr,
    // pub blocks: MutHandle<Vec<MutHandle<BasicBlockDef>>>,
    pub block_map: MutHandle<HashMap<LabelSymbol, usize>>,
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
