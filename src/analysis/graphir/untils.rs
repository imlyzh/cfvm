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
            if let Instruction::BindOperator(BindOperator(var, _)) = i.borrow().to_owned() {
                vars.push(var.clone());
            }
        }
    }
    vars
}