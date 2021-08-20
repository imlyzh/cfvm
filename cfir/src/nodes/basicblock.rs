use std::sync::Arc;

use sexpr_ir::gast::Handle;
use super::instruction::*;


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BasicBlockDef {
    pub label: Option<Handle<String>>,
    pub dependencies: Vec<Arc<BasicBlockDef>>,
    pub instructions: Vec<Arc<Instruction>>,
}
