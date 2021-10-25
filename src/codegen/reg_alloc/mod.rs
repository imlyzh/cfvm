use crate::cfir::graphir::{basicblock::BasicBlockDef, instruction::Instruction};

pub mod simple_alloc;
pub mod linear_scan_alloc;



#[derive(Debug, Clone, Copy)]
pub struct RegConfig {
    pub reg_count: usize,
    pub exreg_count: usize,
}
