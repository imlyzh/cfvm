use crate::cfir::graphir::{
    handles::{LazyLoadSymbol, LocalHandle, LocalSymbol},
    instruction::{BindOperator, Instruction, Store}
};


pub trait GetSymbol {
    fn get_symbol(&self) -> LocalSymbol;
}

impl GetSymbol for LocalSymbol {
    fn get_symbol(&self) -> LocalSymbol {
        self.clone()
    }
}

impl GetSymbol for LocalHandle {
    fn get_symbol(&self) -> LocalSymbol {
        match self.0.read().unwrap().clone() {
            LazyLoadSymbol::Symbol(s) => s,
            LazyLoadSymbol::Reference(i) => i.get_symbol(),
        }
    }
}

impl GetSymbol for Instruction {
    fn get_symbol(&self) -> LocalSymbol {
        match self {
            Instruction::Store(Store(s, _, _, _)) => s.get_symbol(),
            Instruction::BindOperator(BindOperator(l, _, _, _)) => l.get_symbol(),
            Instruction::Operator(_op) => panic!("Operator instruction not supported"),
        }
    }
}
