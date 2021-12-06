pub mod pe;

use std::{collections::HashMap, sync::Arc};

use crate::cfir::{richir::{Expr, Value}, handles::{MutHandle, DefineSymbol}, graphir::handles::{LocalSymbol, Symbol, TypeDefineSymbol}, types::Type};



#[derive(Debug, Clone)]
pub struct Context {
    pub local: LocalContext,
    pub module: ModuleContext,
    pub global: GlobalContext,
}

#[derive(Debug, Clone, Default)]
pub struct LocalContext {
    pub local_vars: MutHandle<HashMap<LocalSymbol, Arc<Expr>>>,
    pub parent: Option<Arc<LocalContext>>,
}

#[derive(Debug, Clone, Default)]
pub struct ModuleContext {
    pub types: MutHandle<HashMap<TypeDefineSymbol, Type>>,
    pub constant: MutHandle<HashMap<DefineSymbol, Arc<Value>>>,
    pub vars: MutHandle<HashMap<DefineSymbol, Arc<Expr>>>,
    pub functions: MutHandle<HashMap<DefineSymbol, Arc<Expr>>>,
}

#[derive(Debug, Clone, Default)]
pub struct GlobalContext {
    pub modules: MutHandle<HashMap<Symbol, ModuleContext>>,
}