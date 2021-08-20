use std::sync::Arc;

use sexpr_ir::gast::Handle;

use super::basicblock::BasicBlockDef;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum BranchOp {
    IfNil,
    IfNonNil,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Op {
    // Vec<Arc<Instruction>>
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Instruction {
    Store(Arc<Instruction>, Arc<Instruction>),
    DataOperator(Handle<String>, Op),
    Branch(BranchOp, Arc<Instruction>, Arc<BasicBlockDef>),
    Call(Handle<String>, Arc<Instruction>),
    CallRef(Arc<Instruction>, Arc<Instruction>),
    Ret(Option<Arc<Instruction>>),
}