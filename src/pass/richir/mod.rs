pub mod pe;

use std::{collections::HashMap, sync::{Arc, RwLock}, cell::RefCell, rc::Rc, ops::DerefMut};

use crate::cfir::{
    richir::{Expr, Value},
    handles::{MutHandle, DefineSymbol, LocalSymbol, Symbol, TypeDefineSymbol},
    types::Type
};

#[derive(Debug, Clone, Default)]
pub struct Context {
    pub local: LocalContext,
    pub module: ModuleContext,
    pub global: GlobalContext,
}

impl Context {
    pub fn new_level(&self) -> Self {
        Self {
            local: self.local.new_level(),
            module: self.module.clone(),
            global: self.global.clone(),
        }
    }
    pub fn set_local(&self, name: &LocalSymbol, value: Arc<Value>) {
        let r = self.local.local_vars.clone();
        r.borrow_mut().deref_mut().insert(name.clone(), value);
    }
}

#[derive(Debug, Clone, Default)]
pub struct LocalContext {
    pub local_vars: Rc<RefCell<HashMap<LocalSymbol, Arc<Value>>>>,
    pub parent: Option<Rc<LocalContext>>,
}

impl LocalContext {
    fn new_level(&self) -> Self {
        Self {
            local_vars: Default::default(),
            parent: Some(Rc::new(self.clone())),
        }
    }
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
