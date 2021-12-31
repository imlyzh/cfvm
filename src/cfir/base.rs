use std::collections::HashMap;

use super::{handles::{ConstantValue, DefineSymbol, TypeSymbol, TypeDefineSymbol}, types::{FunctionType, Type}};


#[derive(Debug, Clone, Default, PartialEq)]
pub struct Env<FunType> {
    pub type_defs: HashMap<TypeDefineSymbol, TypeDef>,
    pub constant_defs: HashMap<DefineSymbol, ConstantDef>,
    pub variable_defs: HashMap<DefineSymbol, VariableDef>,
    pub function_defs: HashMap<DefineSymbol, FunType>,
    pub function_decls: HashMap<DefineSymbol, FunDecl>,
}

impl<FunType> Env<FunType> {
    pub fn new() -> Self {
        Self {
            type_defs: HashMap::new(),
            constant_defs: HashMap::new(),
            variable_defs: HashMap::new(),
            function_defs: HashMap::new(),
            function_decls: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeDef {
    // pub is_pub: IsPublic,
    pub name: TypeDefineSymbol,
    pub type_: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConstantDef {
    // pub is_pub: IsPublic,
    pub name: DefineSymbol,
    pub type_: TypeSymbol,
    pub value: ConstantValue,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariableDef {
    // pub is_pub: IsPublic,
    pub name: DefineSymbol,
    pub type_: TypeSymbol,
    pub value: Option<ConstantValue>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunDecl {
    pub name: DefineSymbol,
    pub header: FunctionType,
}
