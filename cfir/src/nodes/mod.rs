pub mod types;
pub mod instruction;
pub mod basicblock;
pub mod handles;

use std::{collections::HashMap, sync::{Arc, RwLock}};

use sexpr_ir::gast::Handle;
use basicblock::*;
use types::*;

use self::handles::{GlobalValue, GlobalHandle, GlobalSymbol, LabelSymbol, TypeDef};


pub type MutHandle<T> = Arc<RwLock<T>>;


#[derive(Debug, Clone)]
pub struct Module {
    pub name: Option<Handle<String>>,
    pub type_defs: HashMap<GlobalSymbol, TypeDef>,
    pub constant_defs: HashMap<GlobalSymbol, GlobalValue>,
    pub functions: HashMap<GlobalSymbol, FunctionDef>,
    pub public_functions: HashMap<GlobalSymbol, FunctionDef>,
    pub function_decls: HashMap<GlobalSymbol, FunctionDecl>,
}


#[derive(Debug, Clone)]
pub struct FunctionDecl {
    pub name: GlobalHandle,
    pub header: FunctionType,
    // metadatas
}

impl GetType for FunctionDecl {
    fn get_type(&self) -> Type {
        let func_type = Type::FunctionType(self.header.clone());
        // let r = func_type;
        let r = Type::FirstClassType(FirstClassType::SimpleType(SimpleType::Pointer(PointerType(Box::new(func_type)))));
        r
    }
}


#[derive(Debug, Clone)]
pub struct FunctionDef {
    pub name: GlobalHandle,
    pub header: FunctionType,
    // metadatas
    pub blocks: Vec<MutHandle<BasicBlockDef>>,
    pub block_map: HashMap<LabelSymbol, usize>,
}


impl GetType for FunctionDef {
    fn get_type(&self) -> Type {
        let func_type = Type::FunctionType(self.header.clone());
        // let r = func_type;
        let r = Type::FirstClassType(FirstClassType::SimpleType(SimpleType::Pointer(PointerType(Box::new(func_type)))));
        r
    }
}