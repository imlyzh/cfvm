use super::*;
use super::instruction::*;


pub type FunctionPass = fn(&FunctionDef, env: &Module);

pub type BasicBlockPass = fn(&BasicBlockDef, env: &FunctionDef);

pub type InstructionPass = fn(&Instruction, env: &BasicBlockDef);

pub trait Pass<Env> {
    fn run(&self, env: &Env);
}
