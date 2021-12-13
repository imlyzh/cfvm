use std::{sync::Arc, collections::HashMap};

use super::{handles::{ConstantValue, Symbol, LocalSymbol, SymbolRef, DefineSymbol, TypeSymbol}, types::{FunctionType, ParamsType, TypeBindAttr}};


#[derive(Debug, Clone, PartialEq)]
pub struct Module<FunType> {
    pub name: Symbol,
    pub constants: HashMap<DefineSymbol, Constant>,
    pub variables: HashMap<DefineSymbol, Variable>,
    pub function_decl: HashMap<DefineSymbol, FunDecl>,
    pub functions: HashMap<DefineSymbol, FunType>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Constant {
    pub name: Symbol,
    pub type_: TypeSymbol,
    pub value: ConstantValue,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    pub name: Symbol,
    pub type_: TypeSymbol,
    pub value: Option<ConstantValue>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunDecl {
    pub name: DefineSymbol,
    pub header: FunctionType,
    pub is_pure: bool,
}