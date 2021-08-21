
use super::{MutHandle, handles::LabelSymbol, instruction::*};


#[derive(Debug, Clone)]
pub struct BasicBlockDef {
    pub label: Option<LabelSymbol>,
    pub prev_block: MutHandle<Vec<MutHandle<BasicBlockDef>>>,
    pub instructions: MutHandle<Vec<MutHandle<Instruction>>>,
    pub terminator: MutHandle<Option<Terminator>>,
}
