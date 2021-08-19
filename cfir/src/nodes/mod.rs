pub mod types;
pub mod instruction;

use types::*;

#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub type_defs: Vec<TypeDef>,
    pub functions: Vec<FunctionDef>,
    // pub matadatas
}

pub struct FunctionDecl {
    pub name: String,
    pub header: FunctionType,
}

#[derive(Debug, Clone)]
pub struct FunctionDef {
    pub name: String,
    pub header: FunctionType,
    pub is_public: bool
    
}

#[derive(Debug, Clone)]
pub struct TypeDef {
    pub name: String,
    pub type_: Type,
    pub is_public: bool,
}

