pub mod pe;

use std::{collections::HashMap, sync::Arc, cell::RefCell, rc::Rc, ops::DerefMut, borrow::Borrow};

use crate::cfir::{
    richir::{Value, Fun, Literal},
    handles::{MutHandle, DefineSymbol, LocalSymbol, Symbol, TypeDefineSymbol, SymbolRef, GlobalSymbol},
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
    pub fn set_local(&self, name: &LocalSymbol, value: Literal) {
        let r = self.local.local_vars.clone();
        r.borrow_mut().deref_mut().insert(name.clone(), value);
    }

    pub fn get(&self, name: &SymbolRef) -> Option<Literal> {
        match name {
            SymbolRef::Local(l) => todo!(),
            SymbolRef::Global(g) => todo!(),
            SymbolRef::Symbol(s) => todo!(),
        }
        todo!()
    }
    pub fn get_local(&self, name: &LocalSymbol) -> Option<Literal> {
        self.local.local_vars.as_ref().borrow().get(name).map(|v| v.clone())
    }
    pub fn get_global(&self, name: &GlobalSymbol) -> Option<Literal> {
        let record = self.global.modules.as_ref().read().unwrap();
        let module = if let Some(module) = &name.0 {
            record.get(module)?
        } else {
            &self.module
        };
        if let Some(x) = module.functions.as_ref().borrow().read().unwrap().get(&name.1).map(|v| v.clone()) {
            return Some(Literal::Fun(x));
        }
        if let Some(x) = module.constant.as_ref().borrow().read().unwrap().get(&name.1).map(|v| v.clone()) {
            return Some(x);
        }
        if let Some((_ty, value)) = module.vars.as_ref().borrow().read().unwrap().get(&name.1).map(|v| v.clone()) {
            return value;
        }
        None
    }
}

#[derive(Debug, Clone, Default)]
pub struct LocalContext {
    pub local_vars: Rc<RefCell<HashMap<LocalSymbol, Literal>>>,
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
    pub constant: MutHandle<HashMap<DefineSymbol, Literal>>,
    pub vars: MutHandle<HashMap<DefineSymbol, (Type, Option<Literal>)>>,
    pub functions: MutHandle<HashMap<DefineSymbol, Arc<Fun>>>,
}

#[derive(Debug, Clone, Default)]
pub struct GlobalContext {
    pub modules: MutHandle<HashMap<Symbol, ModuleContext>>,
}
