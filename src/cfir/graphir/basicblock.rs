use std::collections::HashMap;

use super::{MutHandle, handles::{LabelSymbol, LocalSymbol}, instruction::*};

#[derive(Debug, Clone)]
pub struct BasicBlockDef {
    pub label: Option<LabelSymbol>,
    // pub prev_block: MutHandle<Vec<MutHandle<BasicBlockDef>>>,
    pub variable_defs: MutHandle<HashMap<LocalSymbol, MutHandle<BindOperator>>>,
    pub instructions: MutHandle<Vec<MutHandle<Instruction>>>,
    pub terminator: MutHandle<Terminator>,
}
