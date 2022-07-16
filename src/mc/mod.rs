use std::collections::HashMap;

use crate::cfir::handles::{Symbol, SymbolRef};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub enum Input {
  Reg(Reg),
  Label(Label),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub enum Reg {
  VirtualReg(SymbolRef),
  PhysicalReg(usize),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub enum Label {
  BasicBlockLabel(Symbol),
  FunctionLabel(Symbol),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct MachineBasicBlock {
  pub label:  Option<Symbol>,
  pub instrs: Vec<Instr>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MachineFunction {
  pub name: Symbol,
  pub bbs: Vec<MachineBasicBlock>,
  pub symble_table: HashMap<Symbol, usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct Instr {
  // pub prefix: Vec<Symbol>,
  pub op:     Symbol,
  pub input:  Vec<Input>,
  pub output: Option<Reg>,
}

impl Instr {
  fn get_output(&self) -> Option<Reg> {
    self.output.clone()
  }
}
