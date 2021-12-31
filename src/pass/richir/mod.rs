pub mod pe;

use std::{collections::HashMap, sync::Arc, cell::RefCell, rc::Rc, ops::DerefMut, borrow::Borrow};

use crate::cfir::{
    handles::{MutHandle, DefineSymbol, LocalSymbol, Symbol, TypeDefineSymbol, SymbolRef, GlobalSymbol},
    types::Type,
    richir::{Value, Fun, Literal},
};

#[derive(Debug, Clone, Default)]
pub struct Context<'a> {
    pub local: LocalContext<'a>,
    pub global: ModuleContext,
}

impl<'a, 'b: 'a> Context<'a> {
    pub fn new_level(&'a self) -> Self {
        Self {
            local: self.local.new_level(),
            global: self.global.clone(),
        }
    }
}

impl<'a> Context<'a> {
    pub fn set_local(&self, name: &LocalSymbol, value: &Value) {
        let r = self.local.local_vars.clone();
        r.borrow_mut().deref_mut().insert(name.clone(), value.clone());
    }

    pub fn get(&self, name: &SymbolRef) -> Option<Literal> {
        match name {
            SymbolRef::Local(name) => self.get_local(name),
            SymbolRef::Global(name) => self.get_global(name),
            SymbolRef::Symbol(_name) => todo!(),
        }
    }
    pub fn get_local(&self, name: &LocalSymbol) -> Option<Literal> {
        let v = self.get_local_value(name)?;
        match v {
            Value::Var(name) => self.get(&name),
            Value::Lit(v) => Some(Literal::ConstVal(v)),
            Value::Fun(f) => Some(Literal::Fun(f)),
            Value::Call(_) => None,
        }
    }
    pub fn get_local_value(&self, name: &LocalSymbol) -> Option<Value> {
        self.local.local_vars.as_ref().borrow().get(name).cloned()
    }
    pub fn get_global(&self, name: &GlobalSymbol) -> Option<Literal> {
        let global = &self.global;
        if let Some(x) = global.functions.as_ref().borrow().read().unwrap().get(&name.0).cloned() {
            return Some(Literal::Fun(x));
        }
        if let Some(x) = global.constant.as_ref().borrow().read().unwrap().get(&name.0).cloned() {
            return Some(x);
        }
        if let Some((_ty, value)) = global.vars.as_ref().borrow().read().unwrap().get(&name.0).cloned() {
            return value;
        }
        None
    }
}

#[derive(Debug, Clone, Default)]
pub struct LocalContext<'a> {
    pub local_vars: Rc<RefCell<HashMap<LocalSymbol, Value>>>,
    pub parent: Option<&'a LocalContext<'a>>,
}

impl<'a, 'b: 'a> LocalContext<'a> {
    fn new_level(&'a self) -> Self {
        Self {
            local_vars: Default::default(),
            parent: Some(self),
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
