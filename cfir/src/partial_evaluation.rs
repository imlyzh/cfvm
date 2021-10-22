use crate::graphir::{FunctionDef, Module, basicblock::BasicBlockDef};



pub trait PartialEval<Env> {
    fn partial_eval(&self, env: &Env) -> Self;
}

impl PartialEval<FunctionDef> for BasicBlockDef {
    fn partial_eval(&self, env: &FunctionDef) -> Self {
        todo!()
    }
}

impl PartialEval<Module> for FunctionDef {
    fn partial_eval(&self, env: &Module) -> Self {
        todo!()
    }
}
