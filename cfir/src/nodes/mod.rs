pub mod basicblock;
pub mod handles;
pub mod instruction;
pub mod parser;
pub mod types;

use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use basicblock::*;
use sexpr_ir::gast::Handle;
use types::*;

use self::handles::{ConstantDef, DefineSymbol, LabelSymbol, TypeDef, VariableDef};

pub type MutHandle<T> = Arc<RwLock<T>>;

#[derive(Debug, Clone)]
pub struct Module {
    pub name: Option<Handle<String>>,
    pub type_defs: HashMap<Handle<String>, TypeDef>,
    pub constant_defs: HashMap<Handle<String>, ConstantDef>,
    pub variable_defs: HashMap<Handle<String>, VariableDef>,
    pub functions: HashMap<Handle<String>, FunctionDef>,
    pub function_decls: HashMap<Handle<String>, FunctionDecl>,
}

#[derive(Debug, Clone)]
pub struct FunctionDecl {
    pub name: DefineSymbol,
    pub header: FunctionType,
}

impl GetType for FunctionDecl {
    fn get_type(&self) -> Type {
        let func_type = Type::FunctionType(self.header.clone());
        // let r = func_type;
        let r = Type::FirstClassType(FirstClassType::SimpleType(SimpleType::Pointer(
            PointerType(Box::new(func_type)),
        )));
        r
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InlineType {
    Inline,
    Const,
}

#[derive(Debug, Clone)]
pub struct FunctionAttr {
    pub is_extern: bool,
    pub is_public: bool,
    pub is_inline: Option<InlineType>,
}

#[derive(Debug, Clone)]
pub struct FunctionDef {
    pub name: DefineSymbol,
    pub header: FunctionType,
    pub function_attr: FunctionAttr,
    pub blocks: Vec<MutHandle<BasicBlockDef>>,
    pub block_map: HashMap<LabelSymbol, usize>,
}

impl GetType for FunctionDef {
    fn get_type(&self) -> Type {
        let func_type = Type::FunctionType(self.header.clone());
        // let r = func_type;
        let r = Type::FirstClassType(FirstClassType::SimpleType(SimpleType::Pointer(
            PointerType(Box::new(func_type)),
        )));
        r
    }
}
