use crate::graphir::*;
use crate::graphir::handles::*;
use crate::graphir::basicblock::*;



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
                let TypeDef(_, _, r) = r.type_defs.get(&name).ok_or(())?.as_ref();
                r.init(env)?;
                *self.0.write().unwrap() = r.0.read().unwrap().to_owned();
            } else {
                let TypeDef(_, _, r) = env.0.type_defs.get(&name).ok_or(())?.as_ref();
                r.init(env)?;
                *self.0.write().unwrap() = r.0.read().unwrap().to_owned();
            };
        }
        Ok(())
    }
}

impl Init<(&Module, &GlobalEnv)> for GlobalHandle {
    fn init(&self, env: &mut (&Module, &GlobalEnv)) -> Result<(), ()> {
        let v = self.0.read().unwrap().to_owned();
        if let LazyLoadSymbol::Symbol(GlobalSymbol(namespace, name)) = v {
            if let Some(k) = namespace {
                let r = env.1;
                if let Some(r) = r.0.get(&k) {
                    let r = r.read().unwrap();
                    if let Some(r) = r.variable_defs.get(&name) {
                        *self.0.write().unwrap() = LazyLoadSymbol::Reference(GlobalValue::Variable(r.clone()));
                    }
                    let r = r.constant_defs.get(&name).ok_or(())?;
                    *self.0.write().unwrap() = LazyLoadSymbol::Reference(GlobalValue::Constant(r.clone()));
                } else {
                    return Err(());
                }
            } else {
                let r = env.0;
                if let Some(r) = r.variable_defs.get(&name) {
                    *self.0.write().unwrap() = LazyLoadSymbol::Reference(GlobalValue::Variable(r.clone()));
                }
                let r = r.constant_defs.get(&name).ok_or(())?;
                *self.0.write().unwrap() = LazyLoadSymbol::Reference(GlobalValue::Constant(r.clone()));
            };
        }
        Ok(())
    }
}

impl Init<(&BasicBlockDef, &FunctionDef)> for LocalHandle {
    fn init(&self, env: &mut (&BasicBlockDef, &FunctionDef)) -> Result<(), ()> {
        todo!();
    }
}

impl Init<GlobalEnv> for Module {
    fn init(&self, env: &mut GlobalEnv) -> Result<(), ()> {
        self.type_defs
            .iter()
            .try_for_each(|(_, t)| {
                let TypeDef(_, _, t) = t.as_ref();
                t.init(&mut (self, env))
            })?;
        self.constant_defs
            .iter()
            .try_for_each(|(_, t)| {
                let ConstantDef(_, _, t, _) = t.as_ref();
                t.init(&mut (self, env))
            })?;
        self.variable_defs
            .iter()
            .try_for_each(|(_, t)| {
                let VariableDef(_, _, t, _) = t.as_ref();
                t.init(&mut (self, env))
            })?;
        // todo: self.function_decls, type init
        Ok(())
    }
}