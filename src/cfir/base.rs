use std::collections::HashMap;

use super::{handles::{ConstantValue, Symbol, DefineSymbol, TypeSymbol, TypeDefineSymbol}, types::{FunctionType, Type, IsPublic}};


#[derive(Debug, Clone, PartialEq)]
pub struct Module<FunType> {
    pub name: Symbol,
    pub type_defs: HashMap<DefineSymbol, TypeDef>,
    pub constant_defs: HashMap<DefineSymbol, ConstantDef>,
    pub variable_defs: HashMap<DefineSymbol, VariableDef>,
    pub function_decl: HashMap<DefineSymbol, FunDecl>,
    pub function_defs: HashMap<DefineSymbol, FunType>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeDef {
    pub is_pub: IsPublic,
    pub name: TypeDefineSymbol,
    pub type_: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConstantDef {
    pub is_pub: IsPublic,
    pub name: DefineSymbol,
    pub type_: TypeSymbol,
    pub value: ConstantValue,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariableDef {
    pub is_pub: IsPublic,
    pub name: DefineSymbol,
    pub type_: TypeSymbol,
    pub value: Option<ConstantValue>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunDecl {
    pub name: DefineSymbol,
    pub header: FunctionType,
}
