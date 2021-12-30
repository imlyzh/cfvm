use crate::cfir::{
    handles::LocalSymbol,
    graphir::{
        FunctionDef,
        instruction::{Instruction, BindOperator}
    },
};




pub fn get_all_veriable(fun_def: &FunctionDef) -> Vec<LocalSymbol> {
    let mut vars = Vec::new();
    for bb in fun_def.bbs.borrow().iter() {
        for i in bb.borrow().instructions.borrow().iter() {
            match i.borrow().to_owned() {
                Instruction::BindOperator(BindOperator(var, _)) => {
                    vars.push(var.clone());
                },
                _ => {},
            }
        }
    }
    vars
}