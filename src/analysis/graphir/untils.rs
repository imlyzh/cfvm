use crate::cfir::{
  graphir::{
    instruction::{BindOperator, Instruction},
    FunctionDef,
  },
  handles::LocalSymbol,
};

pub fn get_all_veriable(fun_def: &FunctionDef) -> Vec<LocalSymbol> {
  let mut vars = Vec::new();
  for bb in fun_def.bbs.iter() {
    for i in bb.instructions.iter() {
      if let Instruction::BindOperator(BindOperator(var, _)) = i {
        vars.push(var.clone());
      }
    }
  }
  vars
}
