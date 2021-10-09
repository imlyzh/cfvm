use sexpr_ir::gast::Handle;

use crate::nodes::{FunctionDef, GlobalEnv, Module, handles::{GlobalHandle, LazyLoadSymbol, LocalHandle, TypeDef, TypeHandle, TypeSymbol}};



pub trait Init<Env> {
    fn init(&self, env: &mut Env) -> Result<(), ()>;
}

impl Init<(&Module, &GlobalEnv)> for TypeHandle {
    fn init(&self, env: &mut (&Module, &GlobalEnv)) -> Result<(), ()> {
        let v = self.0.read().unwrap().to_owned();
        if let LazyLoadSymbol::Symbol(TypeSymbol(namespace, name)) = v {
            if let Some(k) = namespace {
                let r = env.1;
                let r = r.0.get(&k).ok_or(())?.read().unwrap();
                let TypeDef(_, _, r) = r.type_defs.get(&name).ok_or(())?;
                r.init(env)?;
                *self.0.write().unwrap() = r.0.read().unwrap().to_owned();
            } else {
                let TypeDef(_, _, r) = env.0.type_defs.get(&name).ok_or(())?;
                r.init(env)?;
                *self.0.write().unwrap() = r.0.read().unwrap().to_owned();
            };
        }
        Ok(())
    }
}

impl Init<GlobalEnv> for GlobalHandle {
    fn init(&self, env: &mut GlobalEnv) -> Result<(), ()> {
        Ok(())
    }
}

impl Init<FunctionDef> for LocalHandle {
    fn init(&self, env: &mut FunctionDef) -> Result<(), ()> {
        Ok(())
    }
}

impl Init<GlobalEnv> for Module {
    fn init(&self, env: &mut GlobalEnv) -> Result<(), ()> {
        self.type_defs
            .iter()
            .try_for_each(|(_, TypeDef(_, _, t))|
                t.init(&mut (self, env))
            )?;
        Ok(())
    }
}