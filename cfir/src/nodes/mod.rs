pub mod types;
pub mod instruction;
pub mod basicblock;

use std::collections::HashMap;

use sexpr_ir::gast::Handle;
use basicblock::*;
use types::*;



#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Module {
    pub name: String,
    pub type_defs: HashMap<Handle<String>, TypeDef>,
    pub functions: HashMap<Handle<String>, FunctionDef>,
    pub public_functions: HashMap<Handle<String>, FunctionDef>,
    pub function_decls: HashMap<Handle<String>, FunctionDecl>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FunctionDecl {
    pub name: Handle<String>,
    pub header: FunctionType,
    // metadatas
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FunctionDef {
    pub name: Handle<String>,
    pub header: FunctionType,
    pub blocks: Vec<BasicBlockDef>,
    pub block_map: HashMap<Handle<String>, usize>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TypeDef {
    pub name: Handle<String>,
    pub type_: Type,
}
