use std::ops::Range;

use crate::{analysis::find_lifetime::FindVarLifetime, cfir::graphir::{FunctionDef, basicblock::BasicBlockDef, handles::{LazyLoadSymbol, LocalSymbol, SymbolHandle}, instruction::{BindOperator, Instruction, Store}}};


#[derive(Debug, Clone)]
pub struct InstStream(pub Vec<Instruction>);

pub trait Streamify {
    fn streamify(self) -> InstStream;
}

impl Streamify for BasicBlockDef {
    fn streamify(self) -> InstStream {
        InstStream(self.instructions.read().unwrap().iter().map(|x| x.read().unwrap().clone()).collect())
    }
}


#[derive(Debug, Clone, Copy)]
pub struct RegConfig {
    pub reg_count: usize,
    pub exreg_count: usize,
}

pub trait LinealScanAlloc {
    fn linear_scan_alloc(self, rcfg: RegConfig, bs: &mut BasicBlockDef, fd: &mut FunctionDef) -> InstStream;
}


impl LinealScanAlloc for InstStream {
    fn linear_scan_alloc(self, rcfg: RegConfig, bs: &mut BasicBlockDef, fd: &mut FunctionDef) -> InstStream {
        let InstStream(insts) = self;
        let mut reg_set = vec![false; rcfg.reg_count*3];
        let mut exreg_set = vec![false; rcfg.exreg_count*3];
        let mut reg_lifetime: Vec<(usize, usize)> = vec![];
        let mut var_lifetime: Vec<(LocalSymbol, usize)> = vec![];
        for (inst_num, i) in insts.iter().enumerate() {
            i.find_var_lifetime(true, inst_num, &mut reg_lifetime, &mut var_lifetime);
        }
        todo!()
    }
}