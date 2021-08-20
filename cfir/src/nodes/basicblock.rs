use std::sync::Arc;

use sexpr_ir::gast::Handle;
use super::instruction::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockMetadata {
    pub label: Handle<String>,
    pub prev_block: Arc<BasicBlockDef>,
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BasicBlockDef {
    pub metadata: Option<BlockMetadata>,
    pub instructions: Vec<Arc<Instruction>>,
}
